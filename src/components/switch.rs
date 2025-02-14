use leptos::prelude::*;
use leptos::wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn Switch<'a>(
    checked: ReadSignal<bool>,
    on_toggle: WriteSignal<bool>,
    label: &'a str,
    #[prop(optional)] id: Option<&'a str>,
) -> impl IntoView + 'a{
    let input_id = id.unwrap_or_else(|| "switch");
    view! {

        <div>
            <label for=input_id style="margin-right: 15px; font-size: larger;" >{label}</label>
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
