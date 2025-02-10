use std::{collections::{HashMap, HashSet}, rc::Rc};
use crate::request::Request;
use node_bindgen::{core::{val::{JsEnv, JsObject}, NjError, TryIntoJs}, sys::napi_value};
use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize)]
pub struct Schedule {
  pub name: String,
  pub overlaps: HashSet<String>,
  #[serde(deserialize_with = "deserialize_requests")]
  pub requests: Vec<Rc<Request>>,
  //#[serde(deserialize_with = "deserialize_requests")]
  pub reservations: Vec<String>,
}

impl Schedule {
  pub fn new(name: String) -> Self {
      Schedule {
          name,
          overlaps: HashSet::new(),
          requests: Vec::new(),
          reservations: Vec::new(),
      }
  }

  pub fn add_request(&mut self, new_request: Rc<Request>) {
      self.update_overlaps(new_request.clone());
      self.insert_request(new_request);
      self.recalculate_reservations();
  }

  fn insert_request(&mut self, new_request: Rc<Request>) {
      let pos = self.requests
          .binary_search_by(|request|{
              request.weight.cmp(&new_request.weight).reverse()
          }).unwrap_or_else(|e| e);

      self.requests.insert(pos, new_request);
  }

  fn update_overlaps(&mut self, new_request: Rc<Request>) {
      self.requests
          .iter()
          .filter(|request| new_request.is_overlap((*request).clone()))
          .for_each(|request| {
              self.overlaps
                  .insert(String::from(new_request.id.clone() + ":" + &(*request).id));
          })
  }

  fn recalculate_reservations(&mut self) {
      let requests = self.requests.clone();
      let mut request_state = vec![true; requests.len()];


      for i in 0..requests.len() {
          if request_state[i] {
              self.overlaps
                  .iter()
                  .filter(|overlap| { overlap.contains(&requests[i].id) })
                  .map(|overlap| {
                      if let Some((first, last)) = overlap.split_once(':') {
                          if first == &requests[i].id {
                              last
                          } else {
                              first
                          }
                      } else {
                          ""
                      }
                  })
                  .for_each(|request_id| {
                      if let Some(pos) = requests.iter().position(|request| request.id == request_id) {
                          request_state[pos]= false;
                      }

                  })
          }
      }

      let reservations: Vec<String> = requests
          .into_iter()
          .enumerate()
          .filter(|(i, _)| request_state[*i])
          .map(|(_, request)| request.clone().id.clone())
          .collect();

      
      self.reservations = reservations;

  }
}





/**
 * This function indicates how to deserialize requests and reservations properties in the Schedule struct
 */
fn deserialize_requests<'de, D>(deserializer: D) -> Result<Vec<Rc<Request>>, D::Error>
where
    D: Deserializer<'de>,
{
    let requests: Vec<Request> = Deserialize::deserialize(deserializer)?;
    let mut request_map: HashMap<String, Rc<Request>> = HashMap::new();
    let mut rc_requests = Vec::new();

    for request in requests {
        let rc_request = request_map.entry(request.id.clone()).or_insert_with(|| Rc::new(request));
        rc_requests.push(Rc::clone(rc_request));
    }

    Ok(rc_requests)
}

/**
 * This implementation indicate how to serialize Schedule struct into json object.
 */

impl Serialize for Schedule {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
      where
          S: serde::Serializer, 
          
      {
        let mut state = serializer.serialize_struct("Schedule", 4)?;

        state.serialize_field("name", &self.name)?;
        state.serialize_field("overlaps", &self.overlaps)?;

        let requests : Vec<&Request> = self.requests.iter().map(|r| r.as_ref()).collect();
        state.serialize_field("requests", &requests)?;

        
        state.serialize_field("reservations", &self.reservations)?;

        state.end()
      }

}

impl TryIntoJs for Schedule {
    fn try_to_js(self, env: &JsEnv) -> 
    Result<napi_value, NjError> {
        let mut obj = JsObject::create(env)?;

        let name_js = self.name.try_to_js(env)?;
        obj.set_property("name", name_js)?;

        let my_vec: Vec<Result<napi_value, NjError>> = self.requests
        .iter()
        .map(|item| {
            item.as_ref().clone().try_to_js(env)
        })
        .collect();

        let requests_js = my_vec.try_to_js(env)?;
        obj.set_property("requests", requests_js)?;

        let overlaps: Vec<Result<napi_value, NjError>> = self.overlaps
            .iter()
            .map(|o| o.clone().try_to_js(env))
            .collect();

        let overlaps_js = overlaps.try_to_js(env)?;
        obj.set_property("overlaps", overlaps_js)?;

        let reservations: Vec<Result<napi_value, NjError>> = self.reservations
        .iter()
        .map(|item| {
            item.clone().try_to_js(env)
        })
        .collect();

        let reservations_js = reservations.try_to_js(env)?;
        obj.set_property("reservations", reservations_js)?;

        Ok(obj.napi_value())
    }

    
}

