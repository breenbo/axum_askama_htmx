#[derive(Clone, Default, Debug)]
pub struct Base<'a> {
    pub tab_title: &'a str,
    pub is_logged_in: bool,
    pub is_admin: bool,
}
