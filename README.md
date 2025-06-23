# geojson2kml

日本の鉄道GeoJSONデータをKML形式に変換し、Google Earthなどの地図アプリケーションで可視化するためのRustコマンドラインツールです。

A Rust command-line tool to convert Japanese railway GeoJSON data to KML format for visualization in Google Earth and other mapping applications.

## 機能 / Features

- 鉄道会社名または路線名によるインタラクティブ検索 / Interactive search by railway company name or line name
- 部分文字列マッチングのサポート / Support for partial string matching
- カンマ区切りによる複数路線の選択 / Multiple line selection with comma-separated input
- 複数路線を1つのKMLファイルに統合するオプション / Option to merge multiple lines into a single KML file
- 選択した各路線の個別KMLファイル生成 / Individual KML file generation for each selected line

## 必要環境 / Requirements

- Rust 1.70以上 / Rust 1.70 or higher
- 日本の鉄道区間GeoJSONデータファイル / Japanese railway section GeoJSON data file
  - [国土数値情報ダウンロードサイト](https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-N02-v3_0.html)からダウンロード可能 / Available from [National Land Numerical Information Download Site](https://nlftp.mlit.go.jp/ksj/gml/datalist/KsjTmplt-N02-v3_0.html)

## インストール / Installation

```bash
git clone https://github.com/keg66/geojson2kml.git
cd geojson2kml
cargo build --release
```

## 使用方法 / Usage

1. GeoJSONファイルを引数として指定してアプリケーションを実行: / Run the application with a GeoJSON file as argument:

```bash
cargo run <geojson_file>
# 例 / Example:
cargo run N02-20_RailroadSection.geojson
```

2. インタラクティブなプロンプトに従って操作: / Follow the interactive prompts:
   - 鉄道会社名または路線名を入力（例: "山手線", "東日本旅客鉄道"） / Enter a railway company name or line name (e.g., "山手線", "東日本旅客鉄道")
   - 複数の候補が見つかった場合、番号で選択 - カンマ区切りで複数選択可能（例: "0,2,5"） / If multiple matches are found, select by number(s) - supports comma-separated selections (e.g., "0,2,5")
   - 複数選択時に1つのKMLファイルに統合するかを選択 / Choose whether to merge multiple selections into one KML file
   - KMLファイルは`{会社名}-{路線名}.kml`の命名パターンで生成 / KML files will be generated with naming pattern: `{company}-{line}.kml`

### 使用例 / Example Session

```bash
$ cargo run N02-20_RailroadSection.geojson
==================================
enter train company name or line name or 'q' to exit:
山手線
candidates:
[0] 東日本旅客鉄道 山手線
[1] 神戸市 山手線
choose '0'...'1' or : 'q' to exit
0
creating 東日本旅客鉄道-山手線.kml ...
succeeded!!
```

## 出力 / Output

生成されるKMLファイルには以下が含まれます: / Generated KML files contain:
- `<Placemark>`要素としての鉄道路線セグメント / Railway line segments as `<Placemark>` elements
- 経度,緯度,0形式の座標データ / Coordinate data in longitude,latitude,0 format
- 地図アプリケーション向けの適切なKML構造 / Proper KML structure for mapping applications

## 開発 / Development

### ビルド / Building

```bash
cargo build
```

### テスト実行 / Running Tests

```bash
cargo test
```

### プロジェクト構造 / Project Structure

```
src/
├── main.rs          # CLIアプリケーションとユーザーインタラクション / CLI application and user interaction
└── lib.rs           # コアライブラリ関数 / Core library functions

tests/
└── unit_test.rs     # ダミーデータを使用した高速ユニットテスト / Fast unit tests with dummy data
```

## 依存関係 / Dependencies

- `serde` - JSONシリアライゼーション/デシリアライゼーション / JSON serialization/deserialization
- `serde_json` - JSONパース / JSON parsing

## データ形式 / Data Format

アプリケーションは以下のプロパティ構造を持つGeoJSONデータを期待します: / The application expects GeoJSON data with the following property structure:
- `N02_001`: 鉄道種別コード / Railway type code
- `N02_002`: 鉄道区間コード / Railway segment code  
- `N02_003`: 路線名 / Line name
- `N02_004`: 会社名 / Company name

## ライセンス / License

このプロジェクトはMITライセンスの下で利用可能です。 / This project is available under the MIT License.
