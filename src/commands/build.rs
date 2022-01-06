use crate::entry;
use crate::model;
use std::path::Path;
use serde_json::json;
use std::fs::{self};
use std::fs::File;
use std::io;
use std::io::prelude::*;

pub fn build(input_dir: &Path, output_dir: &Path) -> io::Result<()> {
    if input_dir.is_dir() {
        create_output(output_dir);

        let file_list: Vec<model::DiaryList> = entry::make(input_dir, output_dir)?;
        let _ = output_list_json(&output_dir, &file_list);
    }

    Ok(())
}

fn output_list_json(output: &Path, file_list: &Vec<model::DiaryList>) -> io::Result<()> {
    let list = json!(file_list);
    let filepath = output.join("list.json");
    let mut file = File::create(filepath)?;
    let _ = write!(file, "{}", list.to_string());
    file.flush()?;

    Ok(())
}

fn create_output(dir: &Path) {
    let _ = fs::create_dir_all(dir);
}
