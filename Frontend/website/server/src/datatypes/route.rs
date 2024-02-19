use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

#[derive(Deserialize, Serialize)]
pub struct Route {
    endpoint: String,
    method: Vec<Method>,
    description: String,
}

impl Route {
    pub fn new() -> Route {
        Route {
            endpoint: String::new(),
            method: Vec::new(),
            description: String::new(),
        }
    }

    pub fn from(endpoint: String, method: Vec<Method>, description: String) -> Route {
        Route { endpoint, method, description }
    }
}
