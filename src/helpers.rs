use rand::Rng;
use wasm_bindgen::{closure::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{Event, HtmlAudioElement, HtmlImageElement};
use ::futures::channel::oneshot;
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

pub async fn play_sound(path: &str) {
    let audio_element = HtmlAudioElement::new().unwrap();
    let n_p = format!("{}/{}", "/src/assets", path);
    audio_element.set_src(&n_p);
    let play_promise = JsFuture::from(audio_element.play().unwrap());
    let _ = play_promise.await;
    let (tx, rx) = oneshot::channel();
    let tx = std::rc::Rc::new(std::cell::RefCell::new(Some(tx)));

    let closure = Closure::wrap(Box::new(move |_event: Event| {
        if let Some(tx) = tx.borrow_mut().take() {
            let _ = tx.send(());
        }
    }) as Box<dyn FnMut(_)>);

    audio_element.set_onended(Some(closure.as_ref().unchecked_ref()));
    closure.forget();

    let _ = rx.await;
    audio_element.set_onended(None);
    audio_element.set_src("");
}