use crate::components::layout_with_auth::LayoutWithAuth;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Users() -> impl IntoView {
    let rows = 10;
    view! {
        <LayoutWithAuth>
            <div class="grid grid-cols-2 gap-4 mb-4">
                 <div class="flex items-center justify-center h-60 rounded bg-gray-50 dark:bg-gray-800">
                    <p>User lists</p>
                    <ul>
                    {(1..rows)
                        .map(|n| view! { <li>User number: {n}</li>})
                        .collect_view()}
                    </ul>
                 </div>
                 <div class="flex items-center justify-center h-60 rounded bg-gray-50 dark:bg-gray-800">
                    <Outlet/>
                 </div>
            </div>
        </LayoutWithAuth>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let id = 1;
    view! {
        <p>User id: {id}</p>
    }
}

#[component]
pub fn NoUser() -> impl IntoView {
    view! {
        <p>Please select an user</p>
    }
}
