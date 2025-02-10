use std::rc::Rc;
use node_bindgen::{core::{val::{JsEnv, JsObject}, NjError, TryIntoJs}, sys::napi_value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Request {
    pub id: String,
    pub start_date: i64,
    pub end_date: i64,
    pub weight: i16,
}

impl Request {
    pub fn is_overlap(&self, b: Rc<Request>) -> bool {
        self.start_date < b.end_date && b.start_date < self.end_date
    }
}

impl TryIntoJs for Request {
    fn try_to_js(self, env: &JsEnv) -> Result<napi_value, NjError> {
        let mut js_object = JsObject::create(env)?;

        let id_js = self.id.try_to_js(env)?;
        js_object.set_property("id", id_js)?;

        let start_date_js = self.start_date.try_to_js(env)?;
        js_object.set_property("start_date", start_date_js)?;

        let end_date_js = self.end_date.try_to_js(env)?;
        js_object.set_property("end_date", end_date_js)?;

        let weight_js = self.weight.try_to_js(env)?;
        js_object.set_property("weight", weight_js)?;

        Ok(js_object.napi_value())

    }
}
