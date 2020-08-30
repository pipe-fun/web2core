use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation {
    Execute(ExecuteInfo),
    Reload,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExecuteInfo {
    token: String,
    command: String,
}

