use std::collections::HashMap;

pub struct JsModuleMap{
    hash_absolute_path: HashMap<i32,String>,
    absolute_path_to_module:HashMap<String,v8::Global<v8::Module>>
}

impl JsModuleMap{
    pub fn new() -> Self{
        Self{
            absolute_path_to_module:HashMap::new(),
            hash_absolute_path:HashMap::new()
        }
    }

    fn insert(
        &mut self, scope:&mut v8::HandleScope, file_path:&str, module: v8::Local<v8::Module>
    ){
        self.hash_absolute_path.insert(i32::from(module.get_identity_hash()), file_path.to_owned());
        let module = v8::Global::new(scope,module);
        self.absolute_path_to_module.insert(file_path.to_owned(),module);
    }

    
}