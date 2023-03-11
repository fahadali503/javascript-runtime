mod cli;

fn main() {
    let mut cli = cli::JsRuntimeCli::new();
    let filename = cli.start();
    println!("FILE NAME {:?}",filename);
}
