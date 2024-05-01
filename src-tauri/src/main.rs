// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

// -- Re-Exports
pub use error::{Error, Result};

mod ctx;
mod error;
mod ipc;
mod model;

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc::get_cat])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
