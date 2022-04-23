use serde::Deserialize;

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
