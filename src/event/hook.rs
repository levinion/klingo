use web_sys::Element;

#[derive(Clone)]
pub struct Hook<T: Clone + ToString> {
    value: T,
    bind: Vec<Element>,
}

impl<T: Clone + ToString> Hook<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            bind: vec![],
        }
    }

    pub fn keep_bind(&mut self, ele: &Element) {
        let ele = ele.clone();
        self.bind.push(ele);
    }

    pub fn value(&self) -> T {
        self.value.clone()
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
        let content = self.value.to_string();
        self.bind.iter().for_each(|ele| {
            ele.set_text_content(Some(&content));
        })
    }
}

#[allow(clippy::to_string_trait_impl)]
impl<T: Clone + ToString> ToString for Hook<T> {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
