use crate::components::layout::Layout;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Users() -> impl IntoView {
    let rows = 10;
    view! {
        <Layout>
            <p>User lists</p>
            <ul>
            {(1..rows)
                .map(|n| view! { <li>User number: {n}</li>})
                .collect_view()}
            </ul>
            <Outlet/>
        </Layout>
    }
}

#[component]
pub fn UserProfile() -> impl IntoView {
    let id = 1;
    view! {
        <Layout>
            <p>User id: {id}</p>
            <Outlet/>
        </Layout>
    }
}

#[component]
pub fn NoUser() -> impl IntoView {
    view! {
        <Layout>
            <p>Please select an user</p>
        </Layout>
    }
}
