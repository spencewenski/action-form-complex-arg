use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/action-form-complex-arg.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors /> }
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let submit = Action::<Foo, _>::server();

    view! {
        <ActionForm action=submit>
            <label>"foo"<input type="text" name="request[foo]" /></label>
            <label>"bar"<input type="text" name="request[bar]" /></label>
            <input type="submit" value="Submit" />
        </ActionForm>
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
struct FooRequest {
    pub foo: String,
    pub bar: String,
}

#[server]
async fn foo(request: FooRequest) -> Result<(), ServerFnError> {
    Ok(())
}
