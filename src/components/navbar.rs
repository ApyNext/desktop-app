use crate::Route;
use dioxus::prelude::*;
use dioxus_router::prelude::*;

pub fn NavBar(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "flex flex-row w-screen min-h-screen dark:bg-black dark:text-white",
            nav {
                class: "flex flex-col p-6",
                ul {
                    li {
                        "Link"
                    }
                    li {
                        "truc"
                    }
                    li {
                        "truc"
                    }
                }
            }

            Outlet::<Route> {}
        }

    })
}
