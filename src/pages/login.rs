use crate::api::API_ADAPTER_INSTANCE;
use crate::{components::layout_with_auth::LayoutWithAuth, GlobalState};
use leptos::html::Input;
use leptos::{ev::SubmitEvent, *};

/// Default Home Page
#[component]
pub fn Login() -> impl IntoView {
    view! {
        <LayoutWithAuth>
            <SubmitForm/>
        </LayoutWithAuth>
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
struct JWT {
    token_type: String,
    access_token: String,
    refresh_token: String,
    id_token: String,
    expires_in: u16,
}

struct Credential {
    email: String,
    password: String,
}

#[component]
fn SubmitForm() -> impl IntoView {
    let state = expect_context::<RwSignal<GlobalState>>();
    let set_is_logged = create_write_slice(state, |state, n| state.is_logged = n);
    let navigate = leptos_router::use_navigate();

    let email_el: NodeRef<Input> = create_node_ref();
    let password_el: NodeRef<Input> = create_node_ref();
    let login_action = create_action(|input: &Credential| {
        // the input is a reference, but we need the Future to own it
        // this is important: we need to clone and move into the Future
        // so it has a 'static lifetime
        let credential = input.to_owned();
        let body = serde_json::json!({
            "username": credential.email,
            "password": credential.password,
        });
        async move {
            let api_apdater = API_ADAPTER_INSTANCE.get().unwrap();
            let jwt: JWT = api_apdater.post_login(&body).await.json().await.unwrap();
            jwt
        }
    });
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let email_value = email_el().expect("email to exist").value();
        let password_value = password_el().expect("password to exist").value();
        let credential = Credential {
            email: email_value,
            password: password_value,
        };
        login_action.dispatch(credential);
    };

    let submitted = login_action.value();
    create_effect(move |_| {
        // immediately store token and redirect
        match submitted() {
            Some(data) => {
                log::debug!("Value: {:?}", data);
                set_is_logged(true);
                navigate("/admin/users", Default::default());
            },
            None => log::debug!("Value: {:?}", submitted()),
        }
    });

    view! {
        <form
            class="max-w-sm mx-auto"
            on:submit=on_submit>
            <div class="mb-5">
                <label for="email" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Your email</label>
                <input type="email" id="email" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" node_ref=email_el required />
            </div>
            <div class="mb-5">
                <label for="password" class="block mb-2 text-sm font-medium text-gray-900 dark:text-white">Your password</label>
                <input type="password" id="password" class="bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500" node_ref=password_el required />
            </div>
            // Todo: Remember me
            <div class="flex items-start mb-5">
                <div class="flex items-center h-5">
                <input id="remember" type="checkbox" class="w-4 h-4 border border-gray-300 rounded bg-gray-50 focus:ring-3 focus:ring-blue-300 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-blue-600 dark:ring-offset-gray-800 dark:focus:ring-offset-gray-800" />
                </div>
                <label for="remember" class="ms-2 text-sm font-medium text-gray-900 dark:text-gray-300">Remember me</label>
            </div>
            <input type="submit"
                class="text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center dark:bg-blue-600 dark:hover:bg-blue-700 dark:focus:ring-blue-800"
                value="Login"
            />
        </form>
    }
}
