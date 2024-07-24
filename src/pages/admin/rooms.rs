use crate::components::layout::Layout;
use leptos::*;
use leptos_router::*;

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
            <Outlet/>
        </Layout>
    }
}

#[component]
pub fn RoomProfile() -> impl IntoView {
    let id = 1;
    view! {
        <p>Room id: {id}</p>
    }
}

#[component]
pub fn NoRoom() -> impl IntoView {
    view! {
        <p>Please select an room</p>
    }
}
