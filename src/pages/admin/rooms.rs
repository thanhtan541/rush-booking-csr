use crate::components::layout::Layout;
use leptos::*;

/// A parameterized incrementing button
#[component]
pub fn Rooms() -> impl IntoView {
    let rows = 10;
    view! {
        <Layout>
            <p>Room lists</p>
            <ul>
            {(1..rows)
                .map(|n| view! { <li>Room number: {n}</li>})
                .collect_view()}
            </ul>
        </Layout>
    }
}
