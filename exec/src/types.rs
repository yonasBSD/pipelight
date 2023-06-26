use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use subprocess::Popen;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Process {
    pub state: State,
    pub env: Environment,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Environment {
    pub shell: String,
    pub attached: bool,
    pub pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct State {
    pub status: Option<Status>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
