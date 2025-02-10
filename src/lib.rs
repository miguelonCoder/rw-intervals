use node_bindgen::derive::node_bindgen;
use std::rc::Rc;

mod request;
mod schedule;
use request::Request;
use schedule::Schedule;


#[node_bindgen]
pub fn recalculate_reservations(sch: String, new_request: String) -> Schedule {
    let request : Request = serde_json::from_str(&new_request).unwrap();
    let mut schedule : Schedule = serde_json::from_str(&sch).unwrap();
    
    schedule.add_request(Rc::new(request));

    schedule
}

#[node_bindgen]
pub fn build_schedule(requests_string: String) -> Result<Schedule, String> {

    let requests : Vec<Request> = match serde_json::from_str(&requests_string) {
        Ok(req) => req,
        Err(e) => return Err(format!("Error parsing JSON: {}", e)),
    };

    let mut schedule = Schedule::new("a".to_string());
    
    for request in requests {
        schedule.add_request(Rc::new(request));
    }
    
    Ok(schedule)

}
