use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input as pmi,
    punctuated::Punctuated,
    Expr, LitStr, Result, Token,
};

struct Obj {
    items: Punctuated<ObjElem, Token![,]>,
}

struct ObjElem {
    is_ref: Option<Token![ref]>,
    key: LitStr,
    value: Expr,
}

impl Parse for ObjElem {
    fn parse(input: ParseStream) -> Result<Self> {
        let key: LitStr = input.parse()?;
        input.parse::<Token![:]>()?;
        let is_ref: Option<Token![ref]> = input.parse()?;
        let value: Expr = input.parse()?;
        Ok(ObjElem { key, value, is_ref })
    }
}

impl Parse for Obj {
    fn parse(input: ParseStream) -> Result<Self> {
        let items: Punctuated<ObjElem, Token![,]> =
            input.parse_terminated(ObjElem::parse, Token![,])?;
        Ok(Obj { items })
    }
}

#[proc_macro]
pub fn obj(input: TokenStream) -> TokenStream {
    let parsed = pmi!(input as Obj);
    let items = parsed.items;
    let to_extend: Vec<proc_macro2::TokenStream> = items
        .iter()
        .map(|item| {
            let is_ref = item.is_ref.is_some();
            let key = &item.key;
            let value = &item.value;
            if is_ref {
                quote! {
                    #value.iter()
                }
            } else {
                quote! {
                    HashMap::from([
                        (JsStr{ raw: #key.to_string() }, #value)
                    ])
                }
            }
        })
        .collect();
    quote! {
        {
            let mut hmap: HashMap<JsStr, JsAny> = HashMap::new();
            #(
                hmap.extend(#to_extend);
            )*
            JsAny::JsObj(JsObj{ raw: hmap })
        }
    }
    .into()
}
