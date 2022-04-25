use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
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
    pub fn new(status: &str) -> Response {
        let status = String::from(status);
        Response {
            status,
        }
    }

    pub fn to_string(&self) -> Result<String, String> {
        match serde_json::to_string(&self) {
            Ok(s) => Ok(s),
            Err(_) => Err(String::from("Could not parse json.")),
        }
    }
}
