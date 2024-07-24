use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::admin::rooms::Rooms;
use crate::pages::admin::users::*;
use crate::pages::home::Home;
use crate::pages::login::Login;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <UsersRoutes/>
                <Route path="/*" view=NotFound/>
                <Route path="/login" view=Login/>
                <Route path="/admin/room" view=Rooms/>
            </Routes>
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
