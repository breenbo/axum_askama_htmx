use askama::Template;

use crate::base::templates::Base;

#[derive(Template)]
#[template(path = "auth/Login.html")]
pub struct LoginTemplate<'a> {
    pub base: Base<'a>,
}

#[derive(Template)]
#[template(path = "auth/Register.html")]
pub struct RegisterTemplate<'a> {
    pub base: Base<'a>,
    pub name: String,
    pub email: String,
    pub error: Option<&'a str>,
}

impl<'a> Default for RegisterTemplate<'a> {
    fn default() -> Self {
        Self {
            base: Base {
                tab_title: "Register",
                ..Default::default()
            },
            name: String::new(),
            email: String::new(),
            error: None,
        }
    }
}
