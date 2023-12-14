// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::collections::HashMap;

use polars::{io::RowCount, prelude::*};

fn read_parquet(filename: &str, sql: &str) -> Result<DataFrame, PolarsError> {
    let lf = LazyFrame::scan_parquet(filename, Default::default())?.with_row_count("idx", Some(1));
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()
}

fn read_ipc(filename: &str, sql: &str) -> Result<DataFrame, PolarsError> {
    let lf = LazyFrame::scan_ipc(filename, Default::default())?.with_row_count("idx", Some(1));
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()
}

fn read_csv(filename: &str, sql: &str, sep: u8) -> Result<DataFrame, PolarsError> {
    let lf = LazyCsvReader::new(filename)
        .with_missing_is_null(true)
        // .with_try_parse_dates(true)
        .with_separator(sep)
        .with_row_count(Some(RowCount {
            name: "idx".to_string(),
            offset: 1,
        }))
        .finish()?;
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()
}

fn generate_table(df: &DataFrame) -> String {
    let col_names = df.get_column_names();
    let col_types = df.dtypes();
    let row_count = df.height();

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
        "row_count":row_count,
        "headers":headers,
        "body":body,
    });

    serde_json::to_string(&table).unwrap()
}

#[tauri::command]
fn read_parquet_file(filename: &str, sql: &str) -> String {
    let df = read_parquet(filename, sql).unwrap();
    generate_table(&df)
}

#[tauri::command]
fn read_ipc_file(filename: &str, sql: &str) -> String {
    let df = read_ipc(filename, sql).unwrap();
    generate_table(&df)
}

#[tauri::command]
fn read_csv_file(filename: &str, sql: &str, sep: u8) -> String {
    let df = read_csv(filename, sql, sep).unwrap();
    generate_table(&df)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_parquet_file,
            read_ipc_file,
            read_csv_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
