use std::collections::HashMap;
use std::path::{Path, PathBuf};
use v8::{Handle, MapFnTo, PromiseState};
use crate::runtime::isolate_state::IsolateState;
use crate::runtime::script_origin::create_script_origin;

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

pub struct Loader;

impl Loader {
    pub fn new() -> Self {
        Self {}
    }
    pub fn import<'a>(
        &self,
        scope: &mut v8::HandleScope<'a>,
        specifier: &str,
        referrer: &str,
    ) -> Result<v8::Local<'a, v8::Value>, v8::Local<'a, v8::Value>> {
        let scope = &mut v8::TryCatch::new(scope);
        match resolve_module(scope, specifier, referrer) {
            Some(module) => {
                match module.instantiate_module(scope, module_resolve_callback) {
                    Some(_) => {
                        println!("Instantiated");

                        let eval_value = module.evaluate(scope).unwrap();

                        let eval_promise = unsafe{v8::Local::<v8::Promise>::cast(eval_value)};
                        match eval_promise.state() {
                            PromiseState::Fulfilled => Ok(eval_promise.result(scope)),
                            PromiseState::Rejected => Err(eval_promise.result(scope)),
                            PromiseState::Pending => panic!("Pending Promise for Module"),
                        }
                    }
                    None => {
                        let value = v8::String::new(scope,"Something went wrong in the module").unwrap();

                        Err(value.into())
                    }
                }
            }
            None => {
                let val = v8::String::new(scope, "No Module Found").unwrap();
                Err(val.into())
            }
        }
    }
}


/// module_resolve_callback
fn module_resolve_callback<'a>(
    context: v8::Local<'a, v8::Context>,
    specifier: v8::Local<'a, v8::String>,
    import_assertions: v8::Local<'a, v8::FixedArray>,
    referrer: v8::Local<'a, v8::Module>,
) -> Option<v8::Local<'a, v8::Module>> {
    let scope = unsafe { &mut v8::CallbackScope::new(context) };
    let specifier = specifier.to_rust_string_lossy(scope);
    let referrer_path = IsolateState::get(scope).borrow()
        .module_map.hash_to_absolute_map.get(&referrer.get_identity_hash().get()).unwrap().to_string();
    resolve_module(scope, &specifier, &referrer_path)
}

/// resolve_module
fn resolve_module<'s>(scope: &mut v8::HandleScope<'s>,
                      specifier: &str, referrer: &str) -> Option<v8::Local<'s, v8::Module>> {
    let requested_path = normalize_path(specifier, referrer);
    if let Some(module) = IsolateState::get(scope).borrow().module_map.absolute_path_to_map
        .get(&requested_path){
        return Some(v8::Local::new(scope,module));
    }
    let source_code = read_file_to_string(&requested_path);
    let js_code = v8::String::new(scope, &source_code).unwrap();
    let script_origin = create_script_origin(scope, specifier, true);
    let source = v8::script_compiler::Source::new(js_code, Some(&script_origin));
    let module = v8::script_compiler::compile_module(scope, source);
    if let Some(module) = module {
        IsolateState::get(scope).borrow_mut()
            .module_map.insert(scope, &requested_path, module)
    }
    module
}

fn read_file_to_string(path: &str) -> String {
    let source_code = std::fs::read_to_string(path);
    match source_code {
        Ok(code) => code,
        Err(e) => panic!("Error in while reading the file [{}]", e.to_string())
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
    match normalize(specifier, referrer) {
        Ok(val) => {
            println!("VALUE {:?}", val);
            val
        }
        Err(e) => panic!("Error while normalizing the path {}", e.to_string())
    }
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
    } else if specifier_path.extension().unwrap().eq("js") {
        println!("WITHOUT ./");
        specifier_path = referrer_path.parent().unwrap().join(specifier);
        let file_path = specifier_path.canonicalize()?;
        return Ok(file_path.into_os_string().to_str().unwrap().to_string());
    } else if specifier_path.is_absolute() {
        // if it is an absolute path then return it as it is
        println!("ABSOLUTE");
        return Ok(specifier_path.canonicalize()?.to_str().unwrap().to_string());
    } else {
        // now check if the specifier is the module name
        println!("ELSE {:?}", specifier_path.to_str().unwrap().to_string());
        return Ok(specifier_path.to_str().unwrap().to_string());
    }
    Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Cannot find Module"))
}


#[cfg(test)]
#[allow(unused)]
mod test_normalize {
    use crate::runtime::module::normalize;

    #[test]
    fn test_normalize_fn_returns_ok_and_is_empty() {
        let (referrer, specifier) = get_referrer_and_specifier("fs");
        let result = normalize(specifier, &referrer);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_normalize_fn_is_not_empty() {
        let (referrer, specifier) = get_referrer_and_specifier("./index.js");
        let result = normalize(specifier, &referrer);
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn test_normalize_fn_len_greater_than_0() {
        let (referrer, specifier) = get_referrer_and_specifier("./index.js");
        let result = normalize(specifier, &referrer);
        assert!(result.unwrap().len() > 0)
    }

    #[test]
    fn test_normalize_fn_result_1() {
        let (referrer, specifier) = get_referrer_and_specifier("./index.js");
        let result = normalize(specifier, &referrer);
        assert_eq!(result.unwrap().replace("\\\\?\\", "")
                   , r"D:\Languages\Rust\Rust Projects\js_runtime\index.js".to_string())
    }

    #[test]
    fn test_normalize_fn_result_2() {
        let (referrer, specifier) = get_referrer_and_specifier("fs");
        let result = normalize(specifier, &referrer);
        assert_eq!(result.unwrap()
                   , "fs".to_string())
    }

    fn get_referrer_and_specifier(specifier: &'static str) -> (String, &'static str) {
        let specifier = specifier;
        let mut referrer = std::env::current_dir().unwrap();
        referrer.push("package.json");
        let referrer = referrer.into_os_string().into_string().unwrap();
        (referrer, specifier)
    }
}