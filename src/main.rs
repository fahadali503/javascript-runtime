use argmap::ArgMap;
use crate::cli::JsRuntimeCli;

mod cli;

fn main() {

    let booleans = &["h","help","v","version"];
    let args = std::env::args();

    let cli = JsRuntimeCli::new(booleans,args);
    let file_name = cli.start();
    println!("FILE NAME {:?}",file_name);

    // let mut arg_map = argmap::new().booleans(&["h","help","v","version"]);
    // let (args,argv) = arg_map.parse(std::env::args());
    //
    // println!("Args {:?}",args);
    // println!("Argv {:?}",argv);

    // println!("ArgV {:?}",argv.contains_key("h"));
    // println!("ArgV {:?}",argv.contains_key("help"));
    // println!("ArgV {:?}",argv.contains_key("v"));
    // println!("ArgV {:?}",argv.contains_key("version"));
    // println!("ArgV {:?}",argv.contains_key("file"));
}
