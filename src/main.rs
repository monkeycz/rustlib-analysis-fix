#[macro_use]
extern crate serde_json;

use std::{env, fs, io};
use std::io::{stdout, Write};
use std::path::Path;
use std::fs::OpenOptions;
use serde_json::Value;

fn fix_span_file_name(span_file_name_value: &mut Value, rust_src_path: &Path) {
    let span_file_name = String::from_utf8(
        span_file_name_value
            .as_array()
            .unwrap()
            .into_iter()
            .map(|x| x.as_u64().unwrap() as u8)
            .collect(),
    ).unwrap();
    *span_file_name_value = json!(
        rust_src_path
            .join(span_file_name)
            .to_str()
            .unwrap()
            .as_bytes()
    );
}

fn fix_analysis_file(analysis_file_path: &Path, rust_src_path: &Path) -> io::Result<()> {
    let analysis_file = OpenOptions::new().read(true).open(analysis_file_path)?;
    let mut analysis: Value = serde_json::from_reader(&analysis_file)?;

    fix_span_file_name(&mut analysis["prelude"]["span"]["file_name"], rust_src_path);

    let item_list = vec!["imports", "defs", "impls", "refs", "macro_ref", "relations"];
    for item_name in &item_list {
        let mut i = 0;
        loop {
            if let Some(item) = analysis[item_name].get_mut(i) {
                fix_span_file_name(&mut item["span"]["file_name"], rust_src_path);
            } else {
                break;
            }
            i += 1;
        }
    }

    let analysis_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(analysis_file_path)?;
    serde_json::to_writer(&analysis_file, &analysis)?;

    return Ok(());
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("rustlib-analysis-fix <rust_src_path> <rust_analysis_path>");
        return;
    }

    let rust_src_path = Path::new(&args[1]);
    let rust_analysis_path = Path::new(&args[2]);

    for entry in fs::read_dir(rust_analysis_path).unwrap() {
        let file_path = entry.unwrap().path();
        if let Some(extension) = file_path.extension() {
            if extension != "json" {
                continue;
            }
        } else {
            continue;
        }

        print!("{} ... ", file_path.file_name().unwrap().to_str().unwrap());
        stdout().flush().unwrap();

        if let Err(err) = fix_analysis_file(&file_path, &rust_src_path) {
            println!("Error ({}).", err);
        } else {
            println!("Done.");
        }
    }
}
