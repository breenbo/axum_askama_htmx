use askama::Template;

use crate::base::templates::Base;

#[derive(Template)]
#[template(path = "admin/admin.html")]
pub struct AdminTemplate<'a> {
    pub base: Base<'a>,
}
