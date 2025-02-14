use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture;
use crate::components::switch::Switch;

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
            placeholder="Paste here or choose a local file with valid xsd content ..."
        ></textarea>
    }
}

#[component]
pub fn OutputTextArea(
    output_text: ReadSignal<String>,
) -> impl IntoView {
    let (notification, set_notification) = signal(None::<String>);
    let (word_wrap, set_word_wrap) = signal(true);
    view! {
        <div style="position: relative; flex: 1; margin: 10px; max-height: 80vh;">
            <textarea
                style="position: absolute; left: 0; width: 100%; height: 100%; max-height: 80vh; z-index: 1;"
                class=move || if word_wrap.get() { "word-wrap" } else { "" }
                wrap=move || if word_wrap.get() { "soft" } else { "off" }
                readonly
                prop:value=output_text
                placeholder="Transformed schema will appear here..."
            ></textarea>

            <Switch
                    checked=word_wrap on_toggle=set_word_wrap/>

            <button class="text-area-copy-button"

                on:click=move |_| {
                    let text = output_text.get();
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            let clipboard = window.navigator().clipboard();
                            let promise = clipboard.write_text(&text);
                            let _ = wasm_bindgen_futures::JsFuture::from(promise).await;

                            set_notification.set(Some("Copied to clipboard!".to_string()));
                            TimeoutFuture::new(2000).await;
                            set_notification.set(None);
                        }
                    });
                }
            >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                    <path d="M16 1H4C2.9 1 2 1.9 2 3V17H4V3H16V1ZM19 5H8C6.9 5 6 5.9 6 7V21C6 22.1 6.9 23 8 23H19C20.1 23 21 22.1 21 21V7C21 5.9 20.1 5 19 5ZM19 21H8V7H19V21Z"/>
                </svg>
            </button>
            {move || (move || {
                if let Some(msg) = notification.get() {
                    view! {
                        // If there's a message, render the styled notification div with the message
                        <div style="position: absolute; bottom: 10px; right: 10px;
                                    background-color: rgba(85, 64, 64, 0.75); color: white;
                                    padding: 5px 10px; border-radius: 5px;">
                            {msg}
                        </div>
                    }
                } else {
                    view! {
                        // Else, render a div with the same style attributes and an empty string as its child.
                        <div style="position: absolute; bottom: 10px; right: 10px;
                                    background-color: transparent; color: transparent;
                                    padding: 5px 10px; border-radius: 5px;">
                            {"".to_string()}
                        </div>
                    }
                }
            })()}
        </div>
    }
}