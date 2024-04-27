use web_sys::{Document, Element};

pub struct Context {
    document: Document,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_element(&self, name: &str) -> Element {
        self.document.create_element(name).unwrap()
    }

    pub fn append_child(&self, ele: &web_sys::Element) {
        let body = self.document.body().unwrap();
        body.append_child(ele).unwrap();
    }
}

impl Default for Context {
    fn default() -> Self {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        Self { document }
    }
}
