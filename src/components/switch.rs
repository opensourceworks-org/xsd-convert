use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn Switch(
    checked: ReadSignal<bool>,
    on_toggle: WriteSignal<bool>,
) -> impl IntoView {
    view! {

        <div style="position: absolute; bottom: 30px; right: 30px; z-index: 0;">
            <label for="word-wrap-switch" style="margin-right: 15px;" >"Word Wrap"</label>
            <label class="switch" id="word-wrap-switch">

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
