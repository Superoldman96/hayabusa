extern crate serde_derive;
extern crate yaml_rust2;

use crate::detections::configs::{self, Action, CURRENT_EXE_PATH, StoredStatic};
use crate::detections::message::AlertMessage;
use crate::detections::message::ERROR_LOG_STACK;
use crate::detections::utils;
use crate::filter::RuleExclude;
use crate::level::LEVEL;
use crate::yaml_expand::{process_yaml, read_expand_files};
use compact_str::CompactString;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::ffi::OsStr;
use std::fs;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use yaml_rust2::{Yaml, YamlLoader};

lazy_static! {
    static ref DATE_REGEX: Regex = Regex::new(r"^\d{4}[-/]\d{1,2}[-/]\d{1,2}$").unwrap();
}

pub struct ParseYaml {
    pub files: Vec<(String, Yaml)>,
    pub rulecounter: HashMap<CompactString, u128>,
    pub rule_load_cnt: HashMap<CompactString, u128>,
    pub rule_status_cnt: HashMap<CompactString, u128>,
    pub rule_cor_cnt: HashMap<CompactString, u128>,
    pub rule_cor_ref_cnt: HashMap<CompactString, u128>,
    pub rule_expand_cnt: u128,
    pub rule_expand_enabled_cnt: u128,
    pub errorrule_count: u128,
    pub exclude_status: HashSet<String>,
    pub loaded_rule_ids: HashSet<CompactString>,
}

impl ParseYaml {
    pub fn new(stored_static: &StoredStatic) -> ParseYaml {
        let exclude_status_vec = if let Some(output_option) = stored_static.output_option.as_ref() {
            &output_option.exclude_status
        } else {
            &None
        };
        ParseYaml {
            files: Vec::new(),
            rulecounter: HashMap::new(),
            rule_load_cnt: HashMap::from([("excluded".into(), 0_u128), ("noisy".into(), 0_u128)]),
            rule_status_cnt: HashMap::from([
                ("deprecated".into(), 0_u128),
                ("unsupported".into(), 0_u128),
            ]),
            rule_cor_cnt: Default::default(),
            rule_cor_ref_cnt: Default::default(),
            rule_expand_cnt: Default::default(),
            rule_expand_enabled_cnt: Default::default(),
            errorrule_count: 0,
            exclude_status: configs::convert_option_vecs_to_hs(exclude_status_vec.as_ref()),
            loaded_rule_ids: HashSet::new(),
        }
    }

    pub fn read_file(path: &PathBuf) -> Result<String, String> {
        let mut file_content = String::new();

        let mut fr = fs::File::open(path)
            .map(BufReader::new)
            .map_err(|e| e.to_string())?;

        fr.read_to_string(&mut file_content)
            .map_err(|e| e.to_string())?;

        Ok(file_content)
    }

    fn read_encoded_file(path: &PathBuf) -> Result<String, String> {
        let mut fr = fs::File::open(path)
            .map(BufReader::new)
            .map_err(|e| e.to_string())?;
        let mut encrypted_content = Vec::new();
        let _ = fr.read_to_end(&mut encrypted_content);
        let decode_content = encrypted_content.iter().map(|&b| b ^ 0xAA).collect(); // key: 0xAA
        let decode_string = String::from_utf8(decode_content).expect("Invalid UTF-8 sequence");
        Ok(decode_string)
    }

    fn update_correlation_counts(&mut self, yaml_docs: &Vec<Yaml>) {
        for doc in yaml_docs {
            if let Some(correlation) = doc["correlation"].as_hash() {
                let entry = self
                    .rule_cor_cnt
                    .entry(CompactString::from("correlation"))
                    .or_insert(0);
                *entry += 1;
                if let Some(rules) = correlation.get(&Yaml::String("rules".to_string())) {
                    if let Some(rules_list) = rules.as_vec() {
                        for rule in rules_list {
                            if let Some(rule_str) = rule.as_str() {
                                // Update rules count, storing each unique rule
                                let rule_entry = self
                                    .rule_cor_ref_cnt
                                    .entry(CompactString::from(rule_str))
                                    .or_insert(0);
                                *rule_entry += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn read_dir<P: AsRef<Path>>(
        &mut self,
        path: P,
        min_level: &str,
        target_level: &str,
        exclude_ids: &RuleExclude,
        stored_static: &StoredStatic,
    ) -> io::Result<String> {
        let metadata = fs::metadata(path.as_ref());
        let is_contained_include_status_all_allowed = stored_static.include_status.contains("*");
        if metadata.is_err() {
            let err_contents = if let Err(e) = metadata {
                e.to_string()
            } else {
                String::default()
            };
            let mut errmsg = format!(
                "fail to read metadata of file: {} {}",
                path.as_ref().to_path_buf().display(),
                err_contents
            );
            if err_contents.ends_with("123)") {
                errmsg = format!(
                    "{errmsg}. You may not be able to load evtx files when there are spaces in the directory path. Please enclose the path with double quotes and remove any trailing slash at the end of the path."
                );
            }
            if stored_static.verbose_flag {
                AlertMessage::alert(&errmsg)?;
            }
            if !stored_static.quiet_errors_flag {
                ERROR_LOG_STACK
                    .lock()
                    .unwrap()
                    .push(format!("[ERROR] {errmsg}"));
            }
            return io::Result::Ok(String::default());
        }
        let expand_map = read_expand_files(CURRENT_EXE_PATH.join("config/expand"));
        let mut yaml_docs = vec![];
        if metadata.unwrap().file_type().is_file() {
            // 拡張子がymlでないファイルは無視
            if path
                .as_ref()
                .to_path_buf()
                .extension()
                .unwrap_or_else(|| OsStr::new(""))
                != "yml"
            {
                return io::Result::Ok(String::default());
            }
            // 個別のファイルの読み込みは即終了としない。
            let mut is_encoded = false;
            let read_content = if path
                .as_ref()
                .to_path_buf()
                .file_name()
                .unwrap_or_else(|| OsStr::new(""))
                == "encoded_rules.yml"
            {
                is_encoded = true;
                Self::read_encoded_file(&path.as_ref().to_path_buf())
            } else {
                Self::read_file(&path.as_ref().to_path_buf())
            };
            if read_content.is_err() {
                let errmsg = format!(
                    "fail to read file: {}\n{} ",
                    path.as_ref().to_path_buf().display(),
                    read_content.unwrap_err()
                );
                if stored_static.verbose_flag {
                    AlertMessage::warn(&errmsg)?;
                }
                if !stored_static.quiet_errors_flag {
                    ERROR_LOG_STACK
                        .lock()
                        .unwrap()
                        .push(format!("[WARN] {errmsg}"));
                }
                self.errorrule_count += 1;
                return io::Result::Ok(String::default());
            }

            // ここも個別のファイルの読み込みは即終了としない。
            match YamlLoader::load_from_str(&read_content.unwrap()) {
                Ok(contents) => {
                    Self::update_correlation_counts(self, &contents);
                    yaml_docs.extend(contents.into_iter().map(|yaml_content| {
                        let filepath = if is_encoded {
                            yaml_content["rulefile"]
                                .as_str()
                                .unwrap_or_default()
                                .to_string()
                        } else {
                            format!("{}", path.as_ref().to_path_buf().display())
                        };
                        (filepath, yaml_content)
                    }));
                }
                Err(error) => {
                    let errmsg = format!(
                        "Failed to parse yml: {}\n{} ",
                        path.as_ref().to_path_buf().display(),
                        error
                    );
                    if stored_static.verbose_flag {
                        AlertMessage::warn(&errmsg)?;
                    }
                    if !stored_static.quiet_errors_flag {
                        ERROR_LOG_STACK
                            .lock()
                            .unwrap()
                            .push(format!("[WARN] {errmsg}"));
                    }
                    self.errorrule_count += 1;
                }
            }
        } else {
            let mut entries = fs::read_dir(path)?;
            yaml_docs = entries.try_fold(vec![], |mut ret, entry| {
                let entry = entry?;
                // フォルダは再帰的に呼び出す。
                if entry.file_type()?.is_dir() {
                    self.read_dir(
                        entry.path(),
                        min_level,
                        target_level,
                        exclude_ids,
                        stored_static,
                    )?;
                    return io::Result::Ok(ret);
                }
                // ファイル以外は無視
                if !entry.file_type()?.is_file() {
                    return io::Result::Ok(ret);
                }

                // 拡張子がymlでないファイルは無視
                let path = entry.path();
                if path.extension().unwrap_or_else(|| OsStr::new("")) != "yml" {
                    return io::Result::Ok(ret);
                }

                let path_str = path.to_str().unwrap();
                // ignore if yml file in .git folder.
                if utils::contains_str(path_str, "/.git/")
                    || utils::contains_str(path_str, "\\.git\\")
                {
                    return io::Result::Ok(ret);
                }

                // ignore if tool test yml file in hayabusa-rules.
                if utils::contains_str(path_str, "rules/tools/sigmac/test_files")
                    || utils::contains_str(path_str, "rules\\tools\\sigmac\\test_files")
                {
                    return io::Result::Ok(ret);
                }

                // 個別のファイルの読み込みは即終了としない。
                let read_content = Self::read_file(&path);
                if read_content.is_err() {
                    let errmsg = format!(
                        "fail to read file: {}\n{} ",
                        entry.path().display(),
                        read_content.unwrap_err()
                    );
                    if stored_static.verbose_flag {
                        AlertMessage::warn(&errmsg)?;
                    }
                    if !stored_static.quiet_errors_flag {
                        ERROR_LOG_STACK
                            .lock()
                            .unwrap()
                            .push(format!("[WARN] {errmsg}"));
                    }
                    self.errorrule_count += 1;
                    return io::Result::Ok(ret);
                }

                // ここも個別のファイルの読み込みは即終了としない。
                match YamlLoader::load_from_str(&read_content.unwrap()) {
                    Ok(contents) => {
                        Self::update_correlation_counts(self, &contents);
                        let pair = contents.into_iter().map(|yaml_content| {
                            let filepath = format!("{}", entry.path().display());
                            (filepath, yaml_content)
                        });
                        ret.extend(pair);
                        io::Result::Ok(ret)
                    }
                    Err(error) => {
                        let errmsg = format!(
                            "Failed to parse yml: {}\n{} ",
                            entry.path().display(),
                            error
                        );
                        if stored_static.verbose_flag {
                            AlertMessage::warn(&errmsg)?;
                        }
                        if !stored_static.quiet_errors_flag {
                            ERROR_LOG_STACK
                                .lock()
                                .unwrap()
                                .push(format!("[WARN] {errmsg}"));
                        }
                        self.errorrule_count += 1;
                        io::Result::Ok(ret)
                    }
                }
            })?;
        }
        let exist_output_opt = stored_static.output_option.is_some();
        let files = yaml_docs.into_iter().filter_map(|(filepath, yaml_doc)| {
            let mut expand_found = false;
            let mut expand_enabled_found = false;
            let place_holder_map = expand_map.as_ref().unwrap();
            let yaml_doc = process_yaml(
                &yaml_doc,
                place_holder_map,
                &mut expand_found,
                &mut expand_enabled_found,
            );
            if expand_found {
                self.rule_expand_cnt += 1;
                if expand_enabled_found {
                    self.rule_expand_enabled_cnt += 1;
                } else {
                    return Option::None;
                }
            };
            //除外されたルールは無視する
            let rule_id = &yaml_doc["id"].as_str();
            if rule_id.is_some() {
                if let Some(v) = exclude_ids
                    .no_use_rule
                    .get(&rule_id.unwrap_or(&String::default()).to_string())
                {
                    let entry_key = if utils::contains_str(v, "exclude_rule") {
                        "excluded"
                    } else {
                        "noisy"
                    };
                    // テスト用のルール(ID:000...0)の場合はexcluded ruleのカウントから除外するようにする
                    if v != "00000000-0000-0000-0000-000000000000" {
                        let entry = self.rule_load_cnt.entry(entry_key.into()).or_insert(0);
                        *entry += 1;
                    }
                    let enable_noisy_rules = if let Some(o) = stored_static.output_option.as_ref() {
                        o.enable_noisy_rules
                    } else {
                        false
                    };

                    if entry_key == "excluded" || (entry_key == "noisy" && !enable_noisy_rules) {
                        return Option::None;
                    }
                }
                if let Some(id) = rule_id {
                    if !stored_static.target_ruleids.is_target(id, true) {
                        let entry = self.rule_load_cnt.entry("excluded".into()).or_insert(0);
                        *entry += 1;
                        return Option::None;
                    }
                }
            }

            let mut up_rule_status_cnt = |status: &str| {
                let status_cnt = self.rule_status_cnt.entry(status.into()).or_insert(0);
                *status_cnt += 1;
            };

            let mut up_rule_load_cnt = |status: &str| {
                let entry = self.rule_load_cnt.entry(status.into()).or_insert(0);
                *entry += 1;
            };
            match check_hayabusa_rule_fmt(&yaml_doc) {
                Ok(_) => {}
                Err(errmsg) => {
                    if stored_static.verbose_flag {
                        AlertMessage::warn(&errmsg).ok();
                    }
                    if !stored_static.quiet_errors_flag {
                        ERROR_LOG_STACK
                            .lock()
                            .unwrap()
                            .push(format!("[WARN] Invalid rule. {errmsg} ({filepath})"));
                    }
                    self.errorrule_count += 1;
                    return Option::None;
                }
            }

            // 指定されたレベルより低いルールは無視する
            let doc_level = &yaml_doc["level"]
                .as_str()
                .unwrap_or("informational")
                .to_uppercase();
            let doc_level_num = LEVEL::from(doc_level).index();
            let args_level_num = LEVEL::from(min_level).index();
            let target_level_num = LEVEL::from(target_level).index();
            if doc_level_num < args_level_num
                || (target_level_num != 0 && doc_level_num != target_level_num)
            {
                up_rule_load_cnt("excluded");
                return Option::None;
            }
            let status = yaml_doc["status"].as_str();
            if let Some(s) = yaml_doc["status"].as_str() {
                // excluded status optionで指定されたstatusとinclude_status optionで指定されたstatus以外のルールは除外する
                if self.exclude_status.contains(&s.to_string())
                    || !(is_contained_include_status_all_allowed
                        || stored_static.include_status.contains(s))
                {
                    up_rule_load_cnt("excluded");
                    return Option::None;
                }

                if exist_output_opt
                    && ((s == "deprecated"
                        && !stored_static
                            .output_option
                            .as_ref()
                            .unwrap()
                            .enable_deprecated_rules)
                        || (s == "unsupported"
                            && !stored_static
                                .output_option
                                .as_ref()
                                .unwrap()
                                .enable_unsupported_rules))
                {
                    // deprecated or unsupported statusで対応するenable-xxx-rules optionが指定されていない場合はステータスのカウントのみ行ったうえで除外する
                    up_rule_status_cnt(s);
                    return Option::None;
                }
            } else if !is_contained_include_status_all_allowed {
                let need_rules = matches!(
                    stored_static.config.action.as_ref().unwrap(),
                    Action::CsvTimeline(_) | Action::JsonTimeline(_) | Action::PivotKeywordsList(_)
                );
                if need_rules {
                    up_rule_load_cnt("excluded");
                    return Option::None;
                }
            }
            if exist_output_opt {
                let category_in_rule = yaml_doc["logsource"]["category"]
                    .as_str()
                    .unwrap_or_default();
                let mut include_category = &Vec::default();
                let mut exclude_category = &Vec::default();

                if let Some(tmp) = &stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .include_category
                {
                    include_category = tmp;
                }

                if let Some(tmp) = &stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .exclude_category
                {
                    exclude_category = tmp;
                }

                if !include_category.is_empty()
                    && !include_category.contains(&category_in_rule.to_string())
                {
                    up_rule_load_cnt("excluded");
                    return Option::None;
                }
                if !exclude_category.is_empty()
                    && exclude_category.contains(&category_in_rule.to_string())
                {
                    up_rule_load_cnt("excluded");
                    return Option::None;
                }
            }

            // tags optionで指定されたtagsを持たないルールは除外する
            if exist_output_opt
                && stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .include_tag
                    .is_some()
            {
                let target_tags = stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .include_tag
                    .as_ref()
                    .unwrap();
                let rule_tags_vec = yaml_doc["tags"].as_vec();
                if let Some(rule_tags) = rule_tags_vec {
                    let is_match = rule_tags.iter().any(|tag| {
                        target_tags.contains(&tag.as_str().unwrap_or_default().to_string())
                    });
                    if !is_match {
                        up_rule_load_cnt("excluded");
                        return Option::None;
                    }
                } else {
                    up_rule_load_cnt("excluded");
                    return Option::None;
                }
            }

            // exclude-tag optionで指定されたtagを持つルールは除外する
            if stored_static.output_option.is_some()
                && stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .exclude_tag
                    .is_some()
            {
                let exclude_target_tags = stored_static
                    .output_option
                    .as_ref()
                    .unwrap()
                    .exclude_tag
                    .as_ref()
                    .unwrap();
                let rule_tags_vec = yaml_doc["tags"].as_vec();
                if let Some(rule_tags) = rule_tags_vec {
                    let is_match = rule_tags.iter().any(|tag| {
                        exclude_target_tags.contains(&tag.as_str().unwrap_or_default().to_string())
                    });
                    if is_match {
                        up_rule_load_cnt("excluded");
                        return Option::None;
                    }
                }
            }

            self.rulecounter.insert(
                yaml_doc["ruletype"].as_str().unwrap_or("Other").into(),
                self.rulecounter
                    .get(yaml_doc["ruletype"].as_str().unwrap_or("Other"))
                    .unwrap_or(&0)
                    + 1,
            );

            up_rule_status_cnt(status.unwrap_or("undefined"));

            if stored_static.verbose_flag {
                println!("Loaded rule: {filepath}");
            }

            Option::Some((filepath, yaml_doc))
        });
        self.files.extend(files);
        io::Result::Ok(String::default())
    }
}

/// wizardへのルール数表示のためのstatus/level/tagsごとに階層化させてカウントする
pub fn count_rules<P: AsRef<Path>>(
    path: P,
    exclude_ids: &RuleExclude,
    stored_static: &StoredStatic,
    result_container: &mut HashMap<
        CompactString,
        HashMap<CompactString, HashMap<CompactString, i128>>,
    >,
) -> HashMap<CompactString, HashMap<CompactString, HashMap<CompactString, i128>>> {
    let metadata = fs::metadata(path.as_ref());
    if metadata.is_err() {
        return HashMap::default();
    }
    let mut yaml_docs = vec![];
    if metadata.unwrap().file_type().is_file() {
        // 拡張子がymlでないファイルは無視
        if path
            .as_ref()
            .to_path_buf()
            .extension()
            .unwrap_or_else(|| OsStr::new(""))
            != "yml"
        {
            return HashMap::default();
        }

        // 個別のファイルの読み込みは即終了としない。
        let mut is_encoded = false;
        let read_content = if path
            .as_ref()
            .to_path_buf()
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            == "encoded_rules.yml"
        {
            is_encoded = true;
            ParseYaml::read_encoded_file(&path.as_ref().to_path_buf())
        } else {
            ParseYaml::read_file(&path.as_ref().to_path_buf())
        };

        if read_content.is_err() {
            return HashMap::default();
        }

        // ここも個別のファイルの読み込みは即終了としない。
        let yaml_contents = YamlLoader::load_from_str(&read_content.unwrap());
        if yaml_contents.is_err() {
            return HashMap::default();
        }

        yaml_docs.extend(yaml_contents.unwrap().into_iter().map(|yaml_content| {
            let filepath = if is_encoded {
                yaml_content["rulefile"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string()
            } else {
                format!("{}", path.as_ref().to_path_buf().display())
            };
            (filepath, yaml_content)
        }));
    } else {
        let entries = fs::read_dir(path);
        if entries.is_err() {
            return HashMap::default();
        }
        yaml_docs = entries
            .unwrap()
            .try_fold(vec![], |mut ret, entry| {
                let entry = entry?;
                // フォルダは再帰的に呼び出す。
                if entry.file_type()?.is_dir() {
                    count_rules(entry.path(), exclude_ids, stored_static, result_container);
                    return io::Result::Ok(ret);
                }
                // ファイル以外は無視
                if !entry.file_type()?.is_file() {
                    return io::Result::Ok(ret);
                }

                // 拡張子がymlでないファイルは無視
                let path = entry.path();
                if path.extension().unwrap_or_else(|| OsStr::new("")) != "yml" {
                    return io::Result::Ok(ret);
                }

                let path_str = path.to_str().unwrap();
                // ignore if yml file in .git folder.
                if utils::contains_str(path_str, "/.git/")
                    || utils::contains_str(path_str, "\\.git\\")
                {
                    return io::Result::Ok(ret);
                }

                // ignore if tool test yml file in hayabusa-rules.
                if utils::contains_str(path_str, "rules/tools/sigmac/test_files")
                    || utils::contains_str(path_str, "rules\\tools\\sigmac\\test_files")
                {
                    return io::Result::Ok(ret);
                }

                // 個別のファイルの読み込みは即終了としない。
                let read_content = ParseYaml::read_file(&path);
                if read_content.is_err() {
                    return io::Result::Ok(ret);
                }

                // ここも個別のファイルの読み込みは即終了としない。
                let yaml_contents = YamlLoader::load_from_str(&read_content.unwrap());
                if yaml_contents.is_err() {
                    let errmsg = format!(
                        "Failed to parse yml: {}\n{} ",
                        entry.path().display(),
                        yaml_contents.unwrap_err()
                    );
                    if stored_static.verbose_flag {
                        AlertMessage::warn(&errmsg)?;
                    }
                    if !stored_static.quiet_errors_flag {
                        ERROR_LOG_STACK
                            .lock()
                            .unwrap()
                            .push(format!("[WARN] {errmsg}"));
                    }
                    return io::Result::Ok(ret);
                }

                let yaml_contents = yaml_contents.unwrap().into_iter().map(|yaml_content| {
                    let filepath = format!("{}", entry.path().display());
                    (filepath, yaml_content)
                });
                ret.extend(yaml_contents);
                io::Result::Ok(ret)
            })
            .unwrap_or_default();
    }
    yaml_docs.into_iter().for_each(|(_filepath, yaml_doc)| {
        //除外されたルールは無視する
        let empty = vec![];
        let rule_id = &yaml_doc["id"].as_str();
        let rule_tags_vec = yaml_doc["tags"].as_vec().unwrap_or(&empty);
        let included_target_tag_vec = {
            let target_wizard_tags = [
                "detection.emerging_threats",
                "detection.threat_hunting",
                "sysmon",
            ];
            rule_tags_vec
                .iter()
                .filter(|x| target_wizard_tags.contains(&x.as_str().unwrap_or_default()))
                .filter_map(|s| s.as_str())
                .collect_vec()
        };
        if rule_id.is_some() {
            if let Some(v) = exclude_ids
                .no_use_rule
                .get(&rule_id.unwrap_or(&String::default()).to_string())
            {
                let entry_key = if utils::contains_str(v, "exclude_rule") {
                    "excluded"
                } else {
                    "noisy"
                };
                // テスト用のルール(ID:000...0)の場合はexcluded ruleのカウントから除外するようにする
                if v != "00000000-0000-0000-0000-000000000000" {
                    let counter = result_container
                        .entry(entry_key.into())
                        .or_insert(HashMap::new());
                    *counter
                        .entry(
                            yaml_doc["level"]
                                .as_str()
                                .unwrap_or("informational")
                                .to_uppercase()
                                .into(),
                        )
                        .or_insert(HashMap::new())
                        .entry(
                            yaml_doc["status"]
                                .as_str()
                                .unwrap_or("undefined")
                                .to_lowercase()
                                .into(),
                        )
                        .or_insert(0) += 1;
                }
                return;
            }
        }

        if let Some(s) = yaml_doc["status"].as_str() {
            // wizard用の初期カウンティングではstatusとlevelの内容を確認したうえで以降の処理は行わないようにする
            let counter = result_container.entry(s.into()).or_insert(HashMap::new());
            if included_target_tag_vec.is_empty() {
                *counter
                    .entry(
                        yaml_doc["level"]
                            .as_str()
                            .unwrap_or("informational")
                            .to_uppercase()
                            .into(),
                    )
                    .or_insert(HashMap::new())
                    .entry("other".into())
                    .or_insert(0) += 1;
            } else {
                if included_target_tag_vec.len() > 1 {
                    *counter
                        .entry(
                            yaml_doc["level"]
                                .as_str()
                                .unwrap_or("informational")
                                .to_uppercase()
                                .into(),
                        )
                        .or_insert(HashMap::new())
                        .entry("duplicated".into())
                        .or_insert(0) -= (included_target_tag_vec.len() - 1) as i128;
                }
                for tag in included_target_tag_vec {
                    *counter
                        .entry(
                            yaml_doc["level"]
                                .as_str()
                                .unwrap_or("informational")
                                .to_uppercase()
                                .into(),
                        )
                        .or_insert(HashMap::new())
                        .entry(tag.into())
                        .or_insert(0) += 1;
                }
            }
        }
    });
    result_container.to_owned()
}

pub fn check_hayabusa_rule_fmt(yaml: &Yaml) -> Result<(), String> {
    let mut required_keys = vec![
        "author",
        "title",
        "logsource",
        "detection",
        "level",
        "status",
        "date",
        "id",
    ];

    if yaml["correlation"].is_hash() {
        required_keys.retain(|&key| key != "logsource" && key != "detection");
    }

    let mut errors = Vec::new();

    for &key in &required_keys {
        if !yaml[key].is_badvalue() {
            match key {
                "level" => {
                    let value = yaml[key].as_str().unwrap_or("");
                    if !["informational", "low", "medium", "high", "critical"].contains(&value) {
                        errors.push(format!("Invalid: {key}"));
                    }
                }
                "status" => {
                    let value = yaml[key].as_str().unwrap_or("");
                    if ![
                        "stable",
                        "test",
                        "experimental",
                        "deprecated",
                        "unsupported",
                    ]
                    .contains(&value)
                    {
                        errors.push(format!("Invalid: {key}"));
                    }
                }
                "date" => {
                    if !DATE_REGEX.is_match(yaml[key].as_str().unwrap_or("")) {
                        errors.push(format!("Invalid: {key}"));
                    }
                }
                _ => {}
            }
        } else {
            errors.push(format!("Missing: {key}"));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join(" ¦ "))
    }
}

#[cfg(test)]
mod tests {
    use crate::detections::configs::Action;
    use crate::detections::configs::Config;
    use crate::detections::configs::CsvOutputOption;
    use crate::detections::configs::DetectCommonOption;
    use crate::detections::configs::OutputOption;
    use crate::detections::configs::StoredStatic;
    use crate::filter;
    use crate::yaml;
    use crate::yaml::ParseYaml;
    use crate::yaml::RuleExclude;
    use compact_str::CompactString;
    use hashbrown::HashMap;
    use hashbrown::HashSet;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use yaml_rust2::YamlLoader;

    fn create_dummy_stored_static() -> StoredStatic {
        StoredStatic::create_static_data(Some(Config {
            action: Some(Action::CsvTimeline(CsvOutputOption {
                output_options: OutputOption {
                    min_level: "informational".to_string(),
                    include_status: Some(vec!["*".to_string()]),
                    detect_common_options: DetectCommonOption {
                        config: Path::new("./rules/config").to_path_buf(),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            })),
            debug: false,
        }))
    }

    #[test]
    fn test_read_file_yaml() {
        let exclude_ids = RuleExclude::new();
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        let _ = &yaml.read_dir(
            "test_files/rules/yaml/1.yml",
            &String::default(),
            "",
            &exclude_ids,
            &dummy_stored_static,
        );
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_read_dir_yaml() {
        let exclude_ids = RuleExclude {
            no_use_rule: HashMap::new(),
        };
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        let _ = &yaml.read_dir(
            "test_files/rules/yaml/",
            &String::default(),
            "",
            &exclude_ids,
            &dummy_stored_static,
        );
        assert_ne!(yaml.files.len(), 0);
    }

    #[test]
    fn test_read_yaml() {
        let path = Path::new("test_files/rules/yaml/1.yml");
        let ret = ParseYaml::read_file(&path.to_path_buf()).unwrap();
        let rule = YamlLoader::load_from_str(&ret).unwrap();
        for i in rule {
            if i["title"].as_str().unwrap() == "Sysmon Check command lines" {
                assert_eq!(
                    "*",
                    i["detection"]["selection"]["CommandLine"].as_str().unwrap()
                );
                assert_eq!(1, i["detection"]["selection"]["EventID"].as_i64().unwrap());
            }
        }
    }

    #[test]
    fn test_failed_read_yaml() {
        let path = Path::new("test_files/rules/yaml/error.yml");
        let ret = ParseYaml::read_file(&(path.to_path_buf())).unwrap();
        let rule = YamlLoader::load_from_str(&ret);
        assert!(rule.is_err());
    }

    #[test]
    /// no specifed "level" arguments value is adapted default level(informational)
    fn test_default_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 5);
    }

    #[test]
    fn test_info_level_read_yaml() {
        let dummy_stored_static = create_dummy_stored_static();
        let path = Path::new("test_files/rules/level_yaml");
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "INFORMATIONAL",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 5);
    }
    #[test]
    fn test_low_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "LOW",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 4);
    }
    #[test]
    fn test_medium_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "MEDIUM",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 3);
    }
    #[test]
    fn test_high_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "HIGH",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 2);
    }
    #[test]
    fn test_critical_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "CRITICAL",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }
    #[test]
    fn test_all_exclude_rules_file() {
        let path = Path::new("test_files/rules/yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.rule_load_cnt.get("excluded").unwrap().to_owned(), 5);
    }
    #[test]
    fn test_all_noisy_rules_file() {
        let path = Path::new("test_files/rules/yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.rule_load_cnt.get("noisy").unwrap().to_owned(), 5);
    }
    #[test]
    fn test_none_exclude_rules_file() {
        let path = Path::new("test_files/rules/yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static.include_status = HashSet::from_iter(vec![CompactString::from("*")]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        let exclude_ids = RuleExclude::new();
        yaml.read_dir(path, "", "", &exclude_ids, &dummy_stored_static)
            .unwrap();
        assert_eq!(yaml.rule_load_cnt.get("excluded").unwrap().to_owned(), 0);
    }
    #[test]
    fn test_exclude_deprecated_rules_file() {
        let path = Path::new("test_files/rules/deprecated");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static.include_status = HashSet::from_iter(vec![CompactString::from("*")]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        let exclude_ids = RuleExclude::new();
        yaml.read_dir(path, "", "", &exclude_ids, &dummy_stored_static)
            .unwrap();
        assert_eq!(
            yaml.rule_status_cnt.get("deprecated").unwrap().to_owned(),
            1
        );
    }

    #[test]
    fn test_exclude_unsupported_rules_file() {
        let path = Path::new("test_files/rules/unsupported");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static.include_status = HashSet::from_iter(vec![CompactString::from("*")]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        let exclude_ids = RuleExclude::new();
        yaml.read_dir(path, "", "", &exclude_ids, &dummy_stored_static)
            .unwrap();
        assert_eq!(
            yaml.rule_status_cnt.get("unsupported").unwrap().to_owned(),
            1
        );
    }

    #[test]
    fn test_info_exact_level_read_yaml() {
        let dummy_stored_static = create_dummy_stored_static();
        let path = Path::new("test_files/rules/level_yaml");
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "INFORMATIONAL",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_low_exact_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "LOW",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_medium_exact_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "MEDIUM",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_high_exact_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "HIGH",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_critical_exact_level_read_yaml() {
        let path = Path::new("test_files/rules/level_yaml");
        let dummy_stored_static = create_dummy_stored_static();
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "CRITICAL",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_specified_tags_option() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .include_tag = Some(vec!["tag1".to_string(), "tag2".to_string()]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 3);
    }

    #[test]
    fn test_include_category_option_1opt() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .include_category = Some(vec!["test_category1".to_string()]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 1);
    }

    #[test]
    fn test_include_category_option_multi_opt() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .include_category = Some(vec![
            "test_category1".to_string(),
            "test_category2".to_string(),
        ]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 2);
    }

    #[test]
    fn test_include_category_option_not_found() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .include_category = Some(vec!["not found".to_string()]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 0);
    }

    #[test]
    fn test_exclude_category_option_1opt() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .exclude_category = Some(vec!["test_category1".to_string()]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 4);
    }

    #[test]
    fn test_exclude_category_option_multi_opt() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .exclude_category = Some(vec![
            "test_category1".to_string(),
            "test_category2".to_string(),
        ]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 3);
    }

    #[test]
    fn test_exclude_category_option_notfound() {
        let path = Path::new("test_files/rules/level_yaml");
        let mut dummy_stored_static = create_dummy_stored_static();
        dummy_stored_static
            .output_option
            .as_mut()
            .unwrap()
            .exclude_category = Some(vec!["not found".to_string()]);
        let mut yaml = yaml::ParseYaml::new(&dummy_stored_static);
        yaml.read_dir(
            path,
            "",
            "",
            &filter::exclude_ids(&dummy_stored_static),
            &dummy_stored_static,
        )
        .unwrap();
        assert_eq!(yaml.files.len(), 5);
    }

    #[test]
    fn test_read_encoded_file() {
        let test_path = PathBuf::from("test_encoded_file");
        let encoded_content: Vec<u8> = vec![
            b'H' ^ 0xAA,
            b'e' ^ 0xAA,
            b'l' ^ 0xAA,
            b'l' ^ 0xAA,
            b'o' ^ 0xAA,
        ];
        let mut file = File::create(&test_path).expect("Failed to create test file");
        file.write_all(&encoded_content)
            .expect("Failed to write to test file");
        let result = ParseYaml::read_encoded_file(&test_path);
        std::fs::remove_file(&test_path).expect("Failed to delete test file");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello");
    }

    #[test]
    fn test_hayabusa_rule_fmt() {
        let dir = Path::new("test_files/rules/level_yaml/");
        let entries = std::fs::read_dir(dir).unwrap();
        for entry in entries {
            let entry = entry.unwrap();
            let path = entry.path();
            let read_content = ParseYaml::read_file(&path).ok();
            let yaml_contents = YamlLoader::load_from_str(&read_content.unwrap()).ok();
            for yaml_content in yaml_contents.unwrap() {
                let result = yaml::check_hayabusa_rule_fmt(&yaml_content);
                assert!(result.is_ok());
            }
        }
    }
}
