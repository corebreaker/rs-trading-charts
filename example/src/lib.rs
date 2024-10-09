use charts::Chart;
use leptos::{html::ElementChild, IntoView, view, component};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div><Chart/></div>
    }
}
