# Klingo

Klingo is a frontend framework that uses WebAssembly (wasm) to build user interfaces. It provides a simple and efficient way to create interactive web applications.

## Installation

To use Klingo, you can include it as a dependency in your `Cargo.toml` file:

```toml
[dependencies]
klingo = "0.1.0"
```

## Usage

Here is a simple example of how you can use Klingo to create a basic interactive UI:

```rust
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
```

In this example, we create a button and paragraph element using Klingo's `Button` and `P` modules. We also initialize a counter using the `Hook` module. The button's content and paragraph text are bound to the counter value. When the button is clicked, the counter value is incremented.

## Contributing

If you would like to contribute to Klingo, please feel free to submit issues or pull requests on our GitHub repository.

Thank you for using Klingo! Happy coding! 🚀
