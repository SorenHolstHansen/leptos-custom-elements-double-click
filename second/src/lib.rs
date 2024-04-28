use custom_elements::CustomElement;
use leptos::leptos_dom::logging::console_log;
use leptos::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlElement;

#[component]
fn my_comp() -> impl IntoView {
    view! {
        <button on:click=move |_| console_log("Second")>"Second"</button>
    }
}

#[derive(Default)]
struct SecondComponent;

impl CustomElement for SecondComponent {
    fn inject_children(&mut self, this: &HtmlElement) {
        mount_to(this.clone(), MyComp);
    }
}

#[wasm_bindgen]
pub fn run() {
    SecondComponent::define("second-elem");
}
