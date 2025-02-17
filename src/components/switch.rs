use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use leptos::{ev, leptos_dom::helpers::window_event_listener};

#[derive(Clone)]
pub struct SwitchLabelText {
    pub label_text: String,
    pub medium_text: String,
    pub small_text: String,
}
impl SwitchLabelText {
    pub fn new(label_text: String, medium_text: String, small_text: String) -> Self {
        Self {
            label_text,
            medium_text,
            small_text,
        }
    }
}


#[component]
pub fn Switch<'a>(
    checked: ReadSignal<bool>,
    on_toggle: WriteSignal<bool>,
    label: SwitchLabelText,
    #[prop(optional)] id: Option<&'a str>,
) -> impl IntoView + 'a{
    let input_id = id.unwrap_or_else(|| "switch");
    let width = RwSignal::new(
        window()
            .inner_width()
            .unwrap()
            .as_f64()
            .unwrap_or(0.0),
    );

    let label_text = RwSignal::new(label.label_text.clone());

    // Listen for window resize events and update the width signal accordingly.
    window_event_listener(ev::resize, move |_event| {
        let win = window();
            if let Ok(inner_width) = win.inner_width() {
                if let Some(w) = inner_width.as_f64() {
                    width.set(w);
                }
            }
        println!("Window width: {:?} and label: {}", width.get(), label_text.get());

    });
    view! {

        <div>
            <label for=input_id style="margin-right: 15px; font-size: larger; color: #f1f3f5;" >
                { move || {
                        if width.get() < 1000.0 {
                            label.small_text.clone()
                        } else if width.get() < 1200.0 {
                            label.medium_text.clone()
                        } else {
                            label.label_text.clone()
                        }
                    }
                }

            </label>
            <label class="switch" id=input_id>

                <input
                    style="margin-left: 10px;"
                    type="checkbox"
                    checked=checked
                    on:change=move |ev| {
                        let input: HtmlInputElement = ev
                            .target()
                            .unwrap()
                            .unchecked_into();
                        on_toggle.set(input.checked());
                    }
                />
                <span class="slider"></span>
            </label>
        </div>

    }
}
