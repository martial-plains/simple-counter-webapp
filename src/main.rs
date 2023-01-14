use leptos::*;

#[component]
fn SimpleCounter(cx: Scope) -> impl IntoView {
    let (counter, set_counter) = create_signal(cx, 0);

    let decrement = move |_| set_counter(counter() - 1);

    let increment = move |_| set_counter(counter() + 1);

    view! { cx,
        <div class={"flex h-screen flex-col justify-center"}>
            <div id="count-layout">
                {
                    move || view! {cx, <div class={"text-slate-900 dark:text-white text-center text-9xl"}>{counter()}</div>}
                }
            </div>
            <div class={"add-subtract-buttons"}>
                    {move || if counter() == 0 {
                        view!{cx, <div></div>}.into_any()

                    } else {
                        view! {cx,
                            <button id="decrement-button" class={"text-slate-900 dark:text-white box-border h-32 w-32 p-4 border-2"} on:click={decrement}>
                                <i class="fa-solid fa-minus"></i>
                            </button>
                        }.into_any()
                    }}
                    <button id="increment-button" class={"text-slate-900 dark:text-white box-border h-32 w-32 p-4 border-2"} on:click=increment>
                        <i class="fa-solid fa-plus"></i>
                </button>
            </div>
        </div>
    }
}

/// entry point
fn main() {
    console_error_panic_hook::set_once();
    _ = console_log::init_with_level(log::Level::Info);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,  <SimpleCounter/> })
}
