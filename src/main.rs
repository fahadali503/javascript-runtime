use std::cell::RefCell;
use std::env::args;
use std::rc::Rc;
use tokio::*;

use crate::cli::RuntimeCli;
use crate::runtime::isolate_state::IsolateState;

mod cli;
mod runtime;

#[tokio::main]
async fn main() {
    let mut runtime = runtime::js_runtime::JsRuntime::new();
    let isolate = runtime.isolate();
    let isolate_state = isolate.get_slot::<Rc<RefCell<IsolateState>>>().unwrap();
    let is = isolate_state.borrow().get_context();
    println!("Global Context = {:?}",is);

    // let args = args().collect::<Vec<String>>();
    // let cli = RuntimeCli::new();
    // let file_name = cli.start(args);
    // println!("{:?}", file_name);

}