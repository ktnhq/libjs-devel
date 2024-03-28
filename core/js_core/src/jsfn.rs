pub use crate::*;

#[derive(Clone)]
pub struct JsFn {
    pub name: String,
    pub raw: fn(&mut JsCtx),
}
