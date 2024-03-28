pub use crate::*;

#[derive(Clone)]
pub struct JsCtx {
    args: JsArray,
    exports: JsAny,
    globals: JsAny,
}

impl JsCtx {
    fn new() -> JsCtx {
        JsCtx {
            args: JsArray { raw: vec![] },
            exports: obj! {},
            globals: obj! {},
        }
    }
}
