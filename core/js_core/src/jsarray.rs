pub use crate::*;

#[derive(Clone)]
pub struct JsArray {
    pub raw: Vec<JsAny>,
}

impl ToAny for Vec<JsAny> {
    fn to_any(self) -> JsAny {
        JsAny::JsArray(JsArray { raw: self })
    }
}
