// askama templates
use askama::Template;

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate<'a> {
    pub title: &'a str,
    pub some_data: &'a str,
}

#[derive(Template)]
#[template(path = "ideas.html")]
pub struct IdeasTemplate {
    pub title: String,
    pub ideas: Vec<String>,
}
