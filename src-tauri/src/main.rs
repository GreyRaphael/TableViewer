// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;

use polars::prelude::*;
use serde::{Deserialize, Serialize};

fn read_parquet(filename: &str, limit: usize) -> Result<DataFrame, PolarsError> {
    let mut file = std::fs::File::open(filename)?;
    ParquetReader::new(&mut file)
        .with_n_rows(Some(limit))
        .finish()
}

#[tauri::command]
fn get_data(filename: &str) -> String {
    let df = read_parquet(filename, 1000).unwrap();

    // let mut buf = Vec::new();
    // JsonWriter::new(&mut buf)
    //     .with_json_format(JsonFormat::Json)
    //     .finish(&mut df)
    //     .unwrap();
    // let df_json = String::from_utf8(buf).unwrap();
    // println!("{}", df_json);
    // df_json

    let col_names = df.get_column_names();
    let mut iters = df.iter().map(|s| s.iter()).collect::<Vec<_>>();
    let mut json_list = Vec::new();
    for _ in 0..df.height() {
        let mut dict = HashMap::new();
        for (i, iter) in iters.iter_mut().enumerate() {
            let value = iter.next().unwrap();
            dict.insert(col_names[i], value.to_string());
        }
        json_list.push(dict);
    }

    let json_string = serde_json::to_string(&json_list).unwrap();
    json_string
}

#[derive(Serialize, Deserialize)]
struct Header {
    title: String,
    key: String,
    resizable: bool,
    // align:String,
    // ellipsis:bool,
}

#[tauri::command]
fn get_header(filename: &str) -> String {
    // let df = read_parquet("E:/test-data/Iris.parquet", 1).unwrap();
    let df = read_parquet(filename, 1).unwrap();
    let col_names = df.get_column_names();
    let col_types = df.dtypes();

    // let it = col_names.iter().zip(col_types.iter());
    // let mut json_list = Vec::new();
    // for (_, (col_name, col_type)) in it.enumerate() {
    //     let header = std::format!("{}({})", col_name, col_type);
    //     let item = Header {
    //         title: header,
    //         key: col_name.to_string(),
    //     };
    //     json_list.push(item);
    // }

    let json_list: Vec<Header> = col_names
        .into_iter()
        .zip(col_types.into_iter())
        .map(|(k, v)| Header {
            title: std::format!("{}({})", k, v),
            key: k.to_string(),
            resizable: true,
            // align: "center".to_string(),
            // ellipsis: true,
        })
        .collect();

    let json_string = serde_json::to_string(&json_list).unwrap();

    // println!("{}", json_string);
    json_string
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_data, get_header])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
