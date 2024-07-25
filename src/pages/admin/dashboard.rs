use crate::components::layout_with_auth::LayoutWithAuth;
use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <LayoutWithAuth>
            <div class="grid grid-cols-2 gap-4 mb-4">
                Dashboard
            </div>
        </LayoutWithAuth>
    }
}
