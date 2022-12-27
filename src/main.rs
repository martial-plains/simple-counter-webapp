use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/yew-template-for-github-io/")]
    Home,
    #[at("/yew-template-for-github-io/about")]
    About,
    #[not_found]
    #[at("/yew-template-for-github-io/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::About => html! { <p>{ "About" }</p> },
        Route::NotFound => html! { <p>{ "Not Found" }</p> },
    }
}

/// main root
#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

/// entry point
fn main() {
    yew::Renderer::<App>::new().render();
}
