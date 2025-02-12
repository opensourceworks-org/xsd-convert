use leptos::prelude::*;
use web_sys::{FileReader, HtmlInputElement, ProgressEvent};
use leptos::wasm_bindgen::JsCast;
use leptos::wasm_bindgen::closure::Closure;
use std::rc::Rc;

#[component]
pub fn FileInput(
    on_file_load: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <input
            type="file"
            style="height: 100%;"
            on:change=move |ev| {
                let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
                if let Some(files) = input.files() {
                    if files.length() > 0 {
                        let file = files.get(0).unwrap();
                        let reader = Rc::new(FileReader::new().unwrap());
                        let reader_clone = reader.clone();
                        let on_file_load = on_file_load.clone();
                        let onload = Closure::wrap(Box::new(move |_ev: ProgressEvent| {
                            if let Ok(result) = reader_clone.result() {
                                if let Some(text) = result.as_string() {
                                    on_file_load.set(text);
                                }
                            }
                        }) as Box<dyn FnMut(ProgressEvent)>);
                        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                        reader.read_as_text(&file).unwrap();
                        onload.forget();
                    }
                }
            }
        />
    }
}