use leptos::prelude::*;
use leptos_meta::*;
use serde_json::json;
use crate::components::file_input::FileInput;
use crate::components::textarea::{InputTextArea, OutputTextArea};
use crate::components::top_bar::TopBar;
use crate::components::transform_controls::TransformControls;
use transform::transform::transform_xsd;
use crate::components::switch::Switch;

mod components;
mod pages;
mod transform;


#[component]
pub fn App() -> impl IntoView {
    // signals for the input text, output text, and selected action.
    let (input_text, set_input_text) = signal(String::new());
    let (output_text, set_output_text) = signal(String::new());
    let (selected_action, set_selected_action) = signal("arrow".to_string());
    let (to_lowercase, set_to_lowercase) = signal(false);

    let transform_text = move || {
        // web_sys::console::log_1(&format!("started transform_text").into());
        let input = input_text.get();
        let action = selected_action.get();

        let transformed_text = transform_xsd(&input, &action, Some(to_lowercase.get()))
            .unwrap_or_else(|e| json!({"error": e.to_string()})
                .to_string()
            );

        set_output_text.set(transformed_text);
    };

    view! {
        <Html />
        <Meta charset="UTF-8" />
        <Title text="<xsd ⚡> Convert xsd schemas" />
        <div class="page-container">
            <TopBar />

            <div class="text-area-container">
                <InputTextArea
                    input_text=input_text
                    on_input=move |text| set_input_text.set(text)
                />
                <OutputTextArea output_text=output_text />
            </div>

            <div class="footer-bar">
                <FileInput on_file_load=set_input_text.clone() />
                <Switch checked=to_lowercase on_toggle=set_to_lowercase label="Lowercase Columns" id="lowercase-switch"/>
                <TransformControls selected_action=selected_action
                                on_action_change=set_selected_action
                                on_transform=transform_text/>
            </div>
        </div>
    }
}
