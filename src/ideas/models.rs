// forms models/structs
//
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct IdeaForm {
    pub idea: String,
}
