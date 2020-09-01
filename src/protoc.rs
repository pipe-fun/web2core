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

#[derive(Serialize, Deserialize, Debug)]
pub enum ExecuteResult {
    Ok,
    DeviceOffline,
    CoreOffline,
}

impl ExecuteInfo {
    pub fn new(token: &str, command: &str) -> Self {
        Self {
            token: token.into(),
            command: command.into()
        }
    }

    pub fn get_token(&self) -> String {
        self.token.clone()
    }

    pub fn get_command(&self) -> String {
        self.command.clone()
    }
}