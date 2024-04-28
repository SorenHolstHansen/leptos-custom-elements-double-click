use custom_elements::CustomElement;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
fn my_comp() -> impl IntoView {
    view! {
        <button on:click=move |_| console_log("First")>"First"</button>
    }
}

#[derive(Default)]
struct FirstComponent;

impl CustomElement for FirstComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        mount_to(this.clone(), MyComp);
    }
}

#[wasm_bindgen]
pub fn run() {
    FirstComponent::define("first-elem");
}
