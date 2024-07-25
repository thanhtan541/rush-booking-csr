use crate::{components::layout_with_auth::LayoutWithAuth, GlobalState};
use leptos::*;

#[component]
pub fn Logout() -> impl IntoView {
    view! {
        <LayoutWithAuth>
            <SubmitForm/>
        </LayoutWithAuth>
    }
}

#[component]
fn SubmitForm() -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>();
    let set_is_logged = create_write_slice(state, |state, n| state.is_logged = n);
    let navigate = leptos_router::use_navigate();
    view! {
        <button class="bg-sky-500 hover:bg-sky-700"
                on:click=move |_| {
                    set_is_logged(false);
                    navigate("/login", Default::default());
                }
        >
         "Logout"
        </button>
    }
}
