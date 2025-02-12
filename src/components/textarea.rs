use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[component]
pub fn InputTextArea(
    input_text: ReadSignal<String>,
    on_input: impl Fn(String) -> () + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <textarea
            style="flex: 1; margin: 10px; max-height: 80vh;"
            on:input=move |ev| on_input(event_target_value(&ev))
            prop:value=input_text
            placeholder="Enter valid xsd content here..."
        ></textarea>
    }
}

#[component]
pub fn OutputTextArea(
    output_text: ReadSignal<String>,
) -> impl IntoView {
    view! {
        <div style="position: relative; flex: 1; margin: 10px; max-height: 80vh;">
            <textarea
                style="width: 100%; height: 100%;"
                readonly
                prop:value=output_text
                placeholder="Transformed schema will appear here..."
            ></textarea>
            <button
                style="
                    position: absolute;
                    top: 1px;
                    right: 1px;
                    background: none;
                    border: none;
                    cursor: pointer;
                    padding: 2px;
                    margin-top: 5px;
                    margin-right: 5px;
                "
                on:click=move |_| {
                    let text = output_text.get();
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            let clipboard = window.navigator().clipboard();
                            let promise = clipboard.write_text(&text);
                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
                        }
                    });
                }
            >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                    <path d="M16 1H4C2.9 1 2 1.9 2 3V17H4V3H16V1ZM19 5H8C6.9 5 6 5.9 6 7V21C6 22.1 6.9 23 8 23H19C20.1 23 21 22.1 21 21V7C21 5.9 20.1 5 19 5ZM19 21H8V7H19V21Z"/>
                </svg>
            </button>
        </div>
    }
}