pub use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsBool {
    pub raw: bool,
}

impl ToAny for bool {
    fn to_any(self) -> JsAny {
        JsAny::JsBool(JsBool { raw: self })
    }
}
