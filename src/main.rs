use std::env;
use std::fs;
use std::process;

use nadesiko4::{NakoOptions, run_easy};

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let mut options = NakoOptions::new();

    if args.is_empty() {
        print_help();
        process::exit(0);
    }

    while args.len() > 0 {
        let arg = args.remove(0);
        if arg == "--debug" || arg == "-D" || arg == "debug" {
            options.is_debug = true;
            continue;
        }
        if arg == "--help" || arg == "-h" {
            print_help();
            process::exit(0);            
        }
        if arg == "--version" || arg == "-V" {
            println!("nadesiko4 {}", nadesiko4::version());
            process::exit(0);
        }
        if arg == "--eval" || arg == "-e" || arg == "eval" {
            let code = args.remove(0);
            run_code(&code, &options);
            continue;
        }
        // ファイル実行: nadesiko4 <file>
        let file_pathg = arg;
        if let Err(err) = run_file(&file_pathg, &options) {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}

/// Run the given file
fn run_file(path: &str, options: &NakoOptions) -> Result<(), String> {
    let src = fs::read_to_string(path).map_err(|e| format!("ファイル読み込みに失敗しました: {}", e))?;
    run_code(&src, options);
    Ok(())
}

/// Run the given code string
fn run_code(code: &str, options: &NakoOptions) {
    let output = run_easy(code, options);
    if !output.is_empty() {
        println!("{}", output);
    }
}

/// Print help message
fn print_help() {
    println!("nadesiko4 {}", nadesiko4::version());
    println!("使い方:");
    println!("  nadesiko4 <file>          ファイルを実行");
    println!("  nadesiko4 -e \"code\"      文字列コードを実行");
    println!("  nadesiko4 --help           ヘルプを表示");
    println!("  nadesiko4 --version        バージョンを表示");
    println!("  nadesiko4 repl             (未実装) REPL");
}
