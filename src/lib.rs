use std::rc::Rc;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use serde_json::json;
use yaxp_core::xsdp::parser::parse_xsd_string;
use web_sys::{FileReader, HtmlInputElement, HtmlTextAreaElement};
use leptos::web_sys;
use leptos::wasm_bindgen::closure::Closure;
use leptos::wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

mod components;
mod pages;

use crate::pages::home::Home;

#[component]
pub fn App() -> impl IntoView {
    // Signals for the input text, output text, and selected action.
    let (input_text, set_input_text) = create_signal(String::new());
    let (output_text, set_output_text) = create_signal(String::new());
    let (selected_action, set_selected_action) = create_signal("arrow".to_string());

    // Your transform function (unchanged)
    let transform_text = move || {
        let input = input_text.get();
        let action = selected_action.get();

        let transformed_text = match action.as_str() {
            "spark" => {
                let spark_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_spark()
                    .unwrap()
                    .to_json()
                    .unwrap();
                serde_json::to_string_pretty(&spark_schema).unwrap()
            },
            "arrow" => {
                let arrow_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_arrow()
                    .unwrap();
                arrow_schema.to_string()
            },
            "json" => {
                let json_schema = parse_xsd_string(&input, None).unwrap();
                serde_json::to_string_pretty(&json_schema).unwrap()
            },
            "jsonschema" => {
                let json_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_json_schema();
                serde_json::to_string_pretty(&json_schema).unwrap()
            },
            "avro" => {
                let avro_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_avro()
                    .unwrap();
                serde_json::to_string(&avro_schema).unwrap()
            },
            "duckdb" => {
                let duckdb_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_duckdb_schema();
                serde_json::to_string_pretty(&duckdb_schema).unwrap()
            },
            "polars" => {
                let polars_schema = parse_xsd_string(&input, None)
                    .unwrap()
                    .to_polars();
                let fields: Vec<_> = polars_schema.iter().map(|(name, dtype)| {
                    json!({
                        "name": name.to_string(),
                        "dtype": format!("{:?}", dtype.to_string())
                    })
                }).collect();
                format!("{:?}", fields)
            },
            _ => input, // Default: no transformation
        };

        set_output_text.set(transformed_text);
    };

    view! {

        <Html />
        <Meta charset="UTF-8" />
        <Title text="<xsd ⚡> Convert xsd schemas" />
        <div style="display: flex; flex-direction: column; height: 100vh;">

            { /* Top Bar */ }
            <div style="
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                height: 50px;
                background-color: #333;
                color: #fff;
                display: flex;
                align-items: center;
                padding: 0 20px;
                white-space: nowrap;       /* Prevents wrapping */
                overflow: hidden;          /* Hides overflow */
                text-overflow: ellipsis;   /* Adds an ellipsis if text is too long */
                z-index: 1000;
                font-family: 'sans-serif';
            ">
                <p style="margin: 0; font-size: 1.1rem; font-family: monospace; color: #fff;">
                    {"<xsd "}
                    <span style="color: yellow;">{"⚡"}</span>
                    {">"}
                </p>
            </div>

            { /* Main Content Area (with top and bottom margin to avoid overlap) */ }
            <div style="flex: 1; margin-top: 60px; margin-bottom: 60px; display: flex;">
                { /* Left Textarea */ }
                <textarea
                    style="flex: 1; margin: 10px; max-height: 80vh;"
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    prop:value=input_text
                    placeholder="Enter text here..."
                ></textarea>

                { /* Right Textarea Container with Copy Icon */ }
                <div style="position: relative; flex: 1; margin: 10px; max-height: 80vh;">
                    <textarea
                        style="width: 100%; height: 100%;"
                        readonly
                        prop:value=output_text
                        placeholder="Transformed text will appear here..."
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
                        { /* Inline SVG for a copy icon (from Material Icons) */ }
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                            <path d="M16 1H4C2.9 1 2 1.9 2 3V17H4V3H16V1ZM19 5H8C6.9 5 6 5.9 6 7V21C6 22.1 6.9 23 8 23H19C20.1 23 21 22.1 21 21V7C21 5.9 20.1 5 19 5ZM19 21H8V7H19V21Z"/>
                        </svg>
                    </button>
                </div>
            </div>

            { /* Bottom Bar */ }
            <div style="
                position: fixed;
                bottom: 0;
                left: 0;
                right: 0;
                height: 50px;
                background:  #333;
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 0 20px;
            ">
                { /* Left side: File input */ }
                <div>
                    <input
                        type="file"
                        style="height: 100%;"
                        on:change=move |ev| {
                            // Get the file input element.
                            let input: HtmlInputElement = ev.target().unwrap().unchecked_into();
                            if let Some(files) = input.files() {
                                if files.length() > 0 {
                                    // Read the first file.
                                    let file = files.get(0).unwrap();
                                    let reader = Rc::new(FileReader::new().unwrap());
                                    let reader_clone = reader.clone();
                                    let set_input_text = set_input_text.clone();
                                    // Define the onload callback.
                                    let onload = Closure::wrap(Box::new(move |_ev: web_sys::ProgressEvent| {
                                        if let Ok(result) = reader_clone.result() {
                                            if let Some(text) = result.as_string() {
                                                set_input_text.set(text);
                                            }
                                        }
                                    }) as Box<dyn FnMut(web_sys::ProgressEvent)>);
                                    reader.set_onload(Some(onload.as_ref().unchecked_ref()));
                                    reader.read_as_text(&file).unwrap();
                                    onload.forget();
                                }
                            }
                        }
                    />
                </div>
                { /* Right side: Dropdown and Transform Button */ }
                <div style="display: flex; align-items: center;">
                    <select
                        style="height: 40px; width: 160px; margin-right: 10px;"
                        on:change=move |ev| set_selected_action.set(event_target_value(&ev))
                    >
                        <option value="arrow">Arrow</option>
                        <option value="avro">Avro</option>
                        <option value="duckdb">DuckDB</option>
                        <option value="json">JSON</option>
                        <option value="jsonschema">JSON-Schema</option>
                        <option value="polars">Polars</option>
                        <option value="spark">Spark</option>
                    </select>
                    <button
                        style="height: 100%;"
                        on:click=move |_| transform_text()
                    >
                        Transform
                    </button>
                </div>
            </div>
        </div>
    }
}
