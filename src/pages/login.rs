use crate::components::layout::Layout;
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
    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let username_element: NodeRef<html::Input> = create_node_ref();
    let password_element: NodeRef<html::Input> = create_node_ref();
    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        // here, we'll extract the value from the input
        let value = username_element()
            // event handlers can only fire after the view
            // is mounted to the DOM, so the `NodeRef` will be `Some`
            .expect("<input> should be mounted")
            // `leptos::HtmlElement<html::Input>` implements `Deref`
            // to a `web_sys::HtmlInputElement`.
            // this means we can call`HtmlInputElement::value()`
            // to get the current value of the input
            .value();
        set_name(value);
    };
    view! {
        <form on:submit=on_submit> // on_submit defined below
            <div>
                <input type="text"
                    value=name
                    node_ref=username_element
                />
            </div>
            <div>
                <input type="password"
                    node_ref=password_element
                />
            </div>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
