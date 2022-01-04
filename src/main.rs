#![allow(unused)]

use structopt::StructOpt;
use std::io::{self, BufReader};
use std::fs::{self, DirEntry};
use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use serde_json::json;
use serde::{Deserialize, Serialize};

#[derive(StructOpt)]
struct Cli {
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,

    #[structopt(parse(from_os_str))]
    output: std::path::PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct DiaryList {
    title: String,
    date: String,
}

fn markdown_to_html(text: &str) -> String {
    let markdown_input = text;
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

fn output_json(filename: &str, title: &str, content: &str, output: &Path) -> io::Result<()> {
    let diary = json!({
        "title": title,
        "date": filename,
        "body": content
    });

    let format = ".json";
    let filepath = output.join(filename.to_string() + format);
    let mut file = File::create(filepath)?;
    write!(file, "{}",  diary.to_string())?;
    file.flush()?;

    Ok(())
}

fn output_list_json(output: &Path, file_list: &Vec<DiaryList>) -> io::Result<()> {
    let list = json!(file_list);
    let filepath = output.join("list.json");
    let mut file = File::create(filepath)?;
    write!(file, "{}", list.to_string());
    file.flush()?;

    Ok(())
}

fn create_output(dir: &Path) {
    fs::create_dir_all(dir);
}

fn main() -> io::Result<()> {
    let args = Cli::from_args();
    let input_dir = Path::new(&args.input);
    let output_dir = Path::new(&args.output);

    if input_dir.is_dir() {
        create_output(output_dir);
        let mut file_list: Vec<DiaryList> = Vec::new();

        for entry in fs::read_dir(input_dir)? {
            let entry = entry?;
            let path = entry.path();
            let orgfilename = path.file_name().unwrap().to_string_lossy();
            let filename = orgfilename.split(".").next().unwrap();

            let mut file = File::open(&path)?;
            let mut content = String::new();
            file.read_to_string(&mut content);

            let mut lines = content.lines();
            let mut title = lines.next().unwrap();
            let html = markdown_to_html(&content);

            output_json(filename, &title, &content, &output_dir);
            let d = DiaryList {title: title.to_string(), date: filename.to_string()};
            file_list.push(d);
        }

        output_list_json(&output_dir, &file_list);
    }
    Ok(())
}
