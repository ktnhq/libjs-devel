pub use crate::*;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct JsStr {
    pub raw: String,
}

impl ToAny for String {
    fn to_any(self) -> JsAny {
        JsAny::JsStr(JsStr { raw: self })
    }
}

impl ToAny for &str {
    fn to_any(self) -> JsAny {
        JsAny::from(self.to_string())
    }
}

impl JsElem for JsStr {
    fn all_elems(self) -> JsAny {
        let raw = self.raw;
        let length = raw.len();
        obj! {
            "length": JsAny::from(length as f64)
        }
    }
}
