use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::MouseEvent;

use crate::{app::Context, event::Hook};

#[derive(Clone)]
pub struct P {
    ele: web_sys::Element,
}

impl P {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn set_content<T: ToString + Clone>(&self, content: &mut Hook<T>) {
        content.keep_bind(&self.ele);
        self.ele.set_text_content(Some(&content.to_string()));
    }

    pub fn value(&self) -> String {
        self.ele.text_content().unwrap()
    }

    pub fn on_click(&self, f: impl FnMut(MouseEvent) + 'static) {
        let callback = Closure::wrap(Box::new(f) as Box<dyn FnMut(_)>);
        self.ele
            .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
            .unwrap();
        callback.forget();
    }
}

impl Default for P {
    fn default() -> Self {
        let ctx = Context::new();
        let ele = ctx.create_element("p");
        ctx.append_child(&ele);
        Self { ele }
    }
}
