use klingo::event::Hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
fn app() {
    let btn = klingo::element::Button::new();
    let p = klingo::element::P::new();
    let mut counter = Hook::new(1);
    p.set_content(&mut counter);
    btn.set_content(&mut counter);
    btn.on_click(move |_| {
        counter.set(counter.value() + 1);
    })
}
