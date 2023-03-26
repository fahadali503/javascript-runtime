use std::cell::RefCell;
use std::rc::Rc;
use crate::runtime::module::ModuleMap;

pub struct IsolateState{
    context:Option<v8::Global<v8::Context>>,
    pub module_map:ModuleMap
}

impl IsolateState{
    pub fn new(context:v8::Global<v8::Context>,) -> Rc<RefCell<IsolateState>>{
        Rc::new(
            RefCell::new(
                IsolateState{
                    context: Some(context),
                    module_map:ModuleMap::new()
                }
            )
        )
    }

    pub fn get(isolate: &mut v8::Isolate) -> Rc<RefCell<Self>>{
        let state = isolate.get_slot::<Rc<RefCell<IsolateState>>>().unwrap();
        Rc::clone(state)
    }

    pub fn context(&self) -> v8::Global<v8::Context>{
        match &self.context {
            Some(context) => context.clone(),
            None => unreachable!()
        }
    }
}