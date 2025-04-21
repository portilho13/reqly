// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod protocols;

use protocols::http::Http;

#[tokio::main]
async fn main() {
    //reqly_lib::run()
}
