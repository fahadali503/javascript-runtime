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
    pub fn start(&mut self){
        self.app.parse();

    }

    fn set_options(&mut self){
        self.app.add_ops(self.set_option(FILE,"f"));
        self.app.add_ops(self.set_option(HELP,"h"));
        self.app.add_ops(self.set_option(VERSION,"v"));

    }
    fn set_option(&mut self,long:&str,short:&str) -> Ops{
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

    fn get_file_option(&self) -> String{
        let file_option = self.app.get_value(FILE).unwrap();
        file_option
    }

    fn get_version_option(&self) -> String {
        let version = self.app.get_value(VERSION).unwrap();
        version
    }

    fn get_help_option(&self) -> String{
        let help_option = self.app.get_value(HELP).unwrap();
        help_option
    }

    fn print_usage(&self){
        println!("{}",USAGE)
    }
}