use std::env::args;
use tokio::*;

use crate::cli::RuntimeCli;
mod cli;
mod runtime;

#[tokio::main]
async fn main() {

    let isolate = &mut v8::Isolate::new(Default::default());

    let scope = &mut v8::HandleScope::new(isolate);


    // let args = args().collect::<Vec<String>>();
    // let cli = RuntimeCli::new();
    // let file_name = cli.start(args);
    // println!("{:?}", file_name);

}