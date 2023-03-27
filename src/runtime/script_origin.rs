
pub fn create_script_origin<'a>(scope:&mut v8::HandleScope<'a>,
    resource_name:&str,
    is_module:bool) -> v8::ScriptOrigin<'a>{

    let resource = v8::String::new(scope,resource_name).unwrap();
    let  source_map_url =  v8::String::new(scope,resource_name).unwrap();

    let script_origin = v8::ScriptOrigin::new(scope,resource.into(),0,0,
    false,0,source_map_url.into(),false,false,is_module);
    script_origin
}