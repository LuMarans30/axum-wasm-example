use maud::{Markup, Render, html};

use crate::view::layout::Layout;

pub async fn root() -> Markup {
    Layout::new(html!(), "Axum WASM Example".into(), "🦀".into()).render()
}
