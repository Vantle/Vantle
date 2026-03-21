use wasm_bindgen::JsCast;

#[must_use]
pub fn active(event: &web_sys::KeyboardEvent) -> bool {
    event
        .target()
        .map(|target| target.unchecked_ref::<web_sys::Element>().tag_name())
        .is_some_and(|tag| tag == "INPUT" || tag == "TEXTAREA")
}
