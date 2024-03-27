pub use js_core_macros::obj;
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::{Add, Div, Mul, Sub};

// TODO: Add JsBigInt
// TODO: Add JsSymbol
pub enum JsAny {
    JsStr(JsStr),
    JsNum(JsNum),
    JsBool(JsBool),
    JsFn(JsFn),
    JsArray(JsArray),
    JsObj(JsObj),
    JsUndefined,
    JsNull,
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

impl Add for JsAny {
    type Output = Self;

    fn add(self, o: Self) -> Self {
        // TODO: Implement everything
        // String + String == String (e.g "a" + "b" == "ab")
        // String + Number == String (e.g "a" + 1 == "a1")
        // Number + String == String (e.g 1 + "b" == "1b")
        // Number + Number == Number (e.g 1 + 1 == 2)
        // Boolean + Number == Number (e.g true + 1 == 2)
        // Number + Boolean == Number (e.g 1 + false == 1)
        // Boolean + Boolean == Number (e.g true + false == 1)
        // _ + _ == String (e.g {} + "a" == "[object Object]a")
        match (&self, &o) {
            (JsAny::JsStr(s), JsAny::JsStr(o)) => JsAny::from(format!("{}{}", s.raw, o.raw)),
            (JsAny::JsStr(s), JsAny::JsNum(o)) => JsAny::from(format!("{}{}", s.raw, o.raw)),
            (JsAny::JsNum(s), JsAny::JsStr(o)) => JsAny::from(format!("{}{}", s.raw, o.raw)),
            (JsAny::JsNum(n), JsAny::JsNum(o)) => JsAny::from(n.raw + o.raw),
            (JsAny::JsBool(b), JsAny::JsNum(o)) => JsAny::from({
                match b.raw {
                    true => 1 as f64 + o.raw,
                    false => 0 as f64 + o.raw,
                }
            }),
            (JsAny::JsNum(n), JsAny::JsBool(o)) => JsAny::from({
                match o.raw {
                    true => 1 as f64 + n.raw,
                    false => 0 as f64 + n.raw,
                }
            }),
            (JsAny::JsBool(b), JsAny::JsBool(o)) => JsAny::from({
                let a = match b.raw {
                    true => 1,
                    false => 0,
                };
                let b = match o.raw {
                    true => 1,
                    false => 0,
                };
                a as f64 + b as f64
            }),
            (_, _) => JsAny::from({
                let a = self.to_string();
                let b = o.to_string();
                match (a, b) {
                    (JsAny::JsStr(a), JsAny::JsStr(b)) => {
                        format!("{}{}", a.raw, b.raw)
                    }
                    (_, _) => format!("undefinedundefined"),
                }
            }),
        }
    }
}

pub trait JsElem {
    fn all_elems(self) -> JsAny;
}

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

impl ToAny for f64 {
    fn to_any(self) -> JsAny {
        JsAny::JsNum(JsNum { raw: self })
    }
}

impl ToAny for bool {
    fn to_any(self) -> JsAny {
        JsAny::JsBool(JsBool { raw: self })
    }
}

impl ToAny for Vec<JsAny> {
    fn to_any(self) -> JsAny {
        JsAny::JsArray(JsArray { raw: self })
    }
}

pub struct JsArray {
    pub raw: Vec<JsAny>,
}

pub struct JsCtx {
    args: JsArray,
    exports: JsAny,
    globals: JsAny,
}

#[derive(PartialEq, Eq, Hash)]
pub struct JsStr {
    pub raw: String,
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

#[derive(PartialEq)]
pub struct JsNum {
    pub raw: f64,
}

impl std::cmp::Eq for JsNum {}

#[derive(PartialEq, Eq, Hash)]
pub struct JsBool {
    pub raw: bool,
}

pub struct JsFn {
    pub name: String,
    pub raw: Box<dyn Fn(&mut JsCtx)>,
}

impl JsAny {
    pub fn from(src: impl ToAny) -> JsAny {
        src.to_any()
    }

    pub fn new() -> JsAny {
        JsAny::JsUndefined
    }

    pub fn type_of(&self) -> JsAny {
        match self {
            JsAny::JsStr(_) => JsAny::from("string"),
            JsAny::JsNum(_) => JsAny::from("number"),
            JsAny::JsBool(_) => JsAny::from("boolean"),
            JsAny::JsArray(_) => JsAny::from("object"),
            JsAny::JsFn(_) => JsAny::from("function"),
            JsAny::JsUndefined => JsAny::from("undefined"),
            JsAny::JsNull => JsAny::from("null"),
            _ => JsAny::from("undefined"),
        }
    }

    pub fn to_string(self) -> JsAny {
        match self {
            JsAny::JsStr(_) => self,
            JsAny::JsNum(n) => JsAny::from(format!("{}", n.raw)),
            JsAny::JsBool(b) => JsAny::from(format!("{}", b.raw)),
            JsAny::JsUndefined => JsAny::from("undefined"),
            JsAny::JsNull => JsAny::from("null"),
            JsAny::JsArray(_) => JsAny::from("[object Object]"),
            JsAny::JsObj(_) => JsAny::from("[object Object]"),
            JsAny::JsFn(f) => JsAny::from(format!("function {}() {{ [native code] }}", f.name)),
        }
    }

    pub fn to_rs_string(self) -> String {
        match self.to_string() {
            JsAny::JsStr(s) => s.raw,
            _ => "".to_string(),
        }
    }
}

impl ToAny for (String, Box<dyn Fn(&mut JsCtx)>) {
    fn to_any(self) -> JsAny {
        JsAny::JsFn(JsFn {
            name: self.0,
            raw: self.1,
        })
    }
}

pub trait ToAny {
    fn to_any(self) -> JsAny;
}
