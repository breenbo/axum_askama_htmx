// askama templates
use askama::Template;

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate<'a> {
    pub tab_title: &'a str,
    pub is_logged_in: bool,
}
