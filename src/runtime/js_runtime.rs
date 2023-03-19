use std::sync::{Arc, Once};
use v8::{Isolate, OwnedIsolate};

static INIT_PLATFORM:Once = Once::new();

fn init_platform(){
    INIT_PLATFORM.call_once(||{
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}
type Arc_Isolate = Arc<OwnedIsolate>;
pub struct JsRuntime{
    isolate: Option<OwnedIsolate>,
}

impl JsRuntime {
    pub fn new() -> Self {
        init_platform();

        let mut isolate = v8::Isolate::new(Default::default());
        JsRuntime::create(isolate)
    }

    fn create(mut isolate:OwnedIsolate) -> Self{
        let global_context = {
            let handle_scope = &mut v8::HandleScope::new(&mut isolate);
            let context = v8::Context::new(handle_scope);
            v8::Global::new(handle_scope,context)
        };

        Self{
            isolate:Some(isolate)
        }
    }

}