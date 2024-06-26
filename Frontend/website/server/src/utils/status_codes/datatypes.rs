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
    pub fn from(endpoint: String, method: Vec<Method>, description: String) -> Route {
        Route { endpoint, method, description }
    }
}
