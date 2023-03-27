use crate::runtime::script_origin::create_script_origin;

fn logger(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, _rv: v8::ReturnValue) {
    println!("{}", args.get(0).to_rust_string_lossy(scope))
}

pub struct Builtins;

impl Builtins {
    pub fn create(scope:&mut v8::HandleScope){
        let bindings = v8::Object::new(scope);
        let printer_name = v8::String::new(scope,"printer").unwrap();
        let mut value = v8::Function::new(scope, logger).unwrap();
        bindings.set(scope,printer_name.into(),value.into());

        let val = {
            let tc_scope = &mut v8::TryCatch::new(scope);
            let script_origin = create_script_origin(tc_scope, "console.js", false);
            let code = v8::String::new(tc_scope, r#"
            ({printer}) => {
                Object.defineProperty(globalThis.console,"log",{
                    value:printer
                })
            }
        "#).unwrap();
            v8::Script::compile(tc_scope, code, Some(&script_origin))
                .and_then(|script| script.run(tc_scope))
                .map_or_else(|| Err(tc_scope.stack_trace().unwrap()), Ok)
        };
        if let Ok(value) = val {
            let func = v8::Local::<v8::Function>::try_from(value).unwrap();
            let recv = v8::undefined(scope);
            let args = [bindings.into()];
            let res = func.call(scope,recv.into(),&args);
            println!("Result {:?}",res);
        };
    }
}