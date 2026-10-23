use askama::Template;

pub struct Idea {
    pub description: String,
    pub id: u16,
}

#[derive(Template)]
#[template(path = "ideas/show.html")]
pub struct IdeaTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub idea: Idea,
    pub is_logged_in: bool,
}
impl<'a> Default for IdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            tab_title: "Axum Idea",
            title: "Awesome Axum Idea",
            idea: Idea {
                description: String::new(),
                id: 0,
            },
            is_logged_in: false,
        }
    }
}

#[derive(Template)]
#[template(path = "ideas/index.html")]
pub struct IdeasTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub ideas: Vec<Idea>,
    pub is_logged_in: bool,
}
impl<'a> Default for IdeasTemplate<'a> {
    fn default() -> Self {
        Self {
            tab_title: "Axum Awesome Ideas",
            title: "Axum Awesome Ideas",
            ideas: Vec::new(),
            is_logged_in: false,
        }
    }
}

#[derive(Template)]
#[template(path = "ideas/create.html")]
pub struct CreateIdeaTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub error: Option<String>, // manage the validation error in the form
    pub description: String,
    pub is_logged_in: bool,
}
impl<'a> Default for CreateIdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            tab_title: "Axum Create Idea",
            title: "Axum Create Idea",
            error: None,
            description: String::new(),
            is_logged_in: false,
        }
    }
}
impl<'a> CreateIdeaTemplate<'a> {
    pub fn with_error_message(error_message: String, description: String) -> Self {
        Self {
            error: Some(error_message),
            description,
            ..Default::default()
        }
    }
}

#[derive(Template)]
#[template(path = "ideas/edit.html")]
pub struct EditIdeaTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub idea: Idea,
    pub error: Option<String>,
    pub is_logged_in: bool,
}
impl<'a> Default for EditIdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            tab_title: "Axum Edit Idea",
            title: "Axum Edit Idea",
            idea: Idea {
                description: String::new(),
                id: 0,
            },
            error: None,
            is_logged_in: false,
        }
    }
}
impl<'a> EditIdeaTemplate<'a> {
    pub fn with_error_message(error_message: String, idea: Idea) -> Self {
        Self {
            error: Some(error_message),
            idea,
            ..Default::default()
        }
    }
}
