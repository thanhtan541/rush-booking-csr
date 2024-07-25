use crate::{components::layout::Layout, GlobalState};
use leptos::*;

/// Default Home Page
#[component]
pub fn Login() -> impl IntoView {
    view! {
        <Layout>
            <SubmitForm/>
        </Layout>

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
                    set_is_logged(true);
                    navigate("/admin/users", Default::default());
                }
        >
         "Login"
        </button>
    }
}
