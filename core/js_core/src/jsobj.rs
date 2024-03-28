pub use crate::*;

#[derive(Clone)]
pub struct JsObj {
    pub raw: std::collections::HashMap<JsStr, JsAny>,
}

impl JsElem for JsObj {
    fn all_elems(self) -> JsAny {
        JsAny::JsObj(self)
    }
}

impl ToAny for std::collections::HashMap<JsStr, JsAny> {
    fn to_any(self) -> JsAny {
        JsAny::JsObj(JsObj { raw: self })
    }
}
