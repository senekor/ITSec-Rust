use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {


        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos-xss-demo.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>

        <br/><br/>

        <XssSafeForm />
    }
}

#[component]
fn XssSafeForm() -> impl IntoView {
    let (user_input, set_user_input) = create_signal(String::new());

    view! {
        "Search: "
        <input
            id="searchbox"
            type="text"
            autocomplete="off"
            bind:value=user_input
            on:input=move |ev| set_user_input(event_target_value(&ev))
        />
        <br/>
        
        // echo user input into DOM. XSS vulnerability???
        { user_input }

        // actual inline JS script for demonstration
        <script>alert("inline script")</script>
    }
}

/*
copy into input field for demo:

<script>alert("XSS")</script>
*/
