pub(crate) struct Request {
    pub(crate) model: String,
    pub(crate) text: String,
}

pub(crate) struct Response {
    pub(crate) text: String,
    pub(crate) translation: String,
    pub(crate) pronunciation: String,
}

impl Request {
    fn new(model: String, text: String) -> Self {
        Self { model, text }
    }
}
