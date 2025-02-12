use leptos::prelude::*;
use leptos::wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;
use leptos::wasm_bindgen::JsCast;
use leptos::html::Div;

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
    let language = "json";
    let highlighted = create_memo(move |_| {
        let raw_code = output_text.get();
        let window = web_sys::window().expect("no global window exists");
        // Get the global Prism object.
        let prism = js_sys::Reflect::get(&window, &JsValue::from_str("Prism"))
            .expect("Prism not found");
        // Get and cast the "highlight" property to a Function.
        let highlight_fn = js_sys::Reflect::get(&prism, &JsValue::from_str("highlight"))
            .expect("Prism.highlight not found")
            .dyn_into::<js_sys::Function>()
            .expect("Prism.highlight is not a function");

        let args = js_sys::Array::new();
        args.push(&JsValue::from_str(&raw_code));
        // Get the language grammar from Prism.languages.
        let languages = js_sys::Reflect::get(&prism, &JsValue::from_str("languages"))
            .expect("Prism.languages not found");
        let grammar = js_sys::Reflect::get(&languages, &JsValue::from_str(&language))
            .expect("grammar not found for language");
        args.push(&grammar);
        args.push(&JsValue::from_str(&language));

        let result = highlight_fn
            .apply(&prism, &args)
            .expect("highlight.apply failed");
        result.as_string().unwrap_or(raw_code)
    });
    view! {
        <div style="position: relative; flex: 1; margin: 10px; max-height: 80vh;">
            <pre style="width: 100%; height: 100%; position: absolute; top: 0; left: 0; z-index: 0; background: transparent;
                        text-align: left; overflow-y: scroll; padding: 10px;"
                        >
                <code style="white-space: pre-wrap; word-wrap: break-word;"
                      class=format!("language-{}", language)
                      inner_html= move || highlighted.get()

                ></code>
            </pre>
            <textarea
                style="width: 100%; height: 100%; position: absolute; top: 0; left: 0;
                        z-index: 1;"
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
                    z-index: 100;
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