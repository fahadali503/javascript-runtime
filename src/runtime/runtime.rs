use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Once;
use v8::{HandleScope, Isolate};
use crate::runtime::builtins::Builtins;
use crate::runtime::isolate_state::IsolateState;
use crate::runtime::module::{Loader, normalize_path};


static INIT_PLATFORM:Once = Once::new();
fn init_platform(){
    INIT_PLATFORM.call_once(||{
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

pub struct JsRuntime{
    isolate:Option<v8::OwnedIsolate>
}

impl JsRuntime{
    pub fn new() -> Self {
        init_platform();
        let isolate = v8::Isolate::new(Default::default());
        Self::create(isolate)
    }

    fn create(mut isolate: v8::OwnedIsolate) -> Self{
        let global_context = {
            let scope = &mut HandleScope::new(&mut isolate);
            let context = v8::Context::new(scope);
            v8::Global::new(scope,context)
        };
        isolate.set_slot(IsolateState::new(global_context));
        {
            let context = IsolateState::get(&mut isolate).borrow().context();
            let scope = &mut v8::HandleScope::with_context(&mut isolate,context);
            Builtins::create(scope);
        }
        Self{
            isolate:Some(isolate)
        }
    }

    pub fn isolate(&mut self) -> &mut v8::Isolate{
        match self.isolate.as_mut() {
            Some(isolate) => isolate,
            None => unreachable!()
        }
    }

    pub fn import(&mut self, file_name:&str) {
        let isolate = self.isolate();
        let context = IsolateState::get(isolate).borrow().context();
        let scope = &mut v8::HandleScope::with_context(isolate,context);
        println!("FIle name {}", file_name);
        let specifier = file_name;
        let mut referrer = std::env::current_dir().unwrap().to_str()
            .unwrap().to_owned();

        referrer.push_str("\\package.json"); // this is to know if the cwd is a javascript project
        // let file = normalize_path(specifier,&referrer);
        let loader = Loader::new();
        match loader.import(scope,specifier,&referrer){
            Ok(val) => {
                let eval_val = val.to_detail_string(scope).unwrap();
                println!("Final Result {}",eval_val.to_rust_string_lossy(scope) );
            },
            Err(e) => {
                println!("Error {:?}",e.to_rust_string_lossy(scope))
            }
        };
    }
}