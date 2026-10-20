use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate)]
pub struct IdeaForm {
    #[validate(length(min = 10, message = "Must be at least 10 chars long"))]
    pub description: String,
}
