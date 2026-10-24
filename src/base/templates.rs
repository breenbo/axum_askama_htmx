#[derive(Clone, Default)]
pub struct Base<'a> {
    pub tab_title: &'a str,
    pub is_logged_in: bool,
}
