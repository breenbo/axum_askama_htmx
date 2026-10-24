// askama templates
use askama::Template;

use crate::base::templates::Base;

// import html template with askama
// send the title to the template
// props are passed to askama template with struct
#[derive(Template)]
#[template(path = "root.html")]
pub struct RootTemplate<'a> {
    pub base: Base<'a>,
}
