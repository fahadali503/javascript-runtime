use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::runtime::isolate_state::IsolateState;

/// `ModuleMap` is a struct that contains two `HashMap`s, one that maps a hash to an absolute path, and
/// another that maps an absolute path to a `v8::Global<v8::Module>`.
///
/// Properties:
///
/// * `hash_to_absolute_map`: This is a map that maps the hash of a module to its absolute path.
/// * `absolute_path_to_map`: This is a hashmap that maps the absolute path of a module to the
/// v8::Global<v8::Module> object.
pub struct ModuleMap {
    hash_to_absolute_map: HashMap<i32, String>,
    absolute_path_to_map: HashMap<String, v8::Global<v8::Module>>,
}

impl ModuleMap {
    pub fn new() -> Self {
        Self {
            hash_to_absolute_map: HashMap::new(),
            absolute_path_to_map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, scope: &mut v8::HandleScope,
                  file_name: &str,
                  module: v8::Local<v8::Module>,
    ) {
        self.hash_to_absolute_map.insert(module.get_identity_hash().get(), file_name.to_owned());
        let global = v8::Global::new(scope, module);
        self.absolute_path_to_map.insert(file_name.to_owned(), global);
    }
}


/// > It takes a specifier and a referrer and returns the normalized path
///
/// Arguments:
///
/// * `specifier`: The path to the module you want to import.
/// * `referrer`: The path of the file that is importing the module.
///
/// Returns:
///
/// A string
pub fn normalize_path(specifier: &str, referrer: &str) -> String {
    println!("Result {:?}", normalize(specifier, referrer));
    String::from("")
}


fn normalize(specifier: &str, referrer: &str) -> std::io::Result<String> {
    let referrer_path = PathBuf::from(referrer);
    let mut specifier_path = PathBuf::from(specifier);

    // check if the specifier is not an absolute path
    // Also check if the specifier is from the root of the project
    if specifier_path.starts_with(".") || specifier_path.starts_with("..") {
        // Then join it with the referrer parent.
        specifier_path = referrer_path.parent().unwrap().join(specifier);
        let file_path = specifier_path.canonicalize()?;
        println!(". and ..");
        return Ok(file_path.into_os_string().to_str().unwrap().to_string());
    } else if specifier_path.is_absolute(){
        // if it is an absolute path then return it as it is
        println!("ABSOLUTE");
        return Ok(specifier_path.canonicalize()?.to_str().unwrap().to_string())
    } else {
        // now check if the specifier is the module name

        Err(std::io::Error::new(std::io::ErrorKind::NotFound,"Cannot find Module"))
    }
}


#[cfg(test)]
#[allow(unused)]
mod tests {
    use crate::runtime::module::normalize_path;

    #[test]
    fn test_normalize_path() {}
}