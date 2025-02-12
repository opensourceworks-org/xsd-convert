use leptos::prelude::*;

#[component]
pub fn TransformControls(
    selected_action: ReadSignal<String>,
    on_action_change: WriteSignal<String>,
    on_transform: impl Fn() -> () + Send + Sync + 'static,
) -> impl IntoView {
    view! {
        <div style="display: flex; align-items: center;">
            <select
                style="height: 40px; width: 160px; margin-right: 10px;"
                prop:value=selected_action
                //set_count.update(|count: &mut i32| *count += 1);
                on:change=move |ev| {
                           let value = event_target_value(&ev).to_string();
                            //web_sys::console::log_1(&format!("Dropdown changed: {}", value).into());
                            on_action_change.set(value);
        }
            >
                <option value="arrow">{"Arrow"}</option>
                <option value="avro">{"Avro"}</option>
                <option value="duckdb">{"DuckDB"}</option>
                <option value="json">{"JSON"}</option>
                <option value="jsonschema">{"JSON-Schema"}</option>
                <option value="polars">{"Polars"}</option>
                <option value="spark">{"Spark"}</option>
            </select>
            <button
                style="height: 100%;"
                on:click=move |_| on_transform()
            >
                {"Transform"}
            </button>
        </div>
    }
}
