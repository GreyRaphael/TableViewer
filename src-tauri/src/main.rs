// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;

use polars::prelude::*;

fn read_parquet(filename: &str, limit: usize) -> Result<DataFrame, PolarsError> {
    let mut file = std::fs::File::open(filename)?;
    ParquetReader::new(&mut file)
        .with_n_rows(Some(limit))
        .finish()
}

fn read_csv(filename: &str, sep: u8, limit: usize) -> Result<DataFrame, PolarsError> {
    CsvReader::from_path(filename)?
        .with_separator(sep)
        .with_n_rows(Some(limit))
        .finish()
}

fn read_json(filename: &str, limit: usize) -> Result<DataFrame, PolarsError> {
    let mut file = std::fs::File::open(filename)?;
    Ok(JsonReader::new(&mut file).finish()?.head(Some(limit)))
}

fn query_dataframe(df: &DataFrame, sql_str: &str) -> Result<DataFrame, PolarsError> {
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("CURRENT", df.clone().lazy());
    ctx.execute(sql_str).unwrap().collect()
}

fn generate_table(df: &DataFrame) -> String {
    let col_names = df.get_column_names();
    let col_types = df.dtypes();

    let headers = col_names
        .iter()
        .zip(col_types.iter())
        .map(|(k, v)| {
            serde_json::json!({
                "title":std::format!("{}({})", k, v),
                "key":k.to_string(),
                "resizable":true,
            })
        })
        .collect::<Vec<_>>();

    let mut iters = df.iter().map(|s| s.iter()).collect::<Vec<_>>();
    let body = (0..df.height())
        .map(|_| {
            iters
                .iter_mut()
                .zip(col_names.iter())
                .map(|(it, name)| (name.to_string(), it.next().unwrap().to_string()))
                .collect()
        })
        .collect::<Vec<HashMap<_, _>>>();

    let table = serde_json::json!({
        "headers":headers,
        "body":body,
    });

    serde_json::to_string(&table).unwrap()
}

#[tauri::command]
fn read_file(filename: &str) -> String {
    if filename.ends_with(".parquet") {
        let df = read_parquet(filename, 1000).unwrap();
        generate_table(&df)
    } else if filename.ends_with(".json") {
        let df = read_json(filename, 1000).unwrap();
        generate_table(&df)
    } else if filename.ends_with(".csv") {
        let df = read_csv(filename, b';', 1000).unwrap();
        generate_table(&df)
    } else {
        // empty case
        let df = DataFrame::empty();
        generate_table(&df)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
