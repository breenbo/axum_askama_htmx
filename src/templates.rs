// askama templates
use askama::Template;

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
}

pub struct Idea {
    pub description: String,
    pub id: u16,
}

#[derive(Template)]
#[template(path = "ideas/index.html")]
pub struct IdeasTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub ideas: Vec<Idea>,
}

#[derive(Template)]
#[template(path = "ideas/show.html")]
pub struct IdeaTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub idea: Idea,
}

#[derive(Template)]
#[template(path = "ideas/edit.html")]
pub struct EditIdeaTemplate<'a> {
    pub tab_title: &'a str,
    pub title: &'a str,
    pub idea: Idea,
}
