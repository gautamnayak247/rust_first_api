use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::{from_value, to_value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Employee {
    pub id: u32,
    pub name: String,
    pub role: String,
}

#[wasm_bindgen]
pub fn create_employee(payload: JsValue) -> JsValue {
    match (|| -> Result<JsValue, String> {
        let emp: Employee = from_value(payload).map_err(|e| format!("parse error: {}", e))?;
        to_value(&emp).map_err(|e| format!("serialize error: {}", e))
    })() {
        Ok(val) => val,
        Err(e) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"error".into(), &JsValue::from_str(&e)).unwrap();
            obj.into()
        }
    }
}

#[wasm_bindgen]
pub fn update_employee(payload: JsValue) -> JsValue {
    match (|| -> Result<JsValue, String> {
        let _emp: Employee = from_value(payload).map_err(|e| format!("parse error: {}", e))?;
        Ok(JsValue::from_str("ok"))
    })() {
        Ok(val) => val,
        Err(e) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"error".into(), &JsValue::from_str(&e)).unwrap();
            obj.into()
        }
    }
}

#[wasm_bindgen]
pub fn get_employee() -> JsValue {
    let emp = Employee {
        id: 1,
        name: "Gautam".to_string(),
        role: "Permanent".to_string(),
    };
    match to_value(&emp) {
        Ok(val) => val,
        Err(e) => {
            let obj = js_sys::Object::new();
            js_sys::Reflect::set(&obj, &"error".into(), &JsValue::from_str(&format!("serialize error: {}", e))).unwrap();
            obj.into()
        }
    }
}
