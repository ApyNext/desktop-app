use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use std::time::Duration;

pub fn Loading(cx: Scope) -> Element {
    let nav = use_navigator(cx);

    use_future(cx, (), move |_| {
        to_owned![nav];
        async move {
            tokio::time::sleep(Duration::from_secs(1)).await;

            nav.replace(Route::Register {});
        }
    });

    cx.render(rsx! {
        div {
            class: "w-screen min-h-screen bg-black text-white grid place-content-center",
            h1 {
                class: "text-5xl",
                "Loading..."
            }
        }

    })
}
