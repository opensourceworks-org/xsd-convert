use leptos::prelude::*;

#[component]
pub fn TopBar() -> impl IntoView {
    view! {
        <div class="top-bar">
            <p class="logo">
                {"<xsd "}
                <span style="color: yellow;">{"⚡"}</span>
                {">"}
            </p>
        </div>
    }
}
