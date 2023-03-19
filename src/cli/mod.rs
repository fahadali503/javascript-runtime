use commander::Commander;

pub const APP_NAME: &str = "Binzo";
pub const APP_DESCRIPTION: &str = "A Javascript Runtime built on top of Boa Engine.";
pub const APP_VERSION: &str = env!("CARGO_PKG_VERSION");


pub struct RuntimeCli {
    commander:Commander
}

impl RuntimeCli {
    pub fn new() -> Self{
        let mut commander = Commander::new().version(APP_VERSION)
               .usage_desc(APP_DESCRIPTION);
        Self{
            commander
        }
    }

    /// start function will return Option containing the name of the file to be
    /// executed
    pub fn start(self,args:Vec<String>) -> Option<String>{
        let cli = self.set_options().commander.parse_list_or_exit(args);
        if cli.get_str("file").is_some() {
            cli.get_str("file")
        }else{
            None
        }
    }
    fn set_options(self) -> Self{
        let commander = self.commander.
            option_str("-f,--file [value]","file to be executed by the runtime",None);
        Self{commander}
    }

    fn get_str(&self,arg:&str) -> Option<String>{
        self.commander.get_str(arg)
    }
}


#[cfg(test)]
#[allow(unused_imports)]
mod tests {}