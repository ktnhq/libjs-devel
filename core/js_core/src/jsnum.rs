pub use crate::*;

#[derive(Clone, PartialEq)]
pub struct JsNum {
    pub raw: f64,
}

impl ToAny for f64 {
    fn to_any(self) -> JsAny {
        JsAny::JsNum(JsNum { raw: self })
    }
}
