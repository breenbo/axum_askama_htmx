use askama::Template;

use crate::base::templates::Base;

pub struct Idea {
    pub description: String,
    pub id: u16,
}

#[derive(Template)]
#[template(path = "ideas/show.html")]
pub struct IdeaTemplate<'a> {
    pub base: Base<'a>,
    pub title: &'a str,
    pub idea: Idea,
}
impl<'a> Default for IdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            base: Base {
                tab_title: "Awesome idea",
                ..Default::default()
            },
            title: "Awesome Axum Idea",
            idea: Idea {
                description: String::new(),
                id: 0,
            },
        }
    }
}

#[derive(Template)]
#[template(path = "ideas/index.html")]
pub struct IdeasTemplate<'a> {
    pub base: Base<'a>,
    pub title: &'a str,
    pub ideas: Vec<Idea>,
}
impl<'a> Default for IdeasTemplate<'a> {
    fn default() -> Self {
        Self {
            base: Base {
                tab_title: "Ideas",
                ..Default::default()
            },
            title: "Axum Awesome Ideas",
            ideas: Vec::new(),
        }
    }
}

#[derive(Template)]
#[template(path = "ideas/create.html")]
pub struct CreateIdeaTemplate<'a> {
    pub base: Base<'a>,
    pub title: &'a str,
    pub error: Option<String>, // manage the validation error in the form
    pub description: String,
}
impl<'a> Default for CreateIdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            base: Base {
                tab_title: "Create idea",
                ..Default::default()
            },
            title: "Axum Create Idea",
            error: None,
            description: String::new(),
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
    pub base: Base<'a>,
    pub title: &'a str,
    pub idea: Idea,
    pub error: Option<String>,
}
impl<'a> Default for EditIdeaTemplate<'a> {
    fn default() -> Self {
        Self {
            base: Base {
                tab_title: "Edit idea",
                ..Default::default()
            },
            title: "Axum Edit Idea",
            idea: Idea {
                description: String::new(),
                id: 0,
            },
            error: None,
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
