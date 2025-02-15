// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use js_sys::{Object, Reflect};
use wasm_bindgen::convert::RefFromWasmAbi;
use wasm_bindgen::prelude::*;

/// A really hacky and dirty code to downcast a `JsValue` to `T:
/// RefFromWasmAbi`, inspired by
/// https://github.com/rustwasm/wasm-bindgen/issues/2231#issuecomment-656293288.
///
/// The returned value is likely to be a `wasm_bindgen::__ref::Ref<T>`.
pub fn downcast<T>(value: &JsValue, classname: &str) -> Result<T::Anchor, JsError>
where
    T: RefFromWasmAbi<Abi = u32>,
{
    let constructor_name = Object::get_prototype_of(value).constructor().name();

    if constructor_name == classname {
        let pointer = Reflect::get(value, &JsValue::from_str("ptr"))
            .map_err(|_| JsError::new("Failed to read the `JsValue` pointer"))?;
        let pointer = pointer
            .as_f64()
            .ok_or_else(|| JsError::new("Failed to read the `JsValue` pointer as a `f64`"))?
            as u32;

        Ok(unsafe { T::ref_from_abi(pointer) })
    } else {
        Err(JsError::new(&format!(
            "Expect an `{classname}` instance, received `{constructor_name}` instead",
        )))
    }
}
