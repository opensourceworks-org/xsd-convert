use leptos::prelude::*;

#[component]
pub fn TopBar() -> impl IntoView {

    view! {

        <div class="top-bar">
            { /* Left: Branding */ }
            <div style="font-size: 1.1rem; font-family: monospace;">
                {"<xsd "}
                <span style="color: yellow;">{"⚡"}</span>
                {">"}
            </div>
            { /* Right: Icon Links */ }
            <div style="display: flex; gap: 15px; align-items: center;">
                { /* Python (PyPI) Link with Python Logo */ }
                <a
                    href="https://pypi.org/project/pyaxp/"
                    target="_blank"
                    style="color: inherit; text-decoration: none;"
                    title="pyaxp"
                >
                    <img
                        src="/img/python.svg"
                        alt="PyYaxp"
                        style="height: 24px; width: auto;"
                    />
                </a>
                { /* Rust (crates.io) Link with Rust Logo */ }
                <a
                    href="https://crates.io/crates/yaxp-common"
                    target="_blank"
                    style="color: inherit; text-decoration: none;"
                    title="yaxp-common (deprecated -> yaxp-core)"
                >
                    <img
                        src="/img/rust.svg"
                        alt="yaxp-common"
                        style="height: 24px; width: auto;"
                    />
                </a>
                <a
                    href="https://crates.io/crates/yaxp-core"
                    target="_blank"
                    style="color: inherit; text-decoration: none;"
                    title="yaxp-core"
                >
                    <img
                        src="/img/rust.svg"
                        alt="Rust Logo"
                        style="height: 24px; width: auto;"
                    />
                </a>
                <a
                    href="https://crates.io/crates/yaxp-cli"
                    target="_blank"
                    style="color: inherit; text-decoration: none;"
                    title="yaxp-cli"
                >
                    <img
                        src="/img/commandline.svg"
                        alt="Command line"
                        style="height: 24px; width: auto;"
                    />
                </a>
                { /* GitHub Link with Inline SVG Icon */ }
                <a
                    href="https://github.com/opensourceworks-org/xsd-convert/blob/main/README.md"
                    target="_blank"
                    style="color: inherit; text-decoration: none;"
                    title="GitHub"
                >
                    <svg height="24" width="24" viewBox="0 0 16 16" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                        <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29
                                 6.53 5.47 7.59.4.07.55-.17.55-.38
                                 0-.19-.01-.82-.01-1.49C3.76 13.91
                                 3.33 12.73 3.33 12.73c-.36-.91-.88-1.15-.88-1.15-.72-.49.05-.48.05-.48.8.06
                                 1.22.82 1.22.82.71 1.22 1.87.87 2.33.66.07-.52.28-.87.51-1.07-2.22-.25-4.56-1.11-4.56-4.95
                                 0-1.09.39-1.98 1.03-2.68-.1-.25-.45-1.27.1-2.65
                                 0 0 .84-.27 2.75 1.02A9.564 9.564 0 0 1 8 4.7c.85.004
                                 1.71.116 2.51.34 1.9-1.29 2.74-1.02 2.74-1.02.55 1.38.2 2.4.1
                                 2.65.64.7 1.03 1.59 1.03 2.68 0 3.85-2.34 4.7-4.57
                                 4.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2
                                 0 .21.15.46.55.38A8.013 8.013 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                    </svg>
                </a>
            </div>
        </div>
    }
}
