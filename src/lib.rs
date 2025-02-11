use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};
use serde_json::json;
use yaxp_core::xsdp::parser::parse_xsd_string;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::home::Home;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // State for the left text area
    let (input_text, set_input_text) = create_signal(String::new());

    // State for the right text area
    let (output_text, set_output_text) = create_signal(String::new());

    // State for the selected transformation
    let (selected_action, set_selected_action) = create_signal("arrow".to_string());

    // Function to handle the transformation
    let transform_text = move || {
        let input = input_text.get();
        let action = selected_action.get();

        let transformed_text = match action.as_str() {
            "spark" => {
                //let mut spark_text = String::new();
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
                let json_schema = parse_xsd_string(&input, None)
                    .unwrap();
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
            "uppercase" => input.to_uppercase(),
            "snake_case" => input.replace(" ", "_").to_lowercase(),
            _ => input, // Default case, no transformation
        };

        set_output_text.set(transformed_text);
    };

    view! {
        <div style="display: flex; flex-direction: column; height: 100vh;">
            <div style="display: flex; flex: 1;">
                <textarea
                    style="flex: 1; margin: 10px; max-height: 80vh;"
                    on:input=move |ev| set_input_text.set(event_target_value(&ev))
                    placeholder="Enter text here..."
                ></textarea>
                <textarea
                    style="flex: 1; margin: 10px; max-height: 80vh;"
                    readonly
                    prop:value=output_text
                    placeholder="Transformed text will appear here..."
                ></textarea>
            </div>
            <div style="display: flex; justify-content: center; margin: 10px;">
                <select
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
                    style="margin-left: 10px;"
                    on:click=move |_| transform_text()
                >
                    Transform
                </button>
            </div>
        </div>
    }
}
