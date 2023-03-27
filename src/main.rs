use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use tokio::*;
use js_runtime::runtime::runtime::JsRuntime;

mod cli;


#[tokio::main]
async fn main() {
    let mut runtime = JsRuntime::new();


    runtime.import("./index.js");
    // runtime.import("fs");
    // runtime.import(r"D:\Languages\Rust\Rust Projects\js_runtime\index.js");


    // let args = args().collect::<Vec<String>>();
    // let cli = RuntimeCli::new();
    // let file_name = cli.start(args);
    // println!("{:?}", file_name);
}
