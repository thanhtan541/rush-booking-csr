use crate::components::layout_with_auth::LayoutWithAuth;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Users() -> impl IntoView {
    let rows = 10;
    let (selected, set_selected) = create_signal(0);
    view! {
        <LayoutWithAuth>
            <p>USER LIST</p>
            <div class="grid grid-cols-2 gap-4 mb-4">
                // List view
                 <div class="flex flex-col border-2 border-gray-200 border-dashed rounded dark:bg-gray-800">
                    {(1..rows)
                        .map(|n| view! {
                            <div class=("bg-gray-100", move || selected() == n)>
                                <ListItem id=n title=n.to_string() href=n.to_string() on_click=set_selected />
                            </div>
                        })
                        .collect_view()}
                 </div>
                // Item detail
                 <div class="flex items-center justify-center border-2 border-gray-200 border-dashed rounded bg-gray-50 dark:bg-gray-800">
                    <Outlet/>
                 </div>
            </div>
        </LayoutWithAuth>
    }
}

#[component]
fn ListItem(
    /// Id of the listitem
    id: i32,
    /// Title of the listitem
    title: String,
    /// Link to related resource, e.g: http://domain/path/to/resource
    href: String,
    on_click: WriteSignal<i32>,
) -> impl IntoView {
    let class = "flex items-center justify-center p-2 text-gray-900 rounded-lg dark:text-hite hover:bg-gray-100 dark:hover:bg-gray-700";
    view! {
       <A href={href} class={class} on:click=move |_| on_click.update(|value| *value = id) >
        <div>
            <p class="text-xl">{id}</p>
            <p>{title} bed room</p>
        </div>
       </A>
    }
}

#[derive(Params, PartialEq)]
struct UserParams {
    id: usize,
}
#[component]
pub fn UserProfile() -> impl IntoView {
    let params = use_params::<UserParams>();
    let id =
        move || params.with(|params| params.as_ref().map(|params| params.id).unwrap_or_default());
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
