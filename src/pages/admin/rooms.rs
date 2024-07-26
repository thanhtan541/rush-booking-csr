use crate::{api::API_ADAPTER_INSTANCE, components::layout_with_auth::LayoutWithAuth};
use leptos::*;
use leptos_router::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct ResponseData<T> {
    pub data: Vec<T>,
    pub message: String,
    pub code: u16,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Room {
    id: String,
    name: String,
    description: String,
    number_of_beds: u16,
}

#[component]
pub fn Rooms() -> impl IntoView {
    // Check https://book.leptos.dev/async/10_resources.html
    let rooms = create_resource(
        || (),
        |_| async move {
            let api_apdater = API_ADAPTER_INSTANCE.get().unwrap();
            let resp: ResponseData<Room> = api_apdater.get_rooms().await.json().await.unwrap();

            resp
        },
    );

    view! {
        <LayoutWithAuth>
            <div class="grid grid-cols-2 gap-4 mb-4">
                 <div class="flex items-center justify-center h-60 rounded bg-gray-50 dark:bg-gray-800">
                    <p>Room lists</p>
                    <ul>
                    {move || rooms.get().unwrap().data.into_iter()
                        .map(|n| view! { <li>Room number: {n.id}</li>})
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
