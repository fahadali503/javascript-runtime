use std::ffi::OsString;
use paprika::{App, Ops};


/// CONSTANTS
/// These are the constants that will be used for the arguments
const PROJECT_VERSION:&'static str = env!("CARGO_PKG_VERSION");
const FILE:&'static str = "file";
const VERSION:&'static str = "version";
const HELP:&'static str = "help";
const USAGE:&'static str = r#"
    This is the javascript runtime built on top of Boa Engine.
    USAGE:
        -f | --file     <String> => The file that will be executed by runtime.
        -v | --version  <FLAG>   => The current version of the executable.
        -h | --help     <FLAG>   => Display the usage of cli.
"#;


pub struct JsRuntimeCli{
    app:App
}
impl JsRuntimeCli{
    pub fn new() -> Self{
        let app = App::new();
        Self{app}
    }

    /// start function will return Option<str>
    /// The return value will optionally be the filename
    pub fn start(&mut self) -> Option<OsString>{
        self.set_options();
        self.app.parse();
        // check if the file name has been passed in the cli argument
        if self.check_if_option_exists(FILE) {
            let filename = OsString::from(self.get_file_option(FILE));
            Some(filename)
        }
        // check if the version has been passed in the cli argument
        else if self.check_if_option_exists(VERSION) {
            println!("runtime version {}",PROJECT_VERSION);
            None
        }
        // check if the help has been passed in the cli argument
        else if self.check_if_option_exists(HELP) {
            self.print_usage();
            None
        }else{
            None
        }
    }

    fn set_options(&mut self){
        self.app.add_ops(self.set_option(FILE,"f"));
        self.app.add_ops(self.set_option(HELP,"h"));
        self.app.add_ops(self.set_option(VERSION,"v"));

    }
    fn set_option(&self,long:&str,short:&str) -> Ops{
        Ops::new()
            .long(long)
            .short(short)
    }

    fn check_if_option_exists(&self,option:&str) -> bool {
        if self.app.has_ops(option) {
            true
        }else{
            false
        }
    }

    fn get_file_option(&self,option_name:&str) -> String{
        let file_option = self.app.get_value(option_name).unwrap();
        file_option
    }

    fn print_usage(&self){
        println!("{}",USAGE);
        std::process::exit(0);
    }
}