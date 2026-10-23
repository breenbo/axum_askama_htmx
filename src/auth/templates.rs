use askama::Template;

#[derive(Template)]
#[template(path = "auth/Login.html")]
pub struct LoginTemplate<'a> {
    pub tab_title: &'a str,
    pub is_logged_in: bool,
}

#[derive(Template)]
#[template(path = "auth/Register.html")]
pub struct RegisterTemplate<'a> {
    pub tab_title: &'a str,
    pub name: String,
    pub email: String,
    pub error: Option<&'a str>,
    pub is_logged_in: bool,
}

impl<'a> Default for RegisterTemplate<'a> {
    fn default() -> Self {
        Self {
            tab_title: "Register",
            name: String::new(),
            email: String::new(),
            error: None,
            is_logged_in: false,
        }
    }
}
