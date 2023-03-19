use std::cell::RefCell;
use std::hint::unreachable_unchecked;
use std::rc::Rc;
use crate::runtime::types::{GlobalContext};

#[derive(Debug)]
pub struct IsolateState{
    context: Option<GlobalContext>
}
type RcRefIsolateState = Rc<RefCell<IsolateState>>;

impl IsolateState{
    pub fn new(context:GlobalContext) -> RcRefIsolateState {
        Rc::new(
            RefCell::new(
                Self{
                    context:Some(context)
                }
            )
        )
    }

    pub fn get(isolate:&mut v8::Isolate) -> RcRefIsolateState {
        isolate.get_slot::<RcRefIsolateState>()
            .unwrap()
            .clone()
    }

    pub fn get_context(&self) -> GlobalContext {
        match &self.context {
            Some(ctx) => ctx.clone(),
            None => unsafe{unreachable_unchecked()}
        }
    }
}