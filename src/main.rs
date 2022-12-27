use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/simple-counter-webapp/")]
    SimpleCounter,
}

#[function_component]
fn SimpleCounter() -> Html {
    let counter = use_state(|| 0);

    let decrement = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter - 1))
    };

    let increment = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div class={classes!["flex","h-screen", "flex-col", "justify-center"]}>
            <div id="count-layout">
                <div class={classes!["text-slate-900", "dark:text-white", "text-center", "text-9xl"]}>{*counter}</div>
            </div>
            <div class={classes!["add-subtract-buttons"]}>
                    {if *counter == 0 {
                        html!{}

                    } else {
                        html! {
                            <button id="decrement-button" class={classes!["text-slate-900", "dark:text-white", "box-border", "h-32", "w-32", "p-4", "border-2"]} onclick={decrement}>
                                <i class="fa-solid fa-minus"></i>
                            </button>
                        }
                    }}
                    <button id="increment-button" class={classes!["text-slate-900", "dark:text-white","box-border", "h-32", "w-32", "p-4", "border-2"]} onclick={increment}>
                        <i class="fa-solid fa-plus"></i>
                </button>
            </div>
        </div>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::SimpleCounter => html! { <SimpleCounter /> },
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
