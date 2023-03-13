use std::env::Args;
use std::ffi::OsString;
use argmap::*;


/// CONSTANTS
/// These are the constants that will be used for the arguments
const PROJECT_VERSION:&'static str = env!("CARGO_PKG_VERSION");
const FILE_ARG:(&str,&str) = ("file","f");
const VERSION_ARG:(&str,&str) = ("version","v");
const HELP_ARG:(&str,&str) = ("help","h");
const USAGE:&'static str = r#"
    This is the javascript runtime built on top of Boa Engine.
    USAGE:
        -f | --file     <String> => The file that will be executed by runtime.
        -v | --version  <FLAG>   => The current version of the executable.
        -h | --help     <FLAG>   => Display the usage of cli.
"#;

pub struct JsRuntimeCli{
    arg_map: ArgMap,
    args: List,
    argv: Map
}

impl JsRuntimeCli {
    pub fn new(booleans:&[&str],args:Args) -> Self {
        // let args = &["h","help","v","version"];
        let mut arg_map = argmap::new().booleans(booleans);
        let (args,argv) = arg_map.parse(args);
        Self{
            arg_map,
            args,
            argv
        }
    }

    pub fn start(&self) -> Option<String> {
        if self.get_key(FILE_ARG.0,FILE_ARG.1) {
           let file = self.get_last_value_from_vec();
            if file.0.is_some() && file.1.is_none() {
                let long_file_arg= file.0.unwrap().to_string();
                Some(long_file_arg)
            }else{
                let short_file_arg = file.1.unwrap().to_string();
                Some(short_file_arg)
            }
        } else if self.get_key(VERSION_ARG.0,VERSION_ARG.1) {
            println!("version {}",PROJECT_VERSION);
            None
        }
        else if self.get_key(HELP_ARG.0,HELP_ARG.1) {
            println!("{}",USAGE);
            None
        }else{
            None
        }
    }

    fn get_key(&self, long:&str,short:&str) -> bool{
        self.argv.contains_key(long) || self.argv.contains_key(short)
    }

    fn get_first_value_from_vec(&self,keys:(&str,&str)) -> TupleOptionalRefString {
        (self.argv.get(keys.0).and_then(|v| v.first()),
        self.argv.get(keys.1).and_then(|v| v.first()))
    }
    fn get_last_value_from_vec(&self,) -> TupleOptionalRefString {
        (self.argv.get(FILE_ARG.0).and_then(|v| v.last()),
         self.argv.get(FILE_ARG.1).and_then(|v| v.last()))
    }


}

type OptionalRefString<'s> = Option<&'s String>;
type TupleOptionalRefString<'s> = (OptionalRefString<'s>,OptionalRefString<'s>);

#[cfg(test)]
#[allow(unused_imports)]
mod tests{

}