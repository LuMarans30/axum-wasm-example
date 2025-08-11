use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");

    let div = document.create_element("div")?;
    div.set_class_name("center-align");

    let image: web_sys::HtmlImageElement = HtmlImageElement::new()?;
    image.set_width(512);
    image.set_height(512);
    image.set_src("https://www.rustacean.net/assets/rustacean-flat-happy.svg");

    let message = document.create_element("h5")?;
    message.set_inner_html("Hello from Rust generated WASM!");

    let element = document
        .get_element_by_id("container")
        .expect("document should have an element with id #container");

    div.append_child(&image)?;
    div.append_child(&message)?;

    element.append_child(&div)?;

    Ok(())
}
