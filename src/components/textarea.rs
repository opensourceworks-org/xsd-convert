use leptos::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture;
use leptos::wasm_bindgen::JsValue;
use leptos::wasm_bindgen::JsCast;

fn highlightify(raw_code: &str, language: &str) -> String {
    let window = web_sys::window().expect("no global window exists");
    let prism = js_sys::Reflect::get(&window, &JsValue::from_str("Prism"))
        .expect("Prism not found");
    let highlight_fn = js_sys::Reflect::get(&prism, &JsValue::from_str("highlight"))
        .expect("Prism.highlight not found")
        .dyn_into::<js_sys::Function>()
        .expect("Prism.highlight is not a function");

    let args = js_sys::Array::new();
    args.push(&JsValue::from_str(&raw_code));
    let languages = js_sys::Reflect::get(&prism, &JsValue::from_str("languages"))
        .expect("Prism.languages not found");
    let grammar = js_sys::Reflect::get(&languages, &JsValue::from_str(&language))
        .expect("grammar not found for language");
    args.push(&grammar);
    args.push(&JsValue::from_str(&language));

    let result = highlight_fn
        .apply(&prism, &args)
        .expect("highlight.apply failed");
    result.as_string().unwrap_or(raw_code.to_string())
}

#[component]
pub fn InputTextArea(
    input_text: ReadSignal<String>,
    on_input: impl Fn(String) -> () + Send + Sync + Clone + Copy + 'static,
    word_wrap: ReadSignal<bool>,
) -> impl IntoView {
    let language = "xml";
    let highlighted = Memo::new(move |_| {
        let raw_code = input_text.get();
        highlightify(&raw_code, &language)
    });
    view! {
        <div class="text-area-div" style="position: relative;">
            <pre class="code-style">
                <code style="display: block; width: 2000px; max-width: 100%;"
                      class=move || format!("{}language-{}", if word_wrap.get() { "word-wrap " } else { "" }, language)
                      inner_html= move || highlighted.get()
                ></code>
            </pre>
            <textarea
                class=move || format!("text-area{}", if word_wrap.get() { " word-wrap" } else { "" })
                wrap=move || if word_wrap.get() { "soft" } else { "off" }
                on:input=move |ev| on_input(event_target_value(&ev))
                prop:value=input_text
                placeholder="Paste here or choose a local file with valid xsd content ..."
            ></textarea>
            <button title="Paste from clipboard" class="text-area-paste-button"
                // Positioning the paste button in the top-right corner of the container.
                style="position: absolute; top: 5px; right: 5px;"
                on:click=move |_| {
                    let on_input = on_input.clone();
                    spawn_local(async move {
                        if let Some(window) = web_sys::window() {
                            let clipboard = window.navigator().clipboard();
                            let promise = clipboard.read_text();
                            match wasm_bindgen_futures::JsFuture::from(promise).await {
                                Ok(text_js) => {
                                    let pasted_text = text_js.as_string().unwrap_or_default();
                                    on_input(pasted_text);
                                }
                                Err(err) => {
                                    // Optionally handle errors here
                                    web_sys::console::error_1(&err);
                                }
                            }
                        }
                    });
                }
            >
                <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor"
                     xmlns="http://www.w3.org/2000/svg">
                    <path d="M19 3H14.82C14.4 1.84 13.3 1 12 1S9.6 1.84 9.18 3H5c-1.1 0-2
                             .9-2 2v14c0 1.1.9 2 2 2h14c1.1 0 2-.9 2-2V5c0-1.1-.9-2-2-2zm-7
                             0c.55 0 1 .45 1 1s-.45 1-1 1-1-.45-1-1 .45-1 1-1zm7 16H5V5h2v2h10V5h2v14z"/>
                </svg>
            </button>
        </div>
    }
}


#[component]
pub fn OutputTextArea(
    output_text: ReadSignal<String>,
    word_wrap: ReadSignal<bool>,
) -> impl IntoView {
    let (notification, set_notification) = signal(None::<String>);
    let language = "json";
    let highlighted = Memo::new(move |_| {
        let raw_code = output_text.get();
        highlightify(&raw_code, &language)
    });

    view! {
        <div class="text-area-div">
            <pre class="code-style">
                <code style="display: block; width: 2000px; max-width: 100%;"
                      class=move || format!("{}language-{}", if word_wrap.get() { "word-wrap " } else { "" }, language)
                      inner_html= move || highlighted.get()

                ></code>
            </pre>
            <textarea
                class=move || format!("text-area{}", if word_wrap.get() { " word-wrap" } else { "" })
                wrap=move || if word_wrap.get() { "soft" } else { "off" }
                readonly
                prop:value=output_text
                placeholder="Transformed schema will appear here..."
            ></textarea>

            <button title="Copy to clipboard" class="text-area-copy-button"
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
                        <div style="position: absolute; bottom: 10px; right: 10px;
                                    background-color: rgba(85, 64, 64, 0.75); color: white;
                                    padding: 5px 10px; border-radius: 5px; z-index: 100;">
                            {msg}
                        </div>
                    }
                } else {
                    view! {
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