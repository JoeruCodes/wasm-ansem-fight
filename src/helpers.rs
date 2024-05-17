use rand::Rng;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

use crate::Game;
pub fn document_get_element_by_id() -> HtmlImageElement
{
    let window = web_sys::window().expect("global window does not exists");    
    let document = window.document().expect("expecting a document on window");
    //let body = document.body().expect("document expect to have have a body");
    let val = document.get_element_by_id("gameImageId")
    .unwrap()
    .dyn_into::<web_sys::HtmlImageElement>()
    .unwrap();
    // web_sys::console::log_2(&"URL: %s".into(),&JsValue::from_str(&val.inner_text()));
    return val
}


pub fn shuffle_array<'a, T>(array: &'a mut [T]) -> &'a mut [T] {
    let mut rng = rand::thread_rng();
    let len = array.len();
    for i in (1..len).rev() {
        let j = rng.gen_range(0..=i);
        array.swap(i, j);
    }
    array
}
pub fn generate_punches(min: &usize, max: &usize) -> usize{
    rand::thread_rng().gen_range(*min..*max)
}