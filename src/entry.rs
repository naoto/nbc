use crate::model;
use std::path::Path;
use std::fs::{self, File};
use pulldown_cmark::{html, Options, Parser};
use serde_json::json;
use std::io;
use std::io::prelude::*;

pub fn make(input_dir: &Path, output_dir: &Path) -> io::Result<Vec<model::DiaryList>> {
    let mut file_list: Vec<model::DiaryList> = Vec::new();

    for entry in fs::read_dir(input_dir)? {
        let entry = entry?;
        let path = entry.path();
        let orgfilename = path.file_name().unwrap().to_string_lossy();
        let filename = orgfilename.split(".").next().unwrap();

        let mut file = File::open(&path)?;
        let mut content = String::new();
        let _ = file.read_to_string(&mut content);

        let mut lines = content.lines();
        let title = lines.next().unwrap();
        let html = markdown_to_html(&content);

        let _ = output_json(filename, &title, &html, &output_dir);
        let d = model::DiaryList {title: title.to_string(), date: filename.to_string()};

        file_list.push(d);
    }

    Ok(file_list)
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
