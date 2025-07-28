<div align="center">
 <p>
    <img alt="Hayabusa Logo" src="logo.png" width="60%">
 </p>
  [<a href="README.md">English</a>] | [<b>日本語</b>]
</div>

---

<p align="center">
    <a href="https://github.com/Yamato-Security/hayabusa/releases"><img src="https://img.shields.io/github/v/release/Yamato-Security/hayabusa?color=blue&label=Stable%20Version&style=flat"/></a>
    <a href="https://github.com/Yamato-Security/hayabusa/releases"><img src="https://img.shields.io/github/downloads/Yamato-Security/hayabusa/total?style=flat&label=GitHub%F0%9F%A6%85Downloads&color=blue"/></a>
    <a href="https://github.com/Yamato-Security/hayabusa/stargazers"><img src="https://img.shields.io/github/stars/Yamato-Security/hayabusa?style=flat&label=GitHub%F0%9F%A6%85Stars"/></a>
    <a href="https://github.com/Yamato-Security/hayabusa/graphs/contributors"><img src="https://img.shields.io/github/contributors/Yamato-Security/hayabusa?label=Contributors&color=blue&style=flat"/></a>
    <a href="https://www.blackhat.com/asia-22/arsenal/schedule/#hayabusa-26211"><img src="https://raw.githubusercontent.com/toolswatch/badges/master/arsenal/asia/2022.svg"></a>
    <a href="https://codeblue.jp/2022/en/talks/?content=talks_24"><img src="https://img.shields.io/badge/CODE%20BLUE%20Bluebox-2022-blue"></a>
    <a href="https://www.seccon.jp/2022/seccon_workshop/windows.html"><img src="https://img.shields.io/badge/SECCON-2023-blue"></a>
    <a href="https://www.security-camp.or.jp/minicamp/tokyo2023.html"><img src="https://img.shields.io/badge/Security%20MiniCamp%20Tokyo-2023-blue"></a>
    <a href="https://www.sans.org/cyber-security-training-events/digital-forensics-summit-2023/"><img src="https://img.shields.io/badge/SANS%20DFIR%20Summit-2023-blue"></a>
    <a href="https://bsides.tokyo/2024/"><img src="https://img.shields.io/badge/BSides%20Tokyo-2024-blue"></a>
    <a href="https://www.hacker.or.jp/hack-fes-2024/"><img src="https://img.shields.io/badge/Hack%20Fes.-2024-blue"></a>
    <a href="https://hitcon.org/2024/CMT/"><img src="https://img.shields.io/badge/HITCON-2024-blue"></a>
    <a href="https://www.blackhat.com/sector/2024/briefings/schedule/index.html#performing-dfir-and-threat-hunting-with-yamato-security-oss-tools-and-community-driven-knowledge-41347"><img src="https://img.shields.io/badge/SecTor-2024-blue"></a>
    <a href="https://www.infosec-city.com/schedule/sin25-con"><img src="https://img.shields.io/badge/SINCON%20Kampung%20Workshop-2025-blue"></a>
    <a href="https://www.blackhat.com/us-25/arsenal/schedule/index.html#windows-fast-forensics-with-yamato-securitys-hayabusa-45629"><img src="https://img.shields.io/badge/Blackhat%20Arsenal%20USA-2025-blue"></a>
    <a href="https://gist.github.com/cheerfulstoic/d107229326a01ff0f333a1d3476e068d"><img src="https://img.shields.io/badge/Maintenance%20Level-Actively%20Developed-brightgreen.svg" /></a>
    <a href="https://github.com/Yamato-Security/hayabusa/commits/main/"><img src="https://img.shields.io/github/commit-activity/t/Yamato-Security/hayabusa/main" /></a>
    <a href="https://rust-reportcard.xuri.me/report/github.com/Yamato-Security/hayabusa"><img src="https://rust-reportcard.xuri.me/badge/github.com/Yamato-Security/hayabusa" /></a>
    <a href="https://codecov.io/gh/Yamato-Security/hayabusa" ><img src="https://codecov.io/gh/Yamato-Security/hayabusa/branch/main/graph/badge.svg?token=WFN5XO9W8C"/></a>
    <a href="https://twitter.com/SecurityYamato"><img src="https://img.shields.io/twitter/follow/SecurityYamato?style=social"/></a>
</p>

# Hayabusa について

Hayabusaは、日本の[Yamato Security](https://yamatosecurity.connpass.com/)グループによって作られた**Windowsイベントログのファストフォレンジックタイムライン作成**および**脅威ハンティングツール**です。 
Hayabusaは日本語で[「ハヤブサ」](https://ja.wikipedia.org/wiki/%E3%83%8F%E3%83%A4%E3%83%96%E3%82%B5)を意味し、ハヤブサが世界で最も速く、狩猟(hunting)に優れ、とても訓練しやすい動物であることから選ばれました。
[Rust](https://www.rust-lang.org/) で開発され、マルチスレッドに対応し、可能な限り高速に動作するよう配慮されています。
Hayabusaは[upstream Sigma](https://github.com/SigmaHQ/sigma) ルールの解析に対応しています。ただし、[hayabusa-rules repository](https://github.com/Yamato-Security/hayabusa-rules)で使用およびホストしているSigmaルールは、ルールの読み込みをより柔軟にし、誤検知を減らすために一部の変換が施されています。
詳細については、[sigma-to-hayabusa-converter repository](https://github.com/Yamato-Security/sigma-to-hayabusa-converter) リポジトリのREADMEファイルをご参照ください。
稼働中のシステムで実行してライブ調査することも、複数のシステムからログを収集してオフライン調査することも可能です。 また、 [Velociraptor](https://docs.velociraptor.app/)と[Hayabusa artifact](https://docs.velociraptor.app/exchange/artifacts/pages/windows.eventlogs.hayabusa/)を用いることで企業向けの広範囲なスレットハンティングとインシデントレスポンスにも活用できます。
出力は一つのCSVタイムラインにまとめられ、[LibreOffice](https://www.libreoffice.org/)、[Timeline Explorer](https://ericzimmerman.github.io/#!index.md)、[Elastic Stack](doc/ElasticStackImport/ElasticStackImport-Japanese.md)、[Timesketch](https://timesketch.org/)等で簡単に分析できるようになります。

# 関連プロジェクト

* [EnableWindowsLogSettings](https://github.com/Yamato-Security/EnableWindowsLogSettings) - Sigmaベースの脅威ハンティングと、Windowsイベントログのファストフォレンジックタイムライン生成ツール。
* [Hayabusa Encoded Rules](https://github.com/Yamato-Security/hayabusa-encoded-rules) - Hayabusa Rulesリポジトリと同じだが、ルールと設定ファイルは1つのファイルに保存され、アンチウイルスによる誤検知を防ぐためにXORされる。
* [Hayabusa Rules](https://github.com/Yamato-Security/hayabusa-rules/blob/main/README-Japanese.md) - Hayabusaのための検知ルール。
* [Hayabusa EVTX](https://github.com/Yamato-Security/hayabusa-evtx) - `evtxクレート`のよりメンテナンスされたフォーク。
* [Hayabusa Sample EVTXs](https://github.com/Yamato-Security/hayabusa-sample-evtx) - Hayabusa/Sigma検出ルールをテストするためのサンプルevtxファイル。
* [Presentations](https://github.com/Yamato-Security/Presentations) - ツールやリソースについて行った講演のプレゼンテーション。
* [Sigma to Hayabusa Converter](https://github.com/Yamato-Security/sigma-to-hayabusa-converter) - 上流のWindowsイベントログベースのSigmaルールを使いやすい形式にキュレーションする。
* [Takajo](https://github.com/Yamato-Security/takajo/blob/main/README-Japanese.md) - Hayabusa結果の解析ツール。
* [WELA (Windows Event Log Analyzer)](https://github.com/Yamato-Security/WELA/blob/main/README-Japanese.md) - PowerShellで書かれたWindowsイベントログの解析ツール。(非推奨となり、Takajoに置き換えられた)

# Hayabusaを利用したサードパーティプロジェクト

* [AllthingsTimesketch](https://github.com/blueteam0ps/AllthingsTimesketch) - PlasoとHayabusaの結果をTimesketchにインポートするNodeREDワークフロー
* [LimaCharlie](https://docs.limacharlie.io/docs/extensions-third-party-extensions-hayabusa) - ニーズに合わせたクラウドベースのセキュリティツールとインフラを提供
* [OpenRelik](https://openrelik.org/) - デジタル・フォレンジックの共同調査を効率化するために設計されたオープンソース（Apache-2.0）のプラットフォーム
* [Splunk4DFIR](https://github.com/mf1d3l/Splunk4DFIR) - Dockerでsplunkインスタンスを素早く立ち上げ、調査中に出力されるログやツールを閲覧するためのツール
* [Velociraptor](https://github.com/Velocidex/velociraptor) - Velociraptor Query Language (VQL)クエリを使用したホストベースの収集ツール

## 目次

- [Hayabusa について](#hayabusa-について)
- [関連プロジェクト](#関連プロジェクト)
- [Hayabusaを利用したサードパーティプロジェクト](#hayabusaを利用したサードパーティプロジェクト)
  - [目次](#目次)
  - [主な目的](#主な目的)
    - [スレット(脅威)ハンティングと企業向けの広範囲なDFIR](#スレット脅威ハンティングと企業向けの広範囲なdfir)
    - [フォレンジックタイムラインの高速生成](#フォレンジックタイムラインの高速生成)
- [スクリーンショット](#スクリーンショット)
  - [起動](#起動)
  - [DFIRタイムラインのターミナル出力](#dfirタイムラインのターミナル出力)
  - [キーワード検索結果](#キーワード検索結果)
  - [検知頻度タイムライン出力 (`-T`オプション)](#検知頻度タイムライン出力--tオプション)
  - [結果サマリ (Results Summary)](#結果サマリ-results-summary)
  - [HTMLの結果サマリ (`-H`オプション)](#htmlの結果サマリ--hオプション)
  - [LibreOfficeでのDFIRタイムライン解析 (`-M` マルチライン出力)](#libreofficeでのdfirタイムライン解析--m-マルチライン出力)
  - [Timeline ExplorerでのDFIRタイムライン解析](#timeline-explorerでのdfirタイムライン解析)
  - [Criticalアラートのフィルタリングとコンピュータごとのグルーピング](#criticalアラートのフィルタリングとコンピュータごとのグルーピング)
  - [Elastic Stackダッシュボードでの解析](#elastic-stackダッシュボードでの解析)
  - [Timesketchでの解析](#timesketchでの解析)
- [タイムライン結果のインポートと解析について](#タイムライン結果のインポートと解析について)
- [jqによるJSON形式の結果の解析](#jqによるjson形式の結果の解析)
- [特徴＆機能](#特徴機能)
- [ダウンロード](#ダウンロード)
  - [Windowsライブレスポンスパッケージ](#windowsライブレスポンスパッケージ)
- [Gitクローン](#gitクローン)
- [アドバンス: ソースコードからのコンパイル（任意）](#アドバンス-ソースコードからのコンパイル任意)
  - [Rustパッケージの更新](#rustパッケージの更新)
  - [32ビットWindowsバイナリのクロスコンパイル](#32ビットwindowsバイナリのクロスコンパイル)
  - [macOSでのコンパイルの注意点](#macosでのコンパイルの注意点)
  - [Linuxでのコンパイルの注意点](#linuxでのコンパイルの注意点)
  - [LinuxのMUSLバイナリのクロスコンパイル](#linuxのmuslバイナリのクロスコンパイル)
- [Hayabusaの実行](#hayabusaの実行)
  - [注意: アンチウィルス/EDRの誤検知と遅い初回実行](#注意-アンチウィルスedrの誤検知と遅い初回実行)
  - [Windows](#windows)
    - [パスにスペースが含まれるファイルまたはディレクトリをスキャンしようとするとエラーが発生した場合](#パスにスペースが含まれるファイルまたはディレクトリをスキャンしようとするとエラーが発生した場合)
    - [文字が正常に表示されない場合](#文字が正常に表示されない場合)
  - [Linux](#linux)
  - [macOS](#macos)
- [コマンド一覧](#コマンド一覧)
  - [分析コマンド:](#分析コマンド)
  - [Configコマンド:](#configコマンド)
  - [DFIRタイムライン作成のコマンド:](#dfirタイムライン作成のコマンド)
  - [汎用コマンド:](#汎用コマンド)
- [コマンド使用方法](#コマンド使用方法)
  - [分析コマンド](#分析コマンド-1)
    - [`computer-metrics`コマンド](#computer-metricsコマンド)
      - [`computer-metrics`コマンドの使用例](#computer-metricsコマンドの使用例)
      - [`computer-metrics`のスクリーンショット](#computer-metricsのスクリーンショット)
    - [`eid-metrics`コマンド](#eid-metricsコマンド)
      - [`eid-metrics`コマンドの使用例](#eid-metricsコマンドの使用例)
      - [`eid-metrics`コマンドの設定ファイル](#eid-metricsコマンドの設定ファイル)
      - [`eid-metrics`のスクリーンショット](#eid-metricsのスクリーンショット)
    - [`expand-list`コマンド](#expand-listコマンド)
      - [`expand-list`コマンドの使用例](#expand-listコマンドの使用例)
      - [`expand-list`結果](#expand-list結果)
    - [`extract-base64`コマンド](#extract-base64コマンド)
      - [`extract-base64`コマンドの使用例](#extract-base64コマンドの使用例)
      - [`extract-base64`の結果](#extract-base64の結果)
    - [`log-metrics`コマンド](#log-metricsコマンド)
      - [`log-metrics`コマンドの使用例](#log-metricsコマンドの使用例)
      - [`log-metrics`のスクリーンショット](#log-metricsのスクリーンショット)
    - [`logon-summary`コマンド](#logon-summaryコマンド)
      - [`logon-summary`コマンドの使用例](#logon-summaryコマンドの使用例)
      - [`logon-summary`のスクリーンショット](#logon-summaryのスクリーンショット)
    - [`pivot-keywords-list`コマンド](#pivot-keywords-listコマンド)
      - [`pivot-keywords-list`コマンドの使用例](#pivot-keywords-listコマンドの使用例)
      - [`pivot-keywords-list`の設定ファイル](#pivot-keywords-listの設定ファイル)
    - [`search`コマンド](#searchコマンド)
      - [`search`コマンドの使用例](#searchコマンドの使用例)
      - [`search`の設定ファイル](#searchの設定ファイル)
  - [Configコマンド](#configコマンド-1)
    - [`config-critical-systems`コマンド](#config-critical-systemsコマンド)
      - [`config-critical-systems`コマンドの使用例](#config-critical-systemsコマンドの使用例)
  - [DFIRタイムラインコマンド](#dfirタイムラインコマンド)
    - [スキャンウィザード](#スキャンウィザード)
      - [Core ルール](#core-ルール)
      - [Core+ ルール](#core-ルール-1)
      - [Core++ ルール](#core-ルール-2)
      - [Emerging Threats (ET) アドオンルール](#emerging-threats-et-アドオンルール)
      - [Threat Hunting (TH) アドオンルール](#threat-hunting-th-アドオンルール)
    - [Channelベースのイベントログとルールフィルタリング](#channelベースのイベントログとルールフィルタリング)
    - [`csv-timeline`コマンド](#csv-timelineコマンド)
      - [`csv-timeline`コマンドの使用例](#csv-timelineコマンドの使用例)
      - [アドバンス - GeoIPのログエンリッチメント](#アドバンス---geoipのログエンリッチメント)
        - [GeoIPの設定ファイル](#geoipの設定ファイル)
        - [GeoIPデータベースの自動アップデート](#geoipデータベースの自動アップデート)
      - [`csv-timeline`コマンドの設定ファイル](#csv-timelineコマンドの設定ファイル)
    - [`json-timeline`コマンド](#json-timelineコマンド)
      - [`json-timeline`コマンドの使用例と設定ファイル](#json-timelineコマンドの使用例と設定ファイル)
    - [`level-tuning`コマンド](#level-tuningコマンド)
      - [`level-tuning`コマンドの使用例](#level-tuningコマンドの使用例)
      - [`level-tuning`の設定ファイル](#level-tuningの設定ファイル)
    - [`list-profiles`コマンド](#list-profilesコマンド)
    - [`set-default-profile`コマンド](#set-default-profileコマンド)
      - [`set-default-profile`コマンドの使用例](#set-default-profileコマンドの使用例)
  - [`update-rules`コマンド](#update-rulesコマンド)
    - [`update-rules`コマンドの使用例](#update-rulesコマンドの使用例)
- [タイムライン出力](#タイムライン出力)
  - [出力プロファイル](#出力プロファイル)
    - [1. `minimal`プロファイルの出力](#1-minimalプロファイルの出力)
    - [2. `standard`プロファイルの出力](#2-standardプロファイルの出力)
    - [3. `verbose`プロファイルの出力](#3-verboseプロファイルの出力)
    - [4. `all-field-info`プロファイルの出力](#4-all-field-infoプロファイルの出力)
    - [5. `all-field-info-verbose`プロファイルの出力](#5-all-field-info-verboseプロファイルの出力)
    - [6. `super-verbose`プロファイルの出力](#6-super-verboseプロファイルの出力)
    - [7. `timesketch-minimal`プロファイルの出力](#7-timesketch-minimalプロファイルの出力)
    - [8. `timesketch-verbose`プロファイルの出力](#8-timesketch-verboseプロファイルの出力)
    - [プロファイルの比較](#プロファイルの比較)
    - [プロファイルのフィールドエイリアス](#プロファイルのフィールドエイリアス)
      - [その他のプロファイルのフィールドエイリアス](#その他のプロファイルのフィールドエイリアス)
  - [省略](#省略)
    - [Levelの省略](#levelの省略)
    - [MITRE ATT\&CK戦術の省略](#mitre-attck戦術の省略)
    - [Channel情報の省略](#channel情報の省略)
    - [その他の省略](#その他の省略)
  - [プログレスバー](#プログレスバー)
  - [カラー出力](#カラー出力)
  - [結果のサマリ (Results Summary)](#結果のサマリ-results-summary)
    - [検知頻度タイムライン](#検知頻度タイムライン)
- [Hayabusaルール](#hayabusaルール)
  - [Sigma v.s. Hayabusa(ビルトインSigmaとの互換性のある)ルール](#sigma-vs-hayabusaビルトインsigmaとの互換性のあるルール)
- [その他のWindowsイベントログ解析ツールおよび関連リソース](#その他のwindowsイベントログ解析ツールおよび関連リソース)
- [Windowsイベントログ設定のススメ](#windowsイベントログ設定のススメ)
- [Sysmon関係のプロジェクト](#sysmon関係のプロジェクト)
- [コミュニティによるドキュメンテーション](#コミュニティによるドキュメンテーション)
  - [英語](#英語)
  - [日本語](#日本語)
- [貢献](#貢献)
- [バグの報告](#バグの報告)
- [ライセンス](#ライセンス)
- [Twitter](#twitter)

## 主な目的

### スレット(脅威)ハンティングと企業向けの広範囲なDFIR

Hayabusaには現在、4000以上のSigmaルールと170以上のHayabusa検知ルールがあり、定期的にルールが追加されています。
[Velociraptor](https://docs.velociraptor.app/)の[Hayabusa artifact](https://docs.velociraptor.app/exchange/artifacts/pages/windows.eventlogs.hayabusa/)を用いることで企業向けの広範囲なスレットハンティングだけでなくDFIR(デジタルフォレンジックとインシデントレスポンス)にも無料で利用することが可能です。
この2つのオープンソースを組み合わせることで、SIEMが設定されていない環境でも実質的に遡及してSIEMを再現することができます。
具体的な方法は[Eric Capuano](https://twitter.com/eric_capuano)の[こちら](https://www.youtube.com/watch?v=Q1IoGX--814)の動画で学ぶことができます。

### フォレンジックタイムラインの高速生成

Windowsのイベントログは、1）解析が困難なデータ形式であること、2）データの大半がノイズであり調査に有用でないことから、従来は非常に長い時間と手間がかかる解析作業となっていました。
Hayabusaは、有用なデータのみを抽出し、専門的なトレーニングを受けた分析者だけでなく、Windowsのシステム管理者であれば誰でも利用できる読みやすい形式で提示することを主な目的としています。
Hayabusaは従来のWindowsイベントログ分析解析と比較して、分析者が20%の時間で80%の作業を行えるようにすることを目指しています。

![DFIR Timeline](doc/DFIR-TimelineCreation-JP.png)

# スクリーンショット

## 起動

![Hayabusa 起動画面](screenshots/Startup.png)

## DFIRタイムラインのターミナル出力

![Hayabusa ターミナル出力画面](screenshots/Results.png)

## キーワード検索結果

![Hayabusa search results](screenshots/SearchResults.png)

## 検知頻度タイムライン出力 (`-T`オプション)

![Hayabusa 検知頻度タイムライン出力画面](screenshots/DetectionFrequencyTimeline.png)

## 結果サマリ (Results Summary)

![Hayabusa 結果サマリ画面](screenshots/ResultsSummary.png)

## HTMLの結果サマリ (`-H`オプション)

![Hayabusa results summary](screenshots/HTML-ResultsSummary-1.png)

![Hayabusa results summary](screenshots/HTML-ResultsSummary-2.png)

![Hayabusa results summary](screenshots/HTML-ResultsSummary-3.png)

## LibreOfficeでのDFIRタイムライン解析 (`-M` マルチライン出力)

![Hayabusa analysis in LibreOffice](screenshots/DFIR-TimelineLibreOfficeMultiline.jpeg)

## Timeline ExplorerでのDFIRタイムライン解析

![Hayabusa Timeline Explorerでの解析](screenshots/TimelineExplorer-ColoredTimeline.png)

## Criticalアラートのフィルタリングとコンピュータごとのグルーピング

![Timeline ExplorerでCriticalアラートのフィルタリングとコンピュータグルーピング](screenshots/TimelineExplorer-CriticalAlerts-ComputerGrouping.png)

## Elastic Stackダッシュボードでの解析

![Elastic Stack Dashboard 1](doc/ElasticStackImport/17-HayabusaDashboard-1.png)

![Elastic Stack Dashboard 2](doc/ElasticStackImport/18-HayabusaDashboard-2.png)

## Timesketchでの解析

![Timesketch](screenshots/TimesketchAnalysis.png)

# タイムライン結果のインポートと解析について

CSVのタイムラインをTimeline Explorerで分析する方法は[こちら](doc/TimelineExplorerAnalysis/TimelineExplorerAnalysis-Japanese.md)で紹介しています。

CSVのタイムラインをElastic Stackにインポートする方法は[こちら](doc/ElasticStackImport/ElasticStackImport-Japanese.md)で紹介しています。

CSVのタイムラインをTimesketchにインポートする方法は[こちら](doc/TimesketchImport/TimesketchImport-Japanese.md)で紹介しています。

# jqによるJSON形式の結果の解析

JSON形式の結果を`jq`で解析する方法については、[こちら](/doc/AnalysisWithJQ-Japanese.md)を参照してください。

# 特徴＆機能

* クロスプラットフォーム対応: Windows, Linux, macOS。
* Rustで開発され、メモリセーフでハヤブサよりも高速です！
* マルチスレッド対応により、最大5倍のスピードアップを実現。
* フォレンジック調査やインシデントレスポンスのために、分析しやすいCSVタイムラインを作成します。
* 読みやすい/作成/編集可能なYMLベースのHayabusaルールで作成されたIoCシグネチャに基づくスレットハンティング。
* SigmaルールをHayabusaルールに変換するためのSigmaルールのサポートがされています。
* 現在、他の類似ツールに比べ最も多くのSigmaルールをサポートしており、カウントルール、新しい機能の`|equalsfield`や`|endswithfield`等にも対応しています。
* コンピュータ名の統計。(イベントの多い特定のコンピュータをフィルタリングするのに便利です。)
* イベントログの統計。(どのような種類のイベントがあるのかを把握し、ログ設定のチューニングに有効です。)
* 不良ルールやノイズの多いルールを除外するルールチューニング設定が可能です。
* MITRE ATT&CKとのマッピング。
* ルールレベルのチューニング。
* イベントログから不審なユーザやファイルを素早く特定するためのピボットキーワードの一覧作成。
* 詳細な調査のために全フィールド情報の出力。
* 成功と失敗したユーザログオンの要約。
* [Velociraptor](https://docs.velociraptor.app/)と組み合わせた企業向けの広範囲なすべてのエンドポイントに対するスレットハンティングとDFIR。
* CSV、JSON、JSONL形式とHTML結果サマリの出力。
* 毎日のSigmaルール更新。
* JSON形式のログ入力にも対応。
* ログフィールドの正規化
* IPアドレスにGeoIP（ASN、都市、国）情報を付加することによるログエンリッチメント。
* キーワードや正規表現で全イベントの検索。
* フィールドデータのマッピング (例: `0xc0000234` -> `ACCOUNT LOCKED`)
* 空領域からのEvtxレコードカービング。
* 出力時のイベント重複排除。(レコード復元が有効になっている場合や、バックアップされたevtxファイル、VSSから抽出されたevtxファイルなどが含まれている場合に便利。)
* スキャン設定ウィザードにより、有効にするルールの選択が容易に。(誤検出を減らすためなど。）
* PowerShell classicログのフィールドパースと抽出。
* 低メモリモード。(注意: 結果をソートしないことで可能。エージェントやビッグデータでの実行に適している。)
* チャンネルとルールのフィルタリングによって最も効率的なパフォーマンスを達成する。
* ログに含まれるBase64文字列を検出、抽出、デコードする。
* 重要なシステムに基づくアラートレベルの調整。

# ダウンロード

[Releases](https://github.com/Yamato-Security/hayabusa/releases)ページからHayabusaの安定したバージョンでコンパイルされたバイナリが含まれている最新版もしくはソースコードをダウンロードできます。

以下のアーキテクチャ用のバイナリを提供している：
- Linux ARM 64-bit GNU (`hayabusa-x.x.x-lin-aarch64-gnu`)
- Linux Intel 64-bit GNU (`hayabusa-x.x.x-lin-x64-gnu`)
- Linux Intel 64-bit MUSL (`hayabusa-x.x.x-lin-x64-musl`)
- macOS ARM 64-bit (`hayabusa-x.x.x-mac-aarch64`)
- macOS Intel 64-bit (`hayabusa-x.x.x-mac-x64`)
- Windows ARM 64-bit (`hayabusa-x.x.x-win-aarch64.exe`)
- Windows Intel 64-bit (`hayabusa-x.x.x-win-x64.exe`)
- Windows Intel 32-bit (`hayabusa-x.x.x-win-x86.exe`)

> [Linux ARM MUSLバイナリが正常に動作しません](https://github.com/Yamato-Security/hayabusa/issues/1332) そのため、現時点ではそのバイナリを提供していません。これは我々の実装の範囲外の問題なので、修正され次第、将来的に提供する予定です。

## Windowsライブレスポンスパッケージ

v2.18.0から、1つのファイルにまとめたXORエンコードされたルールと、すべての設定ファイルを1つのファイルにまとめた特別なWindowsパッケージを提供しています（[hayabusa-encoded-rules repository](https://github.com/Yamato-Security/hayabusa-encoded-rules)にてホストされています）。
live-responseという名前がついたzipパッケージをダウンロードするだけです。
zipファイルには、Hayabusaのバイナリ、XORエンコードされたルールファイル、設定ファイルの3つのファイルが含まれています。
これらのライブレスポンスパッケージの目的は、クライアントのエンドポイントでHayabusaを実行する際に、Windows Defenderのようなウイルス対策スキャナーが.ymlルールファイルに対して誤検知をしないようにするためです。
また、USNジャーナルなどのフォレンジックアーティファクトが上書きされないよう、システムに書き込まれるファイルの量を最小限に抑えることも目的としています。

# Gitクローン

以下の`git clone`コマンドでレポジトリをダウンロードし、ソースコードからコンパイルして使用することも可能です:

> **注意：** `main`ブランチは開発中のバージョンです。まだ正式にリリースされていない新機能が使えるかもしれないが、バグがある可能性もあるので、テスト版だと思って下さい。

```bash
git clone https://github.com/Yamato-Security/hayabusa.git --recursive
```

> ※ `--recursive`をつけ忘れた場合、サブモジュールとして管理されている`rules`フォルダ内のファイルはダウンロードされません。

`git pull --recurse-submodules`コマンド、もしくは以下のコマンドで`rules`フォルダを同期し、Hayabusaの最新のルールを更新することができます:

```bash
hayabusa.exe update-rules
```

アップデートが失敗した場合は、`rules`フォルダの名前を変更してから、もう一回アップデートしてみて下さい。

>> 注意: アップデートを実行する際に `rules` フォルダは [hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules) レポジトリの最新のルールとコンフィグファイルに置き換えられます
>> 既存ファイルへの修正はすべて上書きされますので、アップデート実行前に編集したファイルのバックアップをおすすめします。
>> もし、`level-tuning` を行っているのであれば、アップデート後にルールファイルの再調整をしてください
>> `rules`フォルダ内に新しく追加したルールは、アップデート時に上書きもしくは削除は行われません。

# アドバンス: ソースコードからのコンパイル（任意）

Rustがインストールされている場合、以下のコマンドでソースコードからコンパイルすることができます:

注意: hayabusaをコンパイルするためにはRust(rustc)が最新版であることが必要です。

```bash
cargo build --release
```

最新のunstable版はmainブランチから、最新の安定版は[Releases](https://github.com/Yamato-Security/hayabusa/releases)ページからダウンロードできます。

以下のコマンドで定期的にRustをアップデートしてください：

```bash
rustup update stable
```

コンパイルされたバイナリは`./target/release`フォルダ配下で作成されます。

## Rustパッケージの更新

コンパイル前に最新のRust crateにアップデートすることで、最新のライブラリを利用することができます:

```bash
cargo update
```

> アップデート後、何か不具合がありましたらお知らせください。

## 32ビットWindowsバイナリのクロスコンパイル

以下のコマンドで64ビットのWindows端末で32ビットのバイナリをクロスコンパイルできます:

```bash
rustup install stable-i686-pc-windows-msvc
rustup target add i686-pc-windows-msvc
rustup run stable-i686-pc-windows-msvc cargo build --release
```

> **注意: Rust の新しい安定版が出たときには必ず`rustup install stable-i686-pc-windows-msvc`を実行してください。`rustup update stable` はクロスコンパイル用のコンパイラを更新しないので、ビルドエラーが発生することがあります。**

## macOSでのコンパイルの注意点

opensslについてのコンパイルエラーが表示される場合は、[Homebrew](https://brew.sh/)をインストールしてから、以下のパッケージをインストールする必要があります：

```bash
brew install pkg-config
brew install openssl
```

## Linuxでのコンパイルの注意点

opensslについてのコンパイルエラーが表示される場合は、以下のパッケージをインストールする必要があります。

Ubuntu系のディストロ:

```bash
sudo apt install libssl-dev
```

Fedora系のディストロ:

```bash
sudo yum install openssl-devel
```

## LinuxのMUSLバイナリのクロスコンパイル

まず、Linux OSでターゲットをインストールします。

```bash
rustup install stable-x86_64-unknown-linux-musl
rustup target add x86_64-unknown-linux-musl
```

以下のようにコンパイルします:

```bash
cargo build --release --target=x86_64-unknown-linux-musl
```

> **注意: Rust の新しい安定版が出たときには必ず`rustup install stable-x86_64-unknown-linux-musl`を実行してください。`rustup update stable` はクロスコンパイル用のコンパイラを更新しないので、ビルドエラーが発生することがあります。**

MUSLバイナリは`./target/x86_64-unknown-linux-musl/release/`ディレクトリ配下に作成されます。
MUSLバイナリはGNUバイナリより約15％遅いですが、より多くのLinuxバージョンとディストロで実行できます。

# Hayabusaの実行

## 注意: アンチウィルス/EDRの誤検知と遅い初回実行

Hayabusa実行する際や、`.yml`ルールのダウンロードや実行時にルール内でdetectionに不審なPowerShellコマンドや`mimikatz`のようなキーワードが書かれている際に、アンチウィルスやEDRにブロックされる可能性があります。
誤検知のため、セキュリティ対策の製品がHayabusaを許可するように設定する必要があります。
マルウェア感染が心配であれば、ソースコードを確認した上で、自分でバイナリをコンパイルして下さい。

Windows PC起動後の初回実行時に時間がかかる場合があります。
これはWindows Defenderのリアルタイムスキャンが行われていることが原因です。
リアルタイムスキャンを無効にするかHayabusaのディレクトリをアンチウィルススキャンから除外することでこの現象は解消しますが、設定を変える前にセキュリティリスクを十分ご考慮ください。

## Windows

コマンドプロンプトやWindows Terminalから32ビットもしくは64ビットのWindowsバイナリをHayabusaのルートディレクトリから実行します。

### パスにスペースが含まれるファイルまたはディレクトリをスキャンしようとするとエラーが発生した場合

Windowsに組み込まれているコマンドプロンプトまたはPowerShellプロンプトを使用する場合、ファイルまたはディレクトリのパスに空白があると、.evtxファイルをロードできないというエラーが表示されることがあります。
.evtxファイルを正しくロードするために、以下のことを行ってください:
1. ファイルまたはディレクトリのパスをダブルクォートで囲む。
2. ディレクトリパスの場合は、最後の文字にバックスラッシュを入れない。

### 文字が正常に表示されない場合

デフォルトのフォントがWindowsの`Lucida Console`の場合、ロゴやテーブルに使用されているさまざまな文字が正しく表示されません。
フォントを`Consolas`に変更することで、これを修正できます。

これにより、ほとんどのテキスト表示の問題は修正されますが、終了メッセージに含まれる日本語文字の表示は修正されません。

![文字化け](screenshots/Mojibake.png)

以下の4つのオプションのいずれかで修正できます：
1. [Windows Terminal](https://learn.microsoft.com/en-us/windows/terminal/)をCommand PromptまたはPowerShellの代わりに使用する。（推奨）
2. `MS Gothic`フォントを使用する。ただし、バックスラッシュが円記号（¥）に変わることに注意してください。
   ![文字化けの修正](screenshots/MojibakeFix.png)
3. [HackGen](https://github.com/yuru7/HackGen/releases)フォントをインストールし、`HackGen Console NF`を使用する。
4. 日本語を含む終了メッセージを表示しないために、`-q, --quiet`オプションを使用する。

## Linux

まず、バイナリに実行権限を与える必要があります。

```bash
chmod +x ./hayabusa
```

次に、Hayabusaのルートディレクトリから実行します：

```bash
./hayabusa
```

## macOS

まず、ターミナルやiTerm2からバイナリに実行権限を与える必要があります。

```bash
chmod +x ./hayabusa
```

次に、Hayabusaのルートディレクトリから実行してみてください：

```bash
./hayabusa
```

macOSの最新版では、以下のセキュリティ警告が出る可能性があります：

![Mac Error 1 JP](screenshots/MacOS-RunError-1-JP.png)

macOSの環境設定から「セキュリティとプライバシー」を開き、「一般」タブから「このまま許可」ボタンをクリックしてください。

![Mac Error 2 JP](screenshots/MacOS-RunError-2-JP.png)

その後、ターミナルからもう一回実行してみてください：

```bash
./hayabusa
```

以下の警告が出るので、「開く」をクリックしてください。

![Mac Error 3 JP](screenshots/MacOS-RunError-3-JP.png)

これで実行できるようになります。

# コマンド一覧

## 分析コマンド:
* `computer-metrics`: コンピュータ名に基づくイベントの合計を出力する。
* `eid-metrics`: イベントIDに基づくイベントの合計と割合の集計を出力する。
* `expand-list`: `expand`のプレースホルダを`rules`フォルダから取り出す。
* `extract-base64`: イベントからbase64文字列を抽出し、デコードする。
* `logon-summary`: ログオンイベントのサマリを出力する。
* `log-metrics`: ログファイルの統計情報を出力する。
* `pivot-keywords-list`: ピボットする不審なキーワードのリストを作成する。
* `search`: キーワードや正規表現で全イベントの検索。

## Configコマンド:
* `config-critical-systems`: ドメインコントローラーやファイルサーバーなどの重要なシステムを見つける。

## DFIRタイムライン作成のコマンド:
* `csv-timeline`: CSV形式のタイムラインを出力する。
* `json-timeline`: JSON/JSONL形式のタイムラインを出力する。
* `level-tuning`: アラート`level`のカスタムチューニング。
* `list-profiles`: 出力プロファイルの一覧表示。
* `set-default-profile`: デフォルトプロファイルを変更する。
* `update-rules`: GitHubの[hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules)リポジトリにある最新のルールに同期させる。

## 汎用コマンド:
* `help`: このメッセージまたは指定されたコマンドのヘルプを表示する。
* `list-contributors`: コントリビュータ一覧の表示。

# コマンド使用方法

## 分析コマンド

### `computer-metrics`コマンド

`computer-metrics`コマンドを使うと、`<System><Computer>`フィールドで定義された各コンピュータに応じたイベントの数をチェックすることができます。
`Computer`フィールドを完全に頼りにしてイベントを元のコンピュータ別に分けることはできないことに注意してください。
Windows 11ではイベントログに保存するときにまったく異なる`Computer`の名前を使うことがあります。
また、Windows 10では`Computer`の名前がすべて小文字で記録されることもあります。
このコマンドは検知ルールを使わないので、すべてのイベントを分析します。
このコマンドは、どのコンピュータに最も多くのログが記録されているかを素早く確認するのに適しています。
この情報があれば、タイムラインを生成する際に`--include-computer`または`--exclude-computer`オプションを使い、コンピュータ別に複数のタイムラインを生成したり、特定のコンピュータからのイベントを除外したりすることで、タイムライン生成をより効率的にすることができます。

```
Usage: computer-metrics <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>  .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>      1つの.evtxファイルに対して解析を行う
  -l, --live-analysis    ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
      --time-offset <OFFSET>      オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)

Output:
  -o, --output <FILE>  イベントIDに基づくイベントの合計と割合の集計を出力する (例: computer-metrics.csv)

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する               
```

#### `computer-metrics`コマンドの使用例

* ディレクトリに対してイベントIDの統計情報を出力する: `hayabusa.exe computer-metrics -d ../logs`
* 結果をCSVファイルに保存する: `hayabusa.exe computer-metrics -d ../logs -o computer-metrics.csv`

#### `computer-metrics`のスクリーンショット

![computer-metrics screenshot](screenshots/ComputerMetrics.png)

### `eid-metrics`コマンド

`eid-metrics`コマンドを使用すると、イベントID(`<System><EventID>`フィールド)の総数や割合をチャンネルごとに分けて表示することができます。
このコマンドは検知ルールを使用しないので、すべてのイベントをスキャンします。

```
Usage: eid-metrics <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う
  -l, --live-analysis          ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)

Output:
  -b, --disable-abbreviations 省略機能を無効にする
  -o, --output <FILE>  イベントIDに基づくイベントの合計と割合の集計を出力する (例: eid-metrics.csv)

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `eid-metrics`コマンドの使用例

* 一つのファイルに対してイベントIDの統計情報を出力する: `hayabusa.exe eid-metrics -f Security.evtx`
* ディレクトリに対してイベントIDの統計情報を出力する: `hayabusa.exe eid-metrics -d ../logs`
* 結果をCSVファイルに保存する: `hayabusa.exe eid-metrics -f Security.evtx -o eid-metrics.csv`

#### `eid-metrics`コマンドの設定ファイル

チャンネル名、イベントID、イベントのタイトルは、`rules/config/channel_eid_info.txt`で定義されています。

例:
```
Channel,EventID,EventTitle
Microsoft-Windows-Sysmon/Operational,1,Process Creation.
Microsoft-Windows-Sysmon/Operational,2,File Creation Timestamp Changed. (Possible Timestomping)
Microsoft-Windows-Sysmon/Operational,3,Network Connection.
Microsoft-Windows-Sysmon/Operational,4,Sysmon Service State Changed.
```

#### `eid-metrics`のスクリーンショット

![eid-metrics screenshot](screenshots/EID-Metrics.png)

### `expand-list`コマンド

ルールフォルダから`expand`プレースホルダーを抽出します。
これは、`expand`フィールド修飾子を使用するルールで利用する設定ファイルを作成する際に役立ちます。
`expand`ルールを使用するには、`./config/expand/`ディレクトリ内に`expand`フィールド修飾子の名前を持つ.txtファイルを作成し、そのファイル内に確認したい値をすべて入力するだけです。

例えば、ルールの`detection`ロジックが次のような場合:
```yaml
detection:
    selection:
        EventID: 5145
        RelativeTargetName|contains: '\winreg'
    filter_main:
        IpAddress|expand: '%Admins_Workstations%'
    condition: selection and not filter_main
```

テキストファイル `./config/expand/Admins_Workstations.txt` を作成し、次のような値を入力します:
```
AdminWorkstation1
AdminWorkstation2
AdminWorkstation3
```

これは本質的に次のロジックと同じ内容を確認します:
```
- IpAddress: 'AdminWorkstation1'
- IpAddress: 'AdminWorkstation2'
- IpAddress: 'AdminWorkstation3'
```

設定ファイルが存在しない場合でも、Hayabusaは`expand`ルールを読み込みますが、それを無視します。

```
Usage:  expand-list <INPUT> [OPTIONS]

General Options:
  -h, --help              ヘルプメニューを表示する
  -r, --rules <DIR/FILE>  ルールファイルまたはルールファイルを持つディレクトリ (デフォルト: ./rules)

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
```

#### `expand-list`コマンドの使用例

* デフォルトの`rules`ディレクトリから`expand`フィールド修飾子を抽出する：`hayabusa.exe expand-list`
* `sigma`ディレクトリから`expand`フィールド修飾子を抽出する：`hayabusa.exe eid-metrics -r ../sigma`

#### `expand-list`結果

```
5 unique expand placeholders found:
Admins_Workstations
DC-MACHINE-NAME
Workstations
internal_domains
domain_controller_hostnames
```

### `extract-base64`コマンド

このコマンドは、次のイベントからBase64文字列を抽出し、それをデコードして、どのようなエンコードが使用されているかを判別します。
* Security 4688 CommandLine
* Sysmon 1 CommandLine, ParentCommandLine
* System 7045 ImagePath
* PowerShell Operational 4104
* PowerShell Operational 4103

```
Usage:  extract-base64 <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>  .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>      1つの.evtxファイルに対して解析を行う
  -l, --live-analysis    ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)

Output:
  -o, --output <FILE>  Base64文字列を抽出する

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `extract-base64`コマンドの使用例

* ディレクトリをスキャンし、結果をターミナルに出力します: `hayabusa.exe  extract-base64 -d ../hayabusa-sample-evtx`
* ディレクトリをスキャンし、結果をCSVファイルに出力します: `hayabusa.exe eid-metrics -r ../sigma -o base64-extracted.csv`

#### `extract-base64`の結果

ターミナルに出力する際、スペースに制限があるため、次のフィールドのみが表示されます：
  * Timestamp
  * Computer
  * Base64 String
  * Decoded String (if not binary)

CSVファイルに保存する際、次のフィールドが保存されます：
  * Timestamp
  * Computer
  * Base64 String
  * Decoded String (if not binary)
  * Original Field
  * Length
  * Binary (`Y/N`)
  * Double Encoding (`Y`の場合、それは通常悪意があります。)
  * Encoding Type
  * File Type
  * Event
  * Record ID
  * File Name

### `log-metrics`コマンド

`log-metrics`コマンドを使うと、イベントログ内の以下のメタデータを出力することができる:
* ファイル名
* コンピュータ名
* イベント数
* 最初のタイムスタンプ
* 最後のタイムスタンプ
* チャネル
* プロバイダー

このコマンドは検知ルールを使用しないので、すべてのイベントをスキャンする。

```
Usage: log-metrics <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う
  -l, --live-analysis          ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)

Output:
  -b, --disable-abbreviations  省略機能を無効にする
  -M, --multiline              イベントフィールド情報を複数の行に出力する
  -o, --output <FILE>          メトリクスをCSV形式で保存する (例: metrics.csv)
  -S, --tab-separator          フィールドをタブ区切りにする

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `log-metrics`コマンドの使用例

* ファイルからログファイルのメトリクスを出力する: `hayabusa.exe log-metrics -f Security.evtx`
* ディレクトリからログファイルのメトリクスを出力する: `hayabusa.exe log-metrics -d ../logs`
* 結果をCSVファイルに保存: `hayabusa.exe log-metrics -d ../logs -o eid-metrics.csv`

#### `log-metrics`のスクリーンショット

![log-metricsスクリーンショット](screenshots/LogMetrics.png)

### `logon-summary`コマンド

`logon-summary`コマンドを使うことでログオン情報の要約(ユーザ名、ログイン成功数、ログイン失敗数)の画面出力ができます。
単体のevtxファイルを解析したい場合は`-f`オプションを利用してください。複数のevtxファイルを対象としたい場合は`-d`オプションを合わせて使うことでevtxファイルごとのログイン情報の要約を出力できます。

ログオン成功は、以下のイベントから取得される:
* `Security 4624` (ログオン成功)
* `RDS-LSM 21` (リモートデスクトップサービス ローカルセッションマネージャーのログオン)
* `RDS-GTW 302` (リモートデスクトップサービス ゲートウェイのログオン)

ログオン失敗は、`Security 4625`イベントから取得される

```
Usage: logon-summary <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う
  -l, --live-analysis          ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する (例１: evtx_data 例２:evtx1,evtx2)

Filtering:
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --timeline-end <DATE>             解析対象とするイベントログの終了時刻 (例: "2022-02-22 23:59:59 +09:00")
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)
      --timeline-start <DATE>           解析対象とするイベントログの開始時刻 (例: "2020-02-22 00:00:00 +09:00")

Output:
  -o, --output <FILENAME-PREFIX>  ログオンサマリをCSV形式で２つのファイルに保存する (例: -o logon-summary.csv)

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `logon-summary`コマンドの使用例

* ログオンサマリの出力: `hayabusa.exe logon-summary -f Security.evtx`
* ログオンサマリ結果を保存する: `hayabusa.exe logon-summary -d ../logs -o logon-summary.csv`

#### `logon-summary`のスクリーンショット

![logon-summary successful logons screenshot](screenshots/LogonSummarySuccessfulLogons.png)

![logon-summary failed logons screenshot](screenshots/LogonSummaryFailedLogons.png)

### `pivot-keywords-list`コマンド

`pivot-keywords-list`コマンドを使用すると、異常なユーザ、ホスト名、プロセスなどを迅速に特定し、イベントを関連付けるための固有のピボットキーワードのリストを作成することができます。

重要：デフォルトでは、Hayabusaはすべてのイベント（informationalおよびそれ以上）から結果を返すので、`pivot-keywords-list`コマンドと`-m, --min-level`オプションを組み合わせることを強くお勧めします。
例えば、まず`-m critical`で`critical`アラートのみのキーワードを作成し、次に`-m high`、`-m medium`等々と続けていきます。
検索結果には、多くの通常のイベントと一致する共通のキーワードが含まれている可能性が高いので、検索結果を手動でチェックし、固有のキーワードのリストを1つのファイルに作成した後、`grep -f keywords.txt timeline.csv`といったコマンドで疑わしい活動のタイムラインを絞り込み作成することが可能です。

```
Usage: pivot-keywords-list <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う
  -l, --live-analysis          ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                        結果ファイルを上書きする
  -h, --help                           ヘルプメニューを表示する
  -J, --JSON-input                     .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -w, --no-wizard                      質問はしない。すべてのイベントとアラートをスキャンする
  -Q, --quiet-errors                   Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>             ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>               スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>  evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
  -E, --EID-filter                      速度を上げるため主なEIDだけスキャンする (コンフィグファイル: ./rules/config/target_event_IDs.txt)
  -D, --enable-deprecated-rules         ステータスがdeprecatedのルールを有効にする
  -n, --enable-noisy-rules              Noisyルールを有効にする
  -u, --enable-unsupported-rules        ステータスがunsupportedのルールを有効にする
  -e, --exact-level <LEVEL>             特定のレベルだけスキャンする (informational, low, medium, high, critical)
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --exclude-eid <EID...>            高速化のために特定のEIDをスキャンしない (例: 1) (例: 1,4688)
      --exclude-status <STATUS...>      読み込み対象外とするルール内でのステータス (例１: experimental) (例２: stable,test)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --exclude-tag <TAG...>            特定のタグを持つルールをロードしない (例: sysmon)
      --include-eid <EID...>            指定したEIDのみをスキャンして高速化する (例 1) (例: 1,4688)
      --include-status <STATUS...>      特定のステータスを持つルールのみをロードする (例: expermimental) (例: stable,test)
      --include-tag <TAG...>            特定のタグを持つルールのみをロードする (例１: attack.execution,attack.discovery) (例２: wmi)
  -m, --min-level <LEVEL>               結果出力をするルールの最低レベル (デフォルト: informational)
      --timeline-end <DATE>             解析対象とするイベントログの終了時刻 (例: "2022-02-22 23:59:59 +09:00")
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)
      --timeline-start <DATE>           解析対象とするイベントログの開始時刻 (例: "2020-02-22 00:00:00 +09:00")

Output:
  -o, --output <FILENAME-PREFIX>  ピボットキーワードの一覧を複数ファイルに出力する (例: PivotKeywords)

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する               
```

#### `pivot-keywords-list`コマンドの使用例

* ピボットキーワードを画面に出力します: `hayabusa.exe pivot-keywords-list -d ../logs -m critical`
* 重要なアラートからピボットキーワードのリストを作成し、その結果を保存します。(結果は、`keywords-Ip Addresses.txt`、`keywords-Users.txt`等に保存されます):

```
hayabusa.exe pivot-keywords-list -d ../logs -m critical -o keywords
```

#### `pivot-keywords-list`の設定ファイル

検索キーワードは、`./rules/config/pivot_keywords.txt`を編集することでカスタマイズすることができます。
デフォルト設定は[こちらのページ](https://github.com/Yamato-Security/hayabusa-rules/blob/main/config/pivot_keywords.txt)です。


フォーマットは、`キーワード名.フィールド名`です。例えば、`Users`のリストを作成する場合、Hayabusaは、`SubjectUserName`、`TargetUserName`、`User`フィールドにあるすべての値をリストアップします。


### `search`コマンド

`search`コマンドは、すべてのイベントのキーワード検索が可能です。
(※Hayabusaの検知結果だけではありません。）
Hayabusaの検知ルールでなにかの痕跡を検知できなくても、検索機能で検知できる可能性があるので、便利です。

```
Usage: hayabusa.exe search <INPUT> <--keywords "<KEYWORDS>" OR --regex "<REGEX>"> [OPTIONS]

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

General Options:
  -C, --clobber                          結果ファイルを上書きする
  -h, --help                             ヘルプメニューを表示する
  -Q, --quiet-errors                     Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                  空ページからevtxレコードをカービングする (デフォルト: 無効)
  -c, --rules-config <DIR>               ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -t, --threads <NUMBER>                 スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>    evtx以外の拡張子を解析対象に追加する (例１: evtx_data 例２:evtx1,evtx2)
  -s, --sort                             ファイル保存前にイベントをソートする (警告: これは多くのメモリを使用する!)

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う
  -l, --live-analysis          ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する
  
Filtering:
  -a, --and-logic                    ANDロジックでキーワード検索を行う (デフォルト: OR)
  -F, --filter <FILTER...>           特定のフィールドでフィルタする
  -i, --ignore-case                  大文字と小文字を区別しない
  -k, --keywords <KEYWORD...>        キーワードでの検索
  -r, --regex <REGEX>                正規表現での検索
      --time-offset <OFFSET>         オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)
      --timeline-end <DATE>             解析対象とするイベントログの終了時刻 (例: "2022-02-22 23:59:59 +09:00")
      --timeline-start <DATE>           解析対象とするイベントログの開始時刻 (例: "2020-02-22 00:00:00 +09:00")

Output:
  -b, --disable-abbreviations        省略機能を無効にする
  -J, --JSON-output                  JSON形式で検索結果を保存する (例: -J -o results.json)
  -L, --JSONL-output                 JSONL形式で検索結果を保存 (例: -L -o results.jsonl)
  -M, --multiline                    イベントフィールド情報を複数の行に出力する
  -o, --output <FILE>                ログオンサマリをCSV形式で保存する (例: search.csv)
  -S, --tab-separator                フィールドをタブ区切りにする

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `search`コマンドの使用例

* `../hayabusa-sample-evtx`ディレクトリで`mimikatz`のキーワードを検索する:

```
hayabusa.exe search -d ../hayabusa-sample-evtx -k "mimikatz"
```

> 注意: `mimikatz`のキーワードがデータ内のどこかに存在する場合にマッチする。完全一致しなくても良い。

* `../hayabusa-sample-evtx`ディレクトリで`mimikatz`または`kali`のキーワードを検索する:

```
hayabusa.exe search -d ../hayabusa-sample-evtx -k "mimikatz" -k "kali"
```

* `../hayabusa-sample-evtx`ディレクトリで大文字小文字を区別せずに`mimikatz`のキーワードを検索する:

```
hayabusa.exe search -d ../hayabusa-sample-evtx -k "mimikatz" -i
```

* `../hayabusa-sample-evtx`ディレクトリで正規表現を使用し、IPアドレスを検索する:

```
hayabusa.exe search -d ../hayabusa-sample-evtx -r "(?:[0-9]{1,3}\.){3}[0-9]{1,3}"
```

* `../hayabusa-sample-evtx`ディレクトリで`WorkstationName`フィールドが`kali`の条件で、全イベントを表示する:

```
hayabusa.exe search -d ../hayabusa-sample-evtx -r ".*" -F WorkstationName:"kali"
```

> ※ `.*`の正規表現を使用すると、すべてのイベントが表示される。

#### `search`の設定ファイル

`./rules/config/channel_abbreviations.txt`: チャンネル名とその略称のマッピング。

## Configコマンド

### `config-critical-systems`コマンド

このコマンドは、自動的にドメインコントローラーやファイルサーバーなどの重要なシステムを見つけ、`./config/critical_systems.txt`コンフィグファイルに追加します。そのためすべてのアラートが1つ上のレベルになります。
ドメインコントローラーかどうかを判断するためにSecurity 4768 (Kerberos TGT requested)イベントを検索します。
ファイルサーバーかどうかを判断するためにSecurity 5145 (Network Share File Access)イベントを検索します。
`critical_systems.txt`ファイルに追加されたホスト名は、すべてのアラートが1つ上のレベルになり、最大で`emergency`レベルになります。

```
Usage: hayabusa.exe config-critical-systems <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>        .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>            1つの.evtxファイルに対して解析を行う

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない
  -v, --verbose   詳細な情報を出力する

General Options:
  -h, --help      ヘルプメニューを表示する
```

#### `config-critical-systems`コマンドの使用例

* `../hayabusa-sample-evtx`ディレクトリでドメインコントローラーとファイルサーバーを検索する:

```
hayabusa.exe config-critical-systems -d ../hayabusa-sample-evtx"
```

## DFIRタイムラインコマンド

### スキャンウィザード

`csv-timeline`や`json-timeline`などのコマンドは、デフォルトでスキャンウィザードが有効になりました。
これは、ユーザのニーズや好みに応じて、どの検知ルールを有効にするかを簡単に選択できるようにするためのものであります。
読み込む検知ルールのセットは、Sigmaプロジェクトの公式リストに基づいています。
詳細は[このブログ記事](https://blog.sigmahq.io/introducing-sigma-rule-packages-releases-76043ce42e81)で説明されています。
`w, --no-wizard`オプションを追加することで、簡単にウィザードを無効にし、従来の方法でHayabusaを使用できます。

#### Core ルール

`core`ルールセットは、ステータスが`test`または`stable`かつ、レベルが`high`または`critical`のルールを有効にします。
これらは高品質のルールで、多くの誤検知は発生しないはずです。
ルールのステータスが`test`または`stable`であるため、6ヶ月以上の間に誤検知が報告されていません。
ルールは攻撃者の戦術、一般的な不審なアクティビティ、または悪意のある振る舞いに一致します。
これは`--exclude-status deprecated,unsupported,experimental --min-level high`オプションを使用した場合と同じです。

#### Core+ ルール

`core+`ルールセットは、ステータスが`test`または`stable`かつ、レベルが`medium`以上のルールを有効にします。
`medium`ルールは、しばしば特定のアプリケーション、正当なユーザーの行動、または組織のスクリプトと一致するため、追加のチューニングが必要です。
これは`--exclude-status deprecated,unsupported,experimental --min-level medium`オプションを使用した場合と同じです。

#### Core++ ルール

`core++`ルールセットは、ステータスが`experimental`、`test`、`stable`のいずれかかつ、レベルが`medium`以上のルールを有効にします。
これらのルールは最先端のものです。
これらはSigmaHQプロジェクトで提供されているベースラインのevtxファイルに対して検証され、複数のエンジニアによってレビューされています。
それ以外最初は、ほとんどテストされていません。
これらは、できるだけ早く脅威を検出できる場合に使用しますが、誤検知のしきい値を高く保つのにコストがかかります。
これは`--exclude-status deprecated,unsupported --min-level medium`オプションを使用した場合と同じです。

#### Emerging Threats (ET) アドオンルール

`Emerging Threats (ET)`ルールセットは、`detection.emerging_threats`のタグを持つルールを有効にします。
これらのルールは特定の脅威を対象とし、情報がまだほとんど入手できていない現在の脅威に特に役立ちます。
これらのルールは多くの誤検知を生成しないはずですが、時間とともに関連性が低下します。
これらのルールが無効になっている場合、`--exclude-tag detection.emerging_threats`オプションを使用した場合と同じです。
ウィザードを無効にしてHayabusaを従来の方法で実行する場合、これらのルールはデフォルトで含まれます。

#### Threat Hunting (TH) アドオンルール

`Threat Hunting (TH)`ルールセットは、`detection.threat_hunting`のタグを持つルールを有効にします。
これらのルールは未知の悪意のあるアクティビティを検出するかもしれませんが、通常は誤検知が多くなります。
これらのルールが無効になっている場合、`--exclude-tag detection.threat_hunting`オプションを使用した場合と同じです。
ウィザードを無効にしてHayabusaを従来の方法で実行する場合、これらのルールはデフォルトで含まれます。

### Channelベースのイベントログとルールフィルタリング

Hayabusa v2.16.0以降、`.evtx`ファイルと`.yml`ルールを読み込む際にチャンネルベースのフィルタを有効にしています。
これは、必要なものだけを読み込むことで、スキャンを可能な限り効率的に行うことを目的としています。
単一のイベントログ内に複数のプロバイダが存在することはありますが、単一の.evtxファイル内に複数のチャンネルが含まれることは一般的ではありません。
（これまで見かけた唯一の例は、異なる2つの.evtxファイルを人工的に結合した[sample-evtx](https://github.com/Yamato-Security/hayabusa-sample-evtx)プロジェクトです。）
この特性を利用して、スキャン対象のすべての`.evtx`ファイルの最初のレコードで`Channel`フィールドを確認します。
また、ルールの`Channel`フィールドに指定されたチャンネルを使用する`.yml`ルールも確認します。
この2つのリストを基に、実際に`.evtx`ファイル内に存在するチャンネルを使用するルールだけを読み込みます。

例えば、ユーザーが`Security.evtx`をスキャンしたい場合、`Channel: Security`を指定しているルールのみが使用されます。
他の検出ルール、例えば`Application`ログのイベントのみを検出するルールなどを読み込む意味はありません。
なお、チャンネルフィールド（例: `Channel: Security`）は、元のSigmaルールには**明示的**に定義されていません。
Sigmaルールでは、`logsource`の`service`や`category`フィールドでチャンネルやイベントIDが**暗黙的**に定義されています（例: `service: security`）
[hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules)リポジトリでSigmaルールを管理する際には、`logsource`フィールドを具体化し、チャンネルやイベントIDフィールドを明示的に定義しています。
これをどのように、そしてなぜ行うのかについては、[こちら](https://github.com/Yamato-Security/sigma-to-hayabusa-converter)で詳しく説明しています。

現在、`Channel`が定義されておらず、すべての`.evtx`ファイルをスキャンするためのルールは以下の2つだけです：
- [Possible Hidden Shellcode](https://github.com/Yamato-Security/hayabusa-rules/blob/main/hayabusa/builtin/UnkwnChannEID_Med_PossibleHiddenShellcode.yml)
- [Mimikatz Use](https://github.com/SigmaHQ/sigma/blob/master/rules/windows/builtin/win_alert_mimikatz_keywords.yml)

これらの2つのルールを使用して、読み込んだすべての`.evtx`ファイルに対してルールをスキャンしたい場合は、`csv-timeline`および`json-timeline`コマンドで`-A, --enable-all-rules`オプションを追加する必要があります。
ベンチマークでは、ルールフィルタリングにより、スキャンするファイルに応じて、速度が20%から10倍に向上することが確認されています。

チャンネルフィルタリングは、`.evtx`ファイルを読み込む際にも使用されます。
例えば、`Security`チャンネルのイベントを探すルールを指定している場合、`Security`ログではない`.evtx`ファイルを読み込む意味はありません。
ベンチマークでは、通常のスキャンで約10%、単一のルールでスキャンする場合には最大60%以上の性能向上が見られました。
1つの.evtxファイル内に複数のチャンネルが使用されている場合、例えば複数の`.evtx`ファイルがツールを使って結合された場合は、`csv-timeline`および`json-timeline`コマンドで`-a, --scan-all-evtx-files`オプションを使用してこのフィルタリングを無効にできます。

> 注意: チャンネルフィルタリングは.evtxファイルでのみ動作します。-J, --json-inputでJSONファイルからイベントログを読み込み、さらに-Aや-aを指定した場合、エラーが発生します。

### `csv-timeline`コマンド

`csv-timeline`コマンドはイベントのフォレンジックタイムラインをCSV形式で作成します。

```
Usage: csv-timeline <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>    .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>        1つの.evtxファイルに対して解析を行う
  -l, --live-analysis      ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                          結果ファイルを上書きする
  -h, --help                             ヘルプメニューを表示する
  -J, --JSON-input                       .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -w, --no-wizard                        質問はしない。すべてのイベントとアラートをスキャンする
  -Q, --quiet-errors                     Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                  空ページからevtxレコードをカービングする (デフォルト: 無効)
  -r, --rules <DIR/FILE>                 ルールファイルまたはルールファイルを持つディレクトリ (デフォルト: ./rules)
  -c, --rules-config <DIR>               ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -s, --sort                             ファイル保存前にイベントをソートする (警告: これは多くのメモリを使用する!)
  -t, --threads <NUMBER>                 スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>    evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
  -E, --EID-filter                      速度を上げるため主なEIDだけスキャンする (コンフィグファイル: ./rules/config/target_event_IDs.txt)
  -A, --enable-all-rules                ロードされたevtxファイルに関係なく、すべてのルールを有効にする（ルールのチャネルフィルターを無効にする）
  -D, --enable-deprecated-rules         ステータスがdeprecatedのルールを有効にする
  -n, --enable-noisy-rules              Noisyルールを有効にする
  -u, --enable-unsupported-rules        ステータスがunsupportedのルールを有効にする
  -e, --exact-level <LEVEL>             特定のレベルだけスキャンする (informational, low, medium, high, critical)
      --exclude-category <CATEGORY...>  特定のlogsourceカテゴリを持つルールをロードしない (例: process_creation,pipe_created)
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --exclude-eid <EID...>            高速化のために特定のEIDをスキャンしない (例: 1) (例: 1,4688)
      --exclude-status <STATUS...>      読み込み対象外とするルール内でのステータス (例１: experimental) (例２: stable,test)
      --exclude-tag <TAG...>            特定のタグを持つルールをロードしない (例: sysmon)
      --include-category <CATEGORY...>  特定のlogsourceカテゴリを持つルールのみをロードする (例: process_creation,pipe_created)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --include-eid <EID...>            指定したEIDのみをスキャンして高速化する (例: 1) (例: 1,4688)
      --include-status <STATUS...>      特定のステータスを持つルールのみをロードする (例: expermimental) (例: stable,test)
      --include-tag <TAG...>            特定のタグを持つルールのみをロードする (例１: attack.execution,attack.discovery) (例２: wmi)
  -m, --min-level <LEVEL>               結果出力をするルールの最低レベル (デフォルト: informational)
  -P, --proven-rules                    実績のあるルールだけでスキャンし、高速化する (./rules/config/proven_rules.txt)
  -a, --scan-all-evtx-files             ロードされたルールに関係なく、すべてのevtxファイルをスキャンする（evtxファイルのチャネルフィルターを無効にする）
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)
      --timeline-end <DATE>             解析対象とするイベントログの終了時刻 (例: "2022-02-22 23:59:59 +09:00")
      --timeline-start <DATE>           解析対象とするイベントログの開始時刻 (例: "2020-02-22 00:00:00 +09:00")

Output:
  -b, --disable-abbreviations        省略機能を無効にする
  -G, --GeoIP <MAXMIND-DB-DIR>       IPアドレスのGeoIP(ASN、都市、国)情報を追加する
  -H, --HTML-report <FILE>           HTML形式で詳細な結果を出力する (例: results.html)
  -M, --multiline                    イベントフィールド情報を複数の行に出力する
  -F, --no-field-data-mapping        フィールドデータのマッピングを無効にする
      --no-pwsh-field-extraction     PowerShell Classicログフィールド抽出の無効化
  -o, --output <FILE>                タイムラインを保存する (例: results.csv)
  -p, --profile <PROFILE>            利用する出力プロファイル名を指定する
  -R, --remove-duplicate-data        重複したフィールドデータは「DUP」に置き換えられる (ファイルサイズが約10〜15％削減される)
  -X, --remove-duplicate-detections  重複した検知項目を削除する (デフォルト: 無効)
  -S, --tab-separator                フィールドをタブ区切りにする

Display Settings:
  -K, --no-color            カラーで出力しない
  -N, --no-summary          結果概要を出力しない (多少速くなる)
  -q, --quiet               Quietモード: 起動バナーを表示しない
  -v, --verbose             詳細な情報を出力する
  -T, --visualize-timeline  検知頻度タイムラインを出力する（ターミナルはUnicodeに対応する必要がある）

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `csv-timeline`コマンドの使用例

* デフォルトの`standard`プロファイルで１つのWindowsイベントログファイルに対してHayabusaを実行する:

```
hayabusa.exe csv-timeline -f eventlog.evtx 
```

* `verbose`プロファイルで複数のWindowsイベントログファイルのあるsample-evtxディレクトリに対して、Hayabusaを実行する:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -p verbose
```

* 全てのフィールド情報も含めて１つのCSVファイルにエクスポートして、LibreOffice、Timeline Explorer、Elastic Stack等でさらに分析することができる(注意: `super-verbose`プロファイルを使すると、出力するファイルのサイズがとても大きくなる！):

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -o results.csv -p super-verbose
```

* EID(イベントID)フィルタを有効にし、タイムラインをJSON形式で保存する:

> 注意: EIDフィルタを有効にすると、私達のテストでは処理時間が約10〜15%速くなりますが、アラートを見逃す可能性があります。

```
hayabusa.exe csv-timeline -E -d .\hayabusa-sample-evtx -o results.csv
```

* Hayabusaルールのみを実行する（デフォルトでは`-r .\rules`にあるすべてのルールが利用される）:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -r .\rules\hayabusa -o results.csv -w
```

* Windowsでデフォルトで有効になっているログに対してのみ、Hayabusaルールを実行する:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -r .\rules\hayabusa\builtin -o results.csv -w
```

* Sysmonログに対してのみHayabusaルールを実行する:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -r .\rules\hayabusa\sysmon -o results.csv -w
```

* Sigmaルールのみを実行する:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -r .\rules\sigma -o results.csv -w
```

* 廃棄(deprecated)されたルール(`status`が`deprecated`になっているルール)とノイジールール(`.\rules\config\noisy_rules.txt`にルールIDが書かれているルール)を有効にする:

> 注意: 最近、廃止されたルールはSigmaリポジトリで別のディレクトリに置かれるようになり、Hayabusaではもうデフォルトでは含まれないようになりました。
> 従って、廃止されたルールを有効にする必要はないでしょう。

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx --enable-noisy-rules --enable-deprecated-rules -o results.csv -w
```

* ログオン情報を分析するルールのみを実行し、UTCタイムゾーンで出力する:

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -r .\rules\hayabusa\builtin\Security\LogonLogoff\Logon -U -o results.csv -w
```

* 起動中のWindows端末上で実行し（Administrator権限が必要）、アラート（悪意のある可能性のある動作）のみを検知する:

```
hayabusa.exe csv-timeline -l -m low
```

* 詳細なメッセージを出力する(処理に時間がかかるファイル、パースエラー等を特定するのに便利):

```
hayabusa.exe csv-timeline -d .\hayabusa-sample-evtx -v
```

* Verbose出力の例:

ルールファイルの読み込み:

```
Loaded rule: rules/sigma/builtin/deprecated/proc_creation_win_susp_run_folder.yml
Loaded rule: rules/sigma/builtin/deprecated/proc_creation_win_execution_mssql_xp_cmdshell_stored_procedure.yml
Loaded rule: rules/sigma/builtin/deprecated/proc_creation_win_susp_squirrel_lolbin.yml
Loaded rule: rules/sigma/builtin/win_alert_mimikatz_keywords.yml
```

スキャン中のエラー:
```
[ERROR] Failed to parse event file.
EventFile: ../logs/Microsoft-Rdms-UI%4Operational.evtx
Error: Failed to parse record number 58471

[ERROR] Failed to parse event file.
EventFile: ../logs/Microsoft-Rdms-UI%4Operational.evtx
Error: Failed to parse record number 58470

[ERROR] Failed to parse event file.
EventFile: ../logs/Microsoft-Windows-AppxPackaging%4Operational.evtx
Error: An error occurred while trying to serialize binary xml to output.
```

* 結果を[Timesketch](https://timesketch.org/)にインポートできるCSV形式に保存する:

```
hayabusa.exe csv-timeline -d ../hayabusa-sample-evtx --RFC-3339 -o timesketch-import.csv -p timesketch -U
```

* エラーログの出力をさせないようにする:
デフォルトでは、Hayabusaはエラーメッセージをエラーログに保存します。
エラーメッセージを保存したくない場合は、`-Q`を追加してください。

#### アドバンス - GeoIPのログエンリッチメント

無償のGeoLite2のジオロケーションデータで、SrcIP（ソースIPアドレス）フィールドとTgtIP（ターゲットIPアドレス）フィールドにGeoIP（ASN組織、都市、国）情報を追加することができます。

手順:
1. まずMaxMindのアカウントを[こちら](https://dev.maxmind.com/geoip/geolite2-free-geolocation-data)で登録してください。
2. [ダウンロードページ](https://www.maxmind.com/en/accounts/current/geoip/downloads)から3つの`.mmdb`ファイルをダウンロードし、ディレクトリに保存してください。ファイル名は、`GeoLite2-ASN.mmdb`、`GeoLite2-City.mmdb`、`GeoLite2-Country.mmdb`であることをご確認ください。
3. `csv-timeline`または`json-timeline`コマンドを実行する際には、`-G`オプションの後にMaxMindデータベースのあるディレクトリを追加してください。

* `csv-timeline`を使用すると、次の6つのカラムが追加で出力されます: `SrcASN`、`SrcCity`、`SrcCountry`、`TgtASN`、`TgtCity`、`TgtCountry`
* `json-timeline`を使用すると、同じ`SrcASN`、`SrcCity`、`SrcCountry`、`TgtASN`、`TgtCity`、`TgtCountry`フィールドが`Details`オブジェクトに追加されますが、情報を含む場合のみとなります。

* `SrcIP`または`TgtIP`がlocalhost (`127.0.0.1`、`::1`等々)の場合、`SrcASN`または`TgtASN`は、`Local`として出力されます。
* `SrcIP`または`TgtIP`がプライベートIPアドレス (`10.0.0.0/8`、`fe80::/10`等々)の場合、`SrcASN`または`TgtASN`は、`Private`として出力されます。

##### GeoIPの設定ファイル

GeoIPデータベースで検索される送信元と送信先のIPアドレスを含むフィールド名は、`rules/config/geoip_field_mapping.yaml`で定義されています。
必要であれば、このリストに追加することができます。
また、このファイルには、どのイベントからIPアドレス情報を抽出するかを決定するフィルタセクションもあります。

##### GeoIPデータベースの自動アップデート

MaxMind GeoIP データベースは、2 週間ごとに更新されます。
これらのデータベースを自動的に更新するために、[こちら](https://github.com/maxmind/geoipupdate)からMaxMindの`geoipupdate`のツールをインストールすることができます。

macOSでの手順:
1. `brew install geoipupdate`
2. `/usr/local/etc/GeoIP.conf`を編集する: MaxMindのウェブサイトにログインした後に作成した`AccountID`と`LicenseKey`を入れる。`EditionIDs`の行に、`EditionIDs GeoLite2-ASN GeoLite2-City GeoLite2-Country`とあることを確認する。
3. `geoipupdate`を実行する。
4. GeoIP情報を追加する場合は、`-G /usr/local/var/GeoIP`を追加する。

Windowsでの手順:
1. [Releases](https://github.com/maxmind/geoipupdate/releases)ページからWindowsバイナリの最新版(例: `geoipupdate_4.10.0_windows_amd64.zip`)をダウンロードする。
2. `\ProgramData\MaxMind/GeoIPUpdate\GeoIP.conf`を編集する: MaxMindのウェブサイトにログインした後に作成した`AccountID`と`LicenseKey`を入れる。`EditionIDs`の行に、`EditionIDs GeoLite2-ASN GeoLite2-City GeoLite2-Country`とあることを確認する。
3. `geoipupdate`を実行する。

#### `csv-timeline`コマンドの設定ファイル

`./rules/config/channel_abbreviations.txt`: チャンネル名とその略称のマッピング。

`./rules/config/default_details.txt`: ルールに`details:`行が指定されていない場合に、どのようなデフォルトのフィールド情報 (`%Details%`フィールド)を出力するかを設定するファイルです。
プロバイダー名とイベントIDを元に作成されます。

`./rules/config/eventkey_alias.txt`: このファイルには、フィールドの短い名前のエイリアスと、元の長いフィールド名のマッピングがあります。

例:
```
InstanceID,Event.UserData.UMDFHostDeviceArrivalBegin.InstanceId
IntegrityLevel,Event.EventData.IntegrityLevel
IpAddress,Event.EventData.IpAddress
```

ここでフィールドが定義されていない場合、Hayabusaは自動的に`Event.EventData`にあるフィールドを使用してみます。

`./rules/config/exclude_rules.txt`: このファイルには、使用から除外されるルールIDのリストがあります。
通常は、あるルールが別のルールに置き換わったか、そもそもそのルールが使用できないことが原因です。
ファイアウォールやIDSと同様に、シグネチャベースのツールは、自身の環境に合わせてチューニングする必要があるため、特定のルールを恒久的または一時的に除外する必要があるかもしれません。
`./rules/config/exclude_rules.txt`にルールID (例:`4fe151c2-ecf9-4fae-95ae-b88ec9c2fca6`)を追加すると、不要なルールや使用できないルールを無視できます。

`./rules/config/noisy_rules.txt`: このファイルには、デフォルトでは無効になっているルールのIDが入っています。`-n, --enable-noisy-rules`オプションでノイジールールを有効にできます。
これらのルールは通常、性質上ノイズが多いか、誤検出があるためです。

`./rules/config/target_event_IDs.txt`: EIDフィルターが有効な場合、このファイルで指定されたイベントIDのみがスキャンされます。
デフォルトでは、Hayabusaはすべてのイベントをスキャンしますが、パフォーマンスを向上させたい場合は、`-E, --EID-filter`オプションを使用してください。
これにより、通常10〜25％の速度向上があります。


### `json-timeline`コマンド

`json-timeline`コマンドは、JSONまたはJSONL形式でイベントのフォレンジックタイムラインを作成します。
JSONLへの出力は、JSONよりも高速でファイルサイズも小さいので、結果をElastic Stack等の他のツールにインポートするだけなら、JSONLが理想です。
テキストエディタで手動で解析する場合は、JSONの方が良いでしょう。
CSV出力は小さいタイムライン(通常2GB以下)をLibreOfficeやTimeline Explorerのようなツールにインポートするのに適しています。
JSONは、`jq`等のツールでデータ(大きな結果ファイルを含む)をより詳細に分析する場合に最適です。`Details`フィールドが分離されているので、分析が容易になるからです。
(CSV出力では、すべてのイベントログのフィールドが1つの大きな`Details`カラムに入っており、データのソートなどが難しくなっています。)

```
Usage: json-timeline <INPUT> [OPTIONS]

Input:
  -d, --directory <DIR>    .evtxファイルを持つディレクトリのパス
  -f, --file <FILE>        1つの.evtxファイルに対して解析を行う
  -l, --live-analysis      ローカル端末のC:\Windows\System32\winevt\Logsフォルダを解析する

General Options:
  -C, --clobber                          結果ファイルを上書きする
  -h, --help                             ヘルプメニューを表示する
  -J, --JSON-input                       .evtxファイルの代わりにJSON形式のログファイル(.jsonまたは.jsonl)をスキャンする
  -w, --no-wizard                        質問はしない。すべてのイベントとアラートをスキャンする
  -Q, --quiet-errors                     Quiet errorsモード: エラーログを保存しない
  -x, --recover-records                  空ページからevtxレコードをカービングする (デフォルト: 無効)
  -r, --rules <DIR/FILE>                 ルールファイルまたはルールファイルを持つディレクトリ (デフォルト: ./rules)
  -c, --rules-config <DIR>               ルールフォルダのコンフィグディレクトリ (デフォルト: ./rules/config)
  -s, --sort                             ファイル保存前イベントをソートする (警告: これは多くのメモリを使用する!)
  -t, --threads <NUMBER>                 スレッド数 (デフォルト: パフォーマンスに最適な数値)
      --target-file-ext <FILE-EXT...>    evtx以外の拡張子を解析対象に追加する。 (例１: evtx_data 例２: evtx1,evtx2)

Filtering:
  -E, --EID-filter                      速度を上げるため主なEIDだけスキャンする (コンフィグファイル: ./rules/config/target_event_IDs.txt)
  -A, --enable-all-rules                ロードされたevtxファイルに関係なく、すべてのルールを有効にする（ルールのチャネルフィルターを無効にする）
  -D, --enable-deprecated-rules         ステータスがdeprecatedのルールを有効にする
  -n, --enable-noisy-rules              Noisyルールを有効にする
  -u, --enable-unsupported-rules        ステータスがunsupportedのルールを有効にする
  -e, --exact-level <LEVEL>             特定のレベルだけスキャンする (informational, low, medium, high, critical)
      --exclude-category <CATEGORY...>  特定のlogsourceカテゴリを持つルールをロードしない (例: process_creation,pipe_created)
      --exclude-computer <COMPUTER...>  特定のコンピュータ名をスキャンしない (例: ComputerA) (例: ComputerA,ComputerB)
      --exclude-eid <EID...>            高速化のために特定のEIDをスキャンしない (例: 1) (例: 1,4688)
      --exclude-status <STATUS...>      読み込み対象外とするルール内でのステータス (例１: experimental) (例２: stable,test)
      --exclude-tag <TAG...>            特定のタグを持つルールをロードしない (例: sysmon)
      --include-category <CATEGORY...>  特定のlogsourceカテゴリを持つルールのみをロードする (例: process_creation,pipe_created)
      --include-computer <COMPUTER...>  特定のコンピュータ名のみをスキャンする (例: ComputerA) (例: ComputerA,ComputerB)
      --include-eid <EID...>            指定したEIDのみをスキャンして高速化する (例: 1) (例: 1,4688)
      --include-status <STATUS...>      特定のステータスを持つルールのみをロードする (例: expermimental) (例: stable,test)
      --include-tag <TAG...>            特定のタグを持つルールのみをロードする (例１: attack.execution,attack.discovery) (例２: wmi)
  -m, --min-level <LEVEL>               結果出力をするルールの最低レベル (デフォルト: informational)
  -P, --proven-rules                    実績のあるルールだけでスキャンし、高速化する (./rules/config/proven_rules.txt)
  -a, --scan-all-evtx-files             ロードされたルールに関係なく、すべてのevtxファイルをスキャンする（evtxファイルのチャネルフィルターを無効にする）
      --time-offset <OFFSET>            オフセットに基づく最近のイベントのスキャン (例: 1y, 3M, 30d, 24h, 30m)
      --timeline-end <DATE>             解析対象とするイベントログの終了時刻 (例: "2022-02-22 23:59:59 +09:00")
      --timeline-start <DATE>           解析対象とするイベントログの開始時刻 (例: "2020-02-22 00:00:00 +09:00")

Output:
  -b, --disable-abbreviations        省略機能を無効にする
  -G, --GeoIP <MAXMIND-DB-DIR>       IPアドレスのGeoIP(ASN、都市、国)情報を追加する
  -H, --HTML-report <FILE>           HTML形式で詳細な結果を出力する (例: results.html)
  -L, --JSONL-output                 タイムラインをJSONL形式で保存する (例: -L -o results.jsonl)
  -F, --no-field-data-mapping        フィールドデータのマッピングを無効にする
      --no-pwsh-field-extraction     PowerShell Classicログフィールド抽出の無効化
  -o, --output <FILE>                タイムラインを保存する (例: results.csv)
  -p, --profile <PROFILE>            利用する出力プロファイル名を指定する
  -R, --remove-duplicate-data        重複したフィールドデータは「DUP」に置き換えられる (ファイルサイズが約10〜15％削減される)
  -X, --remove-duplicate-detections  重複した検知項目を削除する (デフォルト: 無効)

Display Settings:
  -K, --no-color            カラーで出力しない
  -N, --no-summary          結果概要を出力しない (多少速くなる)
  -q, --quiet               Quietモード: 起動バナーを表示しない
  -v, --verbose             詳細な情報を出力する
  -T, --visualize-timeline  検知頻度タイムラインを出力する（ターミナルはUnicodeに対応する必要がある）

Time Format:
      --European-time     ヨーロッパ形式で日付と時刻を出力する (例: 22-02-2022 22:00:00.123 +02:00)
  -O, --ISO-8601          ISO-8601形式で日付と時刻を出力する (例: 2022-02-22T10:10:10.1234567Z) (UTC時刻)
      --RFC-2822          RFC 2822形式で日付と時刻を出力する (例: Fri, 22 Feb 2022 22:00:00 -0600)
      --RFC-3339          RFC 3339形式で日付と時刻を出力する (例: 2022-02-22 22:00:00.123456-06:00)
      --US-military-time  24時間制(ミリタリータイム)のアメリカ形式で日付と時刻を出力する (例: 02-22-2022 22:00:00.123 -06:00)
      --US-time           アメリカ形式で日付と時刻を出力する (例: 02-22-2022 10:00:00.123 PM -06:00)
  -U, --UTC               UTC形式で日付と時刻を出力する (デフォルト: 現地時間)
```

#### `json-timeline`コマンドの使用例と設定ファイル

`json-timeline`のオプションと設定ファイルは、`csv-timeline`と同じですが、JSONL形式で出力するための`-L, --JSONL-output`オプションが1つ追加されています。

### `level-tuning`コマンド

`level-tuning`コマンドを使用すると、環境に応じてリスクレベルを上げたり下げたりして、ルールのアラートレベルを調整できます。
このコマンドは、`rules`フォルダ内のルールのリスクレベル(`level`フィールド)を上書きするために、設定ファイルを使用します。

> 注意: `update-rules`を実行するたびに、アラートレベルが元の設定に上書きされるので、レベルを変更したい場合は、`level-tuning`コマンドも実行する必要があります。

```
Usage: level-tuning [OPTIONS]

Display Settings:
  -K, --no-color      カラーで出力しない
  -q, --quiet         Quietモード: 起動バナーを表示しない

General Options:
  -f, --file <FILE>   ルールlevelのチューニング (デフォルト: ./rules/config/level_tuning.txt)
  -h, --help          ヘルプメニューを表示する
```

#### `level-tuning`コマンドの使用例

* 通常使用: `hayabusa.exe level-tuning`
* カスタム設定ファイルに基づくルールのアラートレベルの調整: `hayabusa.exe level-tuning -f ./config/level_tuning.txt`

#### `level-tuning`の設定ファイル

HayabuaとSigmaのルール作成者は、ルールを作成する際にアラートの適切なリスクレベルを見積もります。
しかし、リスクレベルが一貫していない場合や、実際のリスクレベルが環境によって異なる場合があります。
Yamato Securityは、`./rules/config/level_tuning.txt`に設定ファイルを提供し、ルールを調整することができます。

`./rules/config/level_tuning.txt`の一例:

```csv
id,new_level
570ae5ec-33dc-427c-b815-db86228ad43e,informational # 'Application Uninstalled' - Originally low.
b6ce0b2f-593b-5e1c-e137-d30b2974e30e,high # 'Suspicious Double Extension File Execution' - Sysmon 1 - Originally critical
452b2159-5e6e-c494-63b9-b385d6195f58,high # 'Suspicious Double Extension File Execution' - Security 4688 - Originally critical
51ba8477-86a4-6ff0-35fa-7b7f1b1e3f83,high # 'CobaltStrike Service Installations - System' - System 7045 - Originally critical
daad2203-665f-294c-6d2f-f9272c3214f2,critical # 'Mimikatz DC Sync' - Security 4662 - Originally high
8b061ac2-31c7-659d-aa1b-36ceed1b03f1,high # 'HackTool - Rubeus Execution' - Sysmon 1 - Originally critical
be670d5c-31eb-7391-4d2e-d122c89cd5bb,high # 'HackTool - Rubeus Execution' - Security 4688 - Originally critical
```

この場合、ルールディレクトリ内の`id`が`570ae5ec-33dc-427c-b815-db86228ad43e`のルールのリスクレベルは、`informational`に書き換えられます。
設定可能なレベルは、`critical`、`high`、`medium`、`low`、`informational`です。

> 注意: `./rules/config/level_tuning.txt`設定ファイルは、`update-rules`を実行するたびに、hayabusa-rulesリポジトリの最新バージョンに更新されます。
> したがって、このファイルを変更した場合、変更内容は失われます！
> 自分用の設定ファイルを保持したい場合は、`./config/level_tuning.txt`に設定ファイルを作成し、`hayabusa.exe level-tuning -f ./config/level_tuning.txt`を実行してください。
> また、Yamato Securityが提供する設定ファイルを使用して最初にレベル調整を行い、その後独自の設定ファイルでさらに調整することもできます。

### `list-profiles`コマンド

```
Usage: list-profiles [OPTIONS]

Display Settings:
  -K, --no-color   カラーで出力しない
  -q, --quiet      Quietモード: 起動バナーを表示しない
  
General Options:
  -h, --help       ヘルプメニューを表示する
```

### `set-default-profile`コマンド

```
Usage: set-default-profile [OPTIONS]

Display Settings:
  -K, --no-color           カラーで出力しない
  -q, --quiet              Quietモード: 起動バナーを表示しない

General Options:
  -h, --help               ヘルプメニューを表示する
  -p, --profile <PROFILE>  利用する出力プロファイル名を指定する
```

#### `set-default-profile`コマンドの使用例

* デフォルトプロファイルを`minimal`に設定する: `hayabusa.exe set-default-profile minimal`
* デフォルトプロファイルを`super-verbose`に設定する: `hayabusa.exe set-default-profile super-verbose`

## `update-rules`コマンド

`update-rules`コマンドは、`rules`フォルダを[HayabusaルールのGitHubリポジトリ](https://github.com/Yamato-Security/hayabusa-rules)と同期し、ルールと設定ファイルを更新します。

```
Usage: update-rules [OPTIONS]

Display Settings:
  -K, --no-color  カラーで出力しない
  -q, --quiet     Quietモード: 起動バナーを表示しない

General Options:
  -h, --help              ヘルプメニューを表示する
  -r, --rules <DIR/FILE>  ルールファイルまたはルールファイルを持つディレクトリ (デフォルト: ./rules)
```

### `update-rules`コマンドの使用例

普段は次のように実行します: `hayabusa.exe update-rules`

# タイムライン出力

## 出力プロファイル

Hayabusaの`config/profiles.yaml`設定ファイルでは、５つのプロファイルが定義されています:

1. `minimal`
2. `standard` (デフォルト)
3. `verbose`
4. `all-field-info`
5. `all-field-info-verbose`
6. `super-verbose`
7. `timesketch-minimal`
8. `timesketch-verbose`

このファイルを編集することで、簡単に独自のプロファイルをカスタマイズしたり、追加したりすることができます。
`set-default-profile --profile <profile>`コマンドでデフォルトのプロファイルを変更することもできます。
利用可能なプロファイルとそのフィールド情報を表示するには、`list-profiles`コマンドを使用してください。

### 1. `minimal`プロファイルの出力

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %RecordID%, %RuleTitle%, %Details%`

### 2. `standard`プロファイルの出力

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %RecordID%, %RuleTitle%, %Details%, %ExtraFieldInfo%`, %RuleID%

### 3. `verbose`プロファイルの出力

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %MitreTactics%, %MitreTags%, %OtherTags%, %RecordID%, %RuleTitle%, %Details%, %ExtraFieldInfo%, %RuleFile%, %RuleID%, %EvtxFile%`

### 4. `all-field-info`プロファイルの出力

最小限の`details`情報を出力する代わりに、イベントにあるすべての`EventData`フィールド情報(`%AllFieldInfo%`)が出力されます。フィールド名は元々のフィールド名になります。

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %RecordID%, %RuleTitle%, %AllFieldInfo%, %RuleFile%, %RuleID%, %EvtxFile%`

### 5. `all-field-info-verbose`プロファイルの出力

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %MitreTactics%, %MitreTags%, %OtherTags%, %RecordID%, %RuleTitle%, %AllFieldInfo%, %RuleFile%, %RuleID%, %EvtxFile%`

### 6. `super-verbose`プロファイルの出力

`%Timestamp%, %Computer%, %Channel%, %EventID%, %Level%, %RuleTitle%, %RuleAuthor%, %RuleModifiedDate%, %Status%, %RecordID%, %Details%, %ExtraFieldInfo%, %MitreTactics%, %MitreTags%, %OtherTags%, %Provider%, %RuleCreationDate%, %RuleFile%, %RuleID%, %EvtxFile%`

### 7. `timesketch-minimal`プロファイルの出力

[Timesketch](https://timesketch.org/)にインポートできるプロファイル。

`%Timestamp%, hayabusa, %RuleTitle%, %Computer%, %Channel%, %EventID%, %Level%, %MitreTactics%, %MitreTags%, %OtherTags%, %RecordID%, %Details%, %RuleFile%, %RuleID%, %EvtxFile%`

### 8. `timesketch-verbose`プロファイルの出力

`%Timestamp%, hayabusa, %RuleTitle%, %Computer%, %Channel%, %EventID%, %Level%, %MitreTactics%, %MitreTags%, %OtherTags%, %RecordID%, %Details%, %ExtraFieldInfo%, %RuleFile%, %RuleID%, %EvtxFile%`

### プロファイルの比較

以下のベンチマークは、2018年製のLenovo P51 (CPU: Xeon 4コア / メモリ: 64GB)上で3GBのEVTXデータに対して3891件のルールを有効にして実施されました。(2023/06/01)

| プロファイル | 処理時間 | 結果のファイルサイズ | ファイルサイズ増加 |
| :---: | :---: | :---: | :---: |
| minimal | 8分50秒 | 770 MB | -30% |
| standard (デフォルト) | 9分00秒 | 1.1 GB | 無し |
| verbose | 9分10秒 | 1.3 GB | +20% |
| all-field-info | 9分3秒 | 1.2 GB | +10% |
| all-field-info-verbose | 9分10秒 | 1.3 GB | +20% |
| super-verbose | 9分12秒 | 1.5 GB | +35% |

### プロファイルのフィールドエイリアス

ビルトインの出力プロファイルで出力できる情報は以下の通り:

| エイリアス名 | Hayabusaの出力情報 |
| :--- | :--- |
|%AllFieldInfo% | すべてのフィールド情報。 |
|%Channel% |  ログ名。イベントログの`<Event><System><EventID>`フィールド。 |
|%Computer% | イベントログの`<Event><System><Computer>`フィールド。 |
|%Details% | YML検知ルールの`details`フィールドから来ていますが、このフィールドはHayabusaルールにしかありません。このフィールドはアラートとイベントに関する追加情報を提供し、ログのフィールドから有用なデータを抽出することができます。イベントキーのマッピングが間違っている場合、もしくはフィールドが存在しない場合で抽出ができなかった箇所は`n/a` (not available)と記載されます。YML検知ルールに`details`フィールドが存在しない時のdetailsのメッセージを`./rules/config/default_details.txt`で設定できます。`default_details.txt`では`Provider Name`、`EventID`、`details`の組み合わせで設定することができます。default_details.txt`やYML検知ルールに対応するルールが記載されていない場合はすべてのフィールド情報を出力します。 |
|%ExtraFieldInfo% | %Details%で出力されなかったフィールドデータを出力する。 |
|%EventID% | イベントログの`<Event><System><EventID>`フィールド。 |
|%EvtxFile% | アラートまたはイベントを起こしたevtxファイルへのパス。 |
|%Level% | YML検知ルールの`level`フィールド。(例：`informational`、`low`、`medium`、`high`、`critical`) |
|%MitreTactics% | MITRE ATT&CKの[戦術](https://attack.mitre.org/tactics/enterprise/) (例: Initial Access、Lateral Movement等々） |
|%MitreTags% | MITRE ATT&CKの戦術以外の情報。attack.g(グループ)、attack.t(技術)、attack.s(ソフトウェア)の情報を出力する。 |
|%OtherTags% | YML検知ルールの`tags`フィールドから`MitreTactics`、`MitreTags`以外のキーワードを出力する。|
|%Provider% | `<Event><System><Provider>` フィールド内の`Name`属性。 |
|%RecordID% | `<Event><System><EventRecordID>`フィールドのイベントレコードID。 |
|%RuleAuthor% | YML検知ルールの `author` フィールド。 |
|%RuleCreationDate% | YML検知ルールの `date` フィールド。 |
|%RuleFile% | アラートまたはイベントを生成した検知ルールのファイル名。 |
|%RuleModifiedDate% | YML検知ルールの `modified` フィールド。 |
|%RuleTitle% | YML検知ルールの`title`フィールド。 |
|%Status% | YML検知ルールの `status` フィールド。 |
|%Timestamp% | デフォルトでは`YYYY-MM-DD HH:mm:ss.sss +hh:mm`形式になっている。イベントログの`<Event><System><TimeCreated SystemTime>`フィールドから来ている。デフォルトのタイムゾーンはローカルのタイムゾーンになるが、`--UTC`オプションでUTCに変更することができる。 |

#### その他のプロファイルのフィールドエイリアス

必要であれば、これらのエイリアスを出力プロファイルに追加することもできます:

| エイリアス名 | Hayabusaの出力情報 |
| :--- | :--- |
|%RenderedMessage% | WEC機能で転送されたイベントログの`<Event><RenderingInfo><Message>`フィールド。 |
|%RuleID% | YML検知ルールの`id`フィールド。 |

注意: これらはビルトインプロファイルには**含まれていない**ので、手動で`config/default_profile.yaml`ファイルを編集し、以下の行を追加する必要があります:

```
Message: "%RenderedMessage%"
```

また、[イベントキーエイリアス](https://github.com/Yamato-Security/hayabusa-rules/blob/main/README-Japanese.md#%E3%82%A4%E3%83%99%E3%83%B3%E3%83%88%E3%82%AD%E3%83%BC%E3%82%A8%E3%82%A4%E3%83%AA%E3%82%A2%E3%82%B9)を定義し、出力することもできます。

## 省略

結果をミニマルにするため、レベル、MITRE ATT&CK戦術、チャンネル、プロバイダ、フィールド名などを省略しています。

`b, --disable-abbreviations`オプションで、これらの省略のいくつかを無効にして、元々のチャンネル名、プロバイダ名などを表示することができます。

### Levelの省略

簡潔に出力するために`level`を以下のように省略し出力しています。

* `crit`: `critical`
* `high`: `high`
* `med `: `medium`
* `low `: `low`
* `info`: `informational`

### MITRE ATT&CK戦術の省略

簡潔に出力するためにMITRE ATT&CKの戦術を以下のように省略しています。
`./config/mitre_tactics.txt`の設定ファイルで自由に編集できます。

* `Recon` : Reconnaissance (偵察)
* `ResDev` : Resource Development (リソース開発)
* `InitAccess` : Initial Access (初期アクセス)
* `Exec` : Execution (実行)
* `Persis` : Persistence (永続化)
* `PrivEsc` : Privilege Escalation (権限昇格)
* `Evas` : Defense Evasion (防御回避)
* `CredAccess` : Credential Access (認証情報アクセス)
* `Disc` : Discovery (探索)
* `LatMov` : Lateral Movement (横展開)
* `Collect` : Collection (収集)
* `C2` : Command and Control (遠隔操作)
* `Exfil` : Exfiltration (持ち出し)
* `Impact` : Impact (影響)

### Channel情報の省略

簡潔に出力するためにChannelの表示を以下のように省略しています。
`./rules/config/channel_abbreviations.txt`の設定ファイルで自由に編集できます。

* `App` : `Application`
* `AppLocker` : `Microsoft-Windows-AppLocker/*`
* `BitsCli` : `Microsoft-Windows-Bits-Client/Operational`
* `CodeInteg` : `Microsoft-Windows-CodeIntegrity/Operational`
* `Defender` : `Microsoft-Windows-Windows Defender/Operational`
* `DHCP-Svr` : `Microsoft-Windows-DHCP-Server/Operational`
* `DNS-Svr` : `DNS Server`
* `DvrFmwk` : `Microsoft-Windows-DriverFrameworks-UserMode/Operational`
* `Exchange` : `MSExchange Management`
* `Firewall` : `Microsoft-Windows-Windows Firewall With Advanced Security/Firewall`
* `KeyMgtSvc` : `Key Management Service`
* `LDAP-Cli` : `Microsoft-Windows-LDAP-Client/Debug`
* `NTLM` `Microsoft-Windows-NTLM/Operational`
* `OpenSSH` : `OpenSSH/Operational`
* `PrintAdm` : `Microsoft-Windows-PrintService/Admin`
* `PrintOp` : `Microsoft-Windows-PrintService/Operational`
* `PwSh` : `Microsoft-Windows-PowerShell/Operational`
* `PwShClassic` : `Windows PowerShell`
* `RDP-Client` : `Microsoft-Windows-TerminalServices-RDPClient/Operational`
* `Sec` : `Security`
* `SecMitig` : `Microsoft-Windows-Security-Mitigations/*`
* `SmbCliSec` : `Microsoft-Windows-SmbClient/Security`
* `SvcBusCli` : `Microsoft-ServiceBus-Client`
* `Sys` : `System`
* `Sysmon` : `Microsoft-Windows-Sysmon/Operational`
* `TaskSch` : `Microsoft-Windows-TaskScheduler/Operational`
* `WinRM` : `Microsoft-Windows-WinRM/Operational`
* `WMI` : `Microsoft-Windows-WMI-Activity/Operational`

### その他の省略

できるだけ簡潔にするために、以下の略語を使用しています:

* `Acct` -> Account
* `Addr` -> Address
* `Auth` -> Authentication
* `Cli` -> Client
* `Chan` -> Channel
* `Cmd` -> Command
* `Cnt` -> Count
* `Comp` -> Computer
* `Conn` -> Connection/Connected
* `Creds` -> Credentials
* `Crit` -> Critical
* `Disconn` -> Disconnection/Disconnected
* `Dir` -> Directory
* `Drv` -> Driver
* `Dst` -> Destination
* `EID` -> Event ID
* `Err` -> Error
* `Exec` -> Execution
* `FW` -> Firewall
* `Grp` -> Group
* `Img` -> Image
* `Inj` -> Injection
* `Krb` -> Kerberos
* `LID` -> Logon ID
* `Med` -> Medium
* `Net` -> Network
* `Obj` -> Object
* `Op` -> Operational/Operation
* `Proto` -> Protocol
* `PW` -> Password
* `Reconn` -> Reconnection
* `Req` -> Request
* `Rsp` -> Response
* `Sess` -> Session
* `Sig` -> Signature
* `Susp` -> Suspicious
* `Src` -> Source
* `Svc` -> Service
* `Svr` -> Server
* `Temp` -> Temporary
* `Term` -> Termination/Terminated
* `Tkt` -> Ticket
* `Tgt` -> Target
* `Unkwn` -> Unknown
* `Usr` -> User
* `Perm` -> Permament
* `Pkg` -> Package
* `Priv` -> Privilege
* `Proc` -> Process
* `PID` -> Process ID
* `PGUID` -> Process GUID (Global Unique ID)
* `Ver` -> Version

## プログレスバー

プログレス・バーは、複数のevtxファイルに対してのみ機能します。
解析したevtxファイルの数と割合をリアルタイムで表示します。

## カラー出力

Hayabusaの結果は`level`毎に文字色が変わります。
`./config/level_color.txt`の値を変更することで文字色を変えることができます。形式は`level名,(6桁のRGBのカラーhex)`です。
カラー出力をしないようにしたい場合は`--no-color`オプションをご利用ください。

## 結果のサマリ (Results Summary)

元々のイベント数、検知したイベント数、データ削減の統計、検知数情報、最多検知日、最多検知端末名、最多アラート等の情報がスキャン後に出力されます。

### 検知頻度タイムライン

`-T, --visualize-timeline`オプションを使うことで、検知したイベントの数が5以上の時、頻度のタイムライン(スパークライン)を画面に出力します。
マーカーの数は最大10個です。デフォルトのCommand PromptとPowerShell Promptでは文字化けがでるので、Windows TerminalやiTerm2等のターミナルをご利用ください。

# Hayabusaルール

Hayabusa検知ルールはSigmaのようなYML形式で記述され、`rules`ディレクトリに入っています。
[https://github.com/Yamato-Security/hayabusa-rules](https://github.com/Yamato-Security/hayabusa-rules)のレポジトリで管理しているので、ルールのissueやpull requestはhayabusaのレポジトリではなく、ルールレポジトリへお願いします。

ルールの作成方法については、[hayabusa-rulesレポジトリのREADME](https://github.com/Yamato-Security/hayabusa-rules/blob/main/README-Japanese.md) をお読みください。

[hayabusa-rulesレポジトリ](https://github.com/Yamato-Security/hayabusa-rules)にあるすべてのルールは、`rules`フォルダに配置する必要があります。
`level`がinformationのルールは`イベント`とみなされ、`low`以上は`アラート`とみなされます。

Hayabusaルールのディレクトリ構造は、2つのディレクトリに分かれています:

* `builtin`: Windowsの組み込み機能で生成できるログ。
* `sysmon`: [sysmon](https://docs.microsoft.com/en-us/sysinternals/downloads/sysmon)によって生成されるログ。

ルールはさらにログタイプ（例：Security、Systemなど）によってディレクトリに分けられ、次の形式で名前が付けられます。

現在のルールをご確認いただき、新規作成時のテンプレートとして、また検知ロジックの確認用としてご利用ください。

## Sigma v.s. Hayabusa(ビルトインSigmaとの互換性のある)ルール

Hayabusaは、`logsource`フィールドを内部で処理することを唯一の例外として、Sigmaルールをネイティブにサポートしています。
過検知を減らすため、コンバータで変換した方が良いです。変換のやり方は[ここ](https://github.com/Yamato-Security/hayabusa-rules/tree/main/tools/sigmac/README-Japanese.md)で説明されています。
これにより、適切な`Channel`と`EventID`が追加され、`process_creation`のような特定のカテゴリに対してフィールドマッピングが行われます。

殆どのルールはSigmaルールと互換性があるので、Sigmaルールのようにその他のSIEM形式に変換できます。
Hayabusaルールは、Windowsのイベントログ解析専用に設計されており、以下のような利点があります:

1. ログの有用なフィールドのみから抽出された追加情報を表示するための`details`フィールドを追加しています。
2. Hayabusaルールはすべてサンプルログに対してテストされ、検知することが確認されています。
3. Sigmaルール仕様にない集計式(例：`|equalsfield`、`|endswithfield`)の利用。

私たちの知る限り、HayabusaはオープンソースのWindowsイベントログ解析ツールの中でSigmaルールを最も多くサポートしています。

# その他のWindowsイベントログ解析ツールおよび関連リソース

* [APT-Hunter](https://github.com/ahmedkhlief/APT-Hunter) - Pythonで開発された攻撃検知ツール。
* [Awesome Event IDs](https://github.com/stuhli/awesome-event-ids) -  フォレンジック調査とインシデント対応に役立つイベントIDのリソース。
* [Chainsaw](https://github.com/countercept/chainsaw) - Rustで開発されたSigmaベースの攻撃検知ツール。
* [DeepBlueCLI](https://github.com/sans-blue-team/DeepBlueCLI) - [Eric Conrad](https://twitter.com/eric_conrad) によってPowershellで開発された攻撃検知ツール。
* [Epagneul](https://github.com/jurelou/epagneul) - Windowsイベントログの可視化ツール。
* [EventList](https://github.com/miriamxyra/EventList/) - [Miriam Wiesner](https://github.com/miriamxyra)によるセキュリティベースラインの有効なイベントIDをMITRE ATT&CKにマッピングするPowerShellツール。
* [MITRE ATT&CKとWindowイベントログIDのマッピング](https://www.socinvestigation.com/mapping-mitre-attck-with-window-event-log-ids/) - 作者：[Michel de CREVOISIER](https://twitter.com/mdecrevoisier)
* [EvtxECmd](https://github.com/EricZimmerman/evtx) - [Eric Zimmerman](https://twitter.com/ericrzimmerman)によるEvtxパーサー。
* [EVTXtract](https://github.com/williballenthin/EVTXtract) - 未使用領域やメモリダンプからEVTXファイルを復元するツール。
* [EvtxToElk](https://www.dragos.com/blog/industry-news/evtxtoelk-a-python-module-to-load-windows-event-logs-into-elasticsearch/) - Elastic StackにEvtxデータを送信するPythonツール。
* [EVTX ATTACK Samples](https://github.com/sbousseaden/EVTX-ATTACK-SAMPLES) - [SBousseaden](https://twitter.com/SBousseaden) によるEVTX攻撃サンプルイベントログファイル。
* [EVTX-to-MITRE-Attack](https://github.com/mdecrevoisier/EVTX-to-MITRE-Attack) - [Michel de CREVOISIER](https://twitter.com/mdecrevoisier)によるATT&CKにマッピングされたEVTX攻撃サンプルログのレポジトリ。
* [EVTX parser](https://github.com/omerbenamram/evtx) - [@OBenamram](https://twitter.com/obenamram) によって書かれた、Hayabusaが使用しているRustライブラリ。
* [Grafiki](https://github.com/lucky-luk3/Grafiki) - SysmonとPowerShellログの可視化ツール。
* [LogonTracer](https://github.com/JPCERTCC/LogonTracer) - [JPCERTCC](https://twitter.com/jpcert) による、横方向の動きを検知するためにログオンを視覚化するグラフィカルなインターフェース。
* [NSA Windows Event Monitoring Guidance](https://github.com/nsacyber/Event-Forwarding-Guidance/tree/master/Events) - NSAのWindowsイベントログ監視ガイド。
* [RustyBlue](https://github.com/Yamato-Security/RustyBlue) - 大和セキュリティによるDeepBlueCLIのRust版。
* [Sigma](https://github.com/SigmaHQ/Sigma) - コミュニティベースの汎用SIEMルール。
* [SOF-ELK](https://github.com/philhagen/sof-elk) - [Phil Hagen](https://twitter.com/philhagen) によるDFIR解析用のElastic Stack VM。
* [so-import-evtx](https://docs.securityonion.net/en/2.3/so-import-evtx.html) - evtxファイルをSecurityOnionにインポートするツール。
* [SysmonTools](https://github.com/nshalabi/SysmonTools) - Sysmonの設定とオフライン可視化ツール。
* [Timeline Explorer](https://ericzimmerman.github.io/#!index.md) - [Eric Zimmerman](https://twitter.com/ericrzimmerman) による最高のCSVタイムラインアナライザ。
* [Windows Event Log Analysis - Analyst Reference](https://www.forwarddefense.com/media/attachments/2021/05/15/windows-event-log-analyst-reference.pdf) - Forward DefenseのSteve AnsonによるWindowsイベントログ解析の参考資料。
* [Zircolite](https://github.com/wagga40/Zircolite) - Pythonで書かれたSigmaベースの攻撃検知ツール。

# Windowsイベントログ設定のススメ

Windows機での悪性な活動を検知する為には、デフォルトのログ設定を改善することが必要です。
どのようなログ設定を有効にする必要があるのか、また、自動的に適切な設定を有効にするためのスクリプトを、別のプロジェクトとして作成しました: [https://github.com/Yamato-Security/EnableWindowsLogSettings](https://github.com/Yamato-Security/EnableWindowsLogSettings)

以下のサイトを閲覧することもおすすめします。:

* [JSCU-NL (Joint Sigint Cyber Unit Netherlands) Logging Essentials](https://github.com/JSCU-NL/logging-essentials)
* [ACSC (Australian Cyber Security Centre) Logging and Fowarding Guide](https://www.cyber.gov.au/acsc/view-all-content/publications/windows-event-logging-and-forwarding)
* [Malware Archaeology Cheat Sheets](https://www.malwarearchaeology.com/cheat-sheets)

# Sysmon関係のプロジェクト

フォレンジックに有用な証拠を作り、高い精度で検知をさせるためには、sysmonをインストールする必要があります。以下のサイトを参考に設定することをおすすめします。:

* [Sysmon Modular](https://github.com/olafhartong/sysmon-modular)
* [TrustedSec Sysmon Community Guide](https://github.com/trustedsec/SysmonCommunityGuide)
* [SwiftOnSecurityのSysmon設定ファイル](https://github.com/SwiftOnSecurity/sysmon-config)
* [Neo23x0によるSwiftOnSecurityのSysmon設定ファイルのフォーク](https://github.com/Neo23x0/sysmon-config)
* [ion-stormによるSwiftOnSecurityのSysmon設定ファイルのフォーク](https://github.com/ion-storm/sysmon-config)

# コミュニティによるドキュメンテーション

## 英語

* 2023/12/11 Christian Henriksen氏による[Unleashing the Hayabusa Feathers: My Top Features Revealed!](https://detect.fyi/hunting-with-hayabusa-tool-showcase-aafef7434413)
* 2023/10/16 Md. Mahim Bin Firoj氏による[Incident response and threat hunting using hayabusa tool](https://mahim-firoj.medium.com/incident-response-and-threat-hunting-using-hayabusa-tool-383da273183a)
* 2023/03/21 [Eric Capuano](https://twitter.com/eric_capuano)氏による[Find Threats in Event Logs with Hayabusa](https://blog.ecapuano.com/p/find-threats-in-event-logs-with-hayabusa)
* 2023/03/14 Fukusuke Takahashi氏による[Hayabusa開発者向けRustパフォーマンスガイド](doc/RustPerformance-English.md)
* 2022/06/19 [Eric Capuano](https://twitter.com/eric_capuano)氏による[VelociraptorチュートリアルとHayabusaの統合方法](https://www.youtube.com/watch?v=Q1IoGX--814)
* 2022/01/24 Matthew Seyer ([@forensic_matt](https://twitter.com/forensic_matt))氏による[Hayabusa結果をneo4jで可視化する方法](https://www.youtube.com/watch?v=7sQqz2ek-ko)

## 日本語

* 2024/01/24 NECセキュリティブログ: [LME × Hayabusa　－　Windowsイベントログの集約と解析の効率化](https://jpn.nec.com/cybersecurity/blog/240126/index.html)
* 2023/09/29 NECセキュリティブログ: [HayabusaとSplunkによるファストフォレンジック効率化](https://jpn.nec.com/cybersecurity/blog/230929/index.html)
* 2023/09/13 FFRIセキュリティブログ: [HayabusaによるWindowsイベントログ解析](https://engineers.ffri.jp/entry/2023/09/13/130750)
* 2023/03/14 Fukusuke Takahashi氏による[Hayabusa開発者向けRustパフォーマンスガイド](doc/RustPerformance-Japanese.md)
* 2022/01/22 [@kzzzzo2](https://qiita.com/kzzzzo2)氏による[Hayabusa結果をElastic Stackで可視化する方法](https://qiita.com/kzzzzo2/items/ead8ccc77b7609143749)
* 2021/12/31 itiB ([@itiB_S144](https://twitter.com/itiB_S144))氏による[Windowsイベントログ解析ツール「Hayabusa」を使ってみる](https://itib.hatenablog.com/entry/2021/12/31/222946)
* 2021/12/27 Kazuminn ([@k47_um1n](https://twitter.com/k47_um1n))氏による[Hayabusaの中身](https://kazuminkun.hatenablog.com/entry/2021/12/27/190535)

# 貢献

どのような形でも構いませんので、ご協力をお願いします。
プルリクエスト、ルール作成、evtxログのサンプルなどがベストですが、機能リクエスト、バグの通知なども大歓迎です。

少なくとも、私たちのツールを気に入っていただけたなら、GitHubで星を付けて、あなたのサポートを表明してください。

# バグの報告

見つけたバグを[こちら](https://github.com/Yamato-Security/hayabusa/issues/new?assignees=&labels=bug&template=bug_report.md&title=%5Bbug%5D)でご連絡ください。
報告されたバグを喜んで修正します！

Hayabusaルールの問題点（誤検出、バグ等々）を発見された方は、hayabusa-rulesのGitHubの[Issues](https://github.com/Yamato-Security/hayabusa-rules/issues/new)ページにご報告ください。

Sigmaルールの問題点（誤検出、バグ等々）を発見された方は、上流のSigmaHQ GitHubの[Issues](https://github.com/SigmaHQ/sigma/issues)ページにご報告ください。

# ライセンス

Hayabusaは[AGPLv3](https://gpl.mhatta.org/agpl.ja.html)で公開され、すべてのルールは[Detection Rule License (DRL) 1.1](https://github.com/SigmaHQ/sigma/blob/master/LICENSE.Detection.Rules.md)で公開されています。
Hayabusaの社内利用、SaaSソリューションの利用、コンサルティングの利用などは自由です。
ただし、SaaS型ソリューションでHayabusaを利用し、改良を加えた場合は、その改良をオープンソース化し、プロジェクトに還元してください。

Hayabusaは、MaxMind社が作成したGeoLite2データを使用しており、[https://www.maxmind.com](https://www.maxmind.com)から入手可能です。

# Twitter

[@SecurityYamato](https://twitter.com/SecurityYamato)でHayabusa、ルール更新、その他の大和セキュリティツール等々について情報を提供しています。