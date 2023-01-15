use leptos::*;

fn get_stored_count() -> Option<usize> {
    let storage = window()
        .local_storage()
        .expect("Could not find storage")
        .expect("Could not get storage struct");

    storage
        .get("count")
        .expect("Could not get `count` from storage")
        .map(|s| s.parse::<usize>().expect("Could not parse string to usize"))
}

fn set_stored_count(value: usize) {
    let storage = window()
        .local_storage()
        .expect("Could not find storage")
        .expect("Could not get storage struct");

    storage
        .set("count", value.to_string().as_str())
        .expect("Could not set `count` to storage")
}

#[component]
fn SimpleCounter(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, get_stored_count().unwrap_or_default());

    window_event_listener("storage", move |_| {
        if let Some(stored_count) = get_stored_count() {
            if stored_count != count() {
                set_stored_count(stored_count);
                set_count(stored_count)
            }
        }
    });

    let decrement = move |_| {
        let result = count() - 1;
        set_stored_count(result);
        set_count(result);
    };

    let increment = move |_| {
        let result = count() + 1;
        set_stored_count(result);
        set_count(result);
    };

    view! { cx,
        <div class={"flex h-screen flex-col justify-center"}>
            <div id="count-layout">
                {
                    move || view! {cx, <div class={"text-slate-900 dark:text-white text-center text-9xl"}>{count()}</div>}
                }
            </div>
            <div class={"add-subtract-buttons"}>
                    {move || if count() == 0 {
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
