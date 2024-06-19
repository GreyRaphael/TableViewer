// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::{collections::HashMap, path::PathBuf};

use polars::prelude::*;

fn read_parquets(paths: Arc<[PathBuf]>, sql: &str) -> Result<DataFrame, PolarsError> {
    let lf = LazyFrame::scan_parquet_files(paths, Default::default())?;
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()?.with_row_index("idx", Some(1))
}

fn read_ipcs(paths: Arc<[PathBuf]>, sql: &str) -> Result<DataFrame, PolarsError> {
    let lf = LazyFrame::scan_ipc_files(paths, Default::default())?;
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()?.with_row_index("idx", Some(1))
}

fn read_csvs(paths: Arc<[PathBuf]>, sql: &str, sep: u8) -> Result<DataFrame, PolarsError> {
    let lf = LazyCsvReader::new_paths(paths)
        .with_missing_is_null(true)
        // .with_try_parse_dates(true)
        .with_separator(sep)
        .finish()?;
    let mut ctx = polars::sql::SQLContext::new();
    ctx.register("LAST", lf);
    ctx.execute(sql)?.collect()?.with_row_index("idx", Some(1))
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
                // .map(|(it, name)| (name.to_string(), std::format!("{:?}", it.next().unwrap())))
                .collect()
        })
        .collect::<Vec<HashMap<_, _>>>();

    let table = serde_json::json!({
        "col_count":col_names.len(),
        "row_count":row_count,
        "headers":headers,
        "body":body,
    });

    serde_json::to_string(&table).unwrap()
}

fn deal_error(e: PolarsError) -> String {
    serde_json::json!({
        "err_msg":e.to_string(),
    })
    .to_string()
}

#[tauri::command]
fn read_parquet_files(filenames: &str, sql: &str) -> String {
    // filenames is json string
    let filename_vec: Vec<String> = serde_json::from_str(filenames).unwrap();
    let paths = filename_vec.into_iter().map(PathBuf::from).collect();
    match read_parquets(paths, sql) {
        Ok(df) => generate_table(&df),
        Err(e) => deal_error(e),
    }
}

#[tauri::command]
fn read_ipc_files(filenames: &str, sql: &str) -> String {
    // filenames is json string
    let filename_vec: Vec<String> = serde_json::from_str(filenames).unwrap();
    let paths = filename_vec.into_iter().map(PathBuf::from).collect();
    match read_ipcs(paths, sql) {
        Ok(df) => generate_table(&df),
        Err(e) => deal_error(e),
    }
}

#[tauri::command]
fn read_csv_files(filenames: &str, sql: &str, sep: u8) -> String {
    // filenames is json string
    let filename_vec: Vec<String> = serde_json::from_str(filenames).unwrap();
    let paths = filename_vec.into_iter().map(PathBuf::from).collect();
    match read_csvs(paths, sql, sep) {
        Ok(df) => generate_table(&df),
        Err(e) => deal_error(e),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            read_parquet_files,
            read_ipc_files,
            read_csv_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
