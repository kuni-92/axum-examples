use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct Request {
    name: String
}

impl Request {
    pub fn new(name : String) -> Request {
        Request {
            name,
        }
    }
}

#[derive(Serialize)]
pub struct Response {
    status: String
}

impl Response {
    pub fn new(status: String) -> Response {
        Response {
            status,
        }
    }
}
