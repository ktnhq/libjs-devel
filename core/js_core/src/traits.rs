pub use crate::*;

pub trait ToAny {
    fn to_any(self) -> JsAny;
}

pub trait JsElem {
    fn all_elems(self) -> JsAny;
}
