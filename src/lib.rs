use leptos::*;
use leptos_meta::*;
use leptos_router::{Router, *};

// Modules
mod api;
mod components;
mod pages;

use crate::api::ApiAdapter;
use crate::api::API_ADAPTER_INSTANCE;
// Top-Level pages
use crate::components::{navbar::Navbar, sidebar::Sidebar};
use crate::pages::admin::dashboard::Dashboard;
use crate::pages::admin::rooms::*;
use crate::pages::admin::users::*;
use crate::pages::login::Login;
use crate::pages::logout::Logout;
use crate::pages::not_found::NotFound;

#[derive(Clone, Debug, Default)]
struct GlobalState {
    is_logged: bool,
}

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    // Global state
    let state = create_rw_signal(GlobalState::default());
    provide_context(state);
    // Global instance, such as logger, api client
    let api_client = reqwest::Client::builder().build().unwrap();
    let api_adapter = ApiAdapter {
        address: "http://localhost:8000".into(), // Backend address
        client: api_client,
    };
    API_ADAPTER_INSTANCE.set(api_adapter).unwrap();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>
        <Stylesheet id="leptos" href="/style/output.css"/>

        <Router>
            <Navbar/>
            <Sidebar/>
            <main>
                <Routes>
                    <Route path="/" view=Dashboard/>
                    <Route path="/admin/dashboard" view=Dashboard/>
                    <UsersRoutes/>
                    <RoomsRoutes/>
                    <Route path="/*" view=NotFound/>
                    <Route path="/login" view=Login/>
                    <Route path="/logout" view=Logout/>
                </Routes>
            </main>
        </Router>
    }
}

#[component(transparent)]
fn UsersRoutes() -> impl IntoView {
    view! {
        <Route path="admin/users" view=Users>
            <Route path=":id" view=UserProfile/>
            <Route path="" view=NoUser/>
        </Route>
    }
}

#[component(transparent)]
fn RoomsRoutes() -> impl IntoView {
    view! {
        <Route path="admin/rooms" view=Rooms>
            <Route path=":id" view=RoomProfile/>
            <Route path="" view=NoRoom/>
        </Route>
    }
}
