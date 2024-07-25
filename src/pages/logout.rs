use crate::{components::layout::Layout, GlobalState};
use leptos::*;

#[component]
pub fn Logout() -> impl IntoView {
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
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_is_logged(false);
    };
    view! {
        <form on:submit=on_submit> // on_submit defined below
            <input type="submit" value="Submit"/>
        </form>
    }
}
