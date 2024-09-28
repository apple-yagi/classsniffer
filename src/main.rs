use clap::Parser;
use classsniffer::{css, tsx};
use std::fs;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// Target CSS file (default: stdin)
    #[clap(value_parser)]
    input_file: Vec<String>,
}

fn main() {
    let cli_args = CliArgs::parse();

    // CSSファイルを読み込み
    let css_file_path = &cli_args.input_file[1];
    let css_content = fs::read_to_string(css_file_path).expect("Failed to read the CSS file");

    // クラス名を抽出
    let classnames = css::extract_classname(&css_content);

    // 結果を表示
    for classname in classnames {
        println!("{}", classname);
    }

    // TSXファイルを読み込み
    let tsx_content = fs::read_to_string("testdata/test.tsx").expect("Failed to read the Tsx file");
    tsx::extract_classname(&tsx_content);
}
