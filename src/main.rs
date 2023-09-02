#![allow(non_snake_case)]

mod components;
mod screens;

use components::navbar::NavBar;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use screens::home::Home;
use screens::loading::Loading;
use screens::register::Register;

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(NavBar)]
        #[route("/home")]
        Home {},
    #[end_layout]
    #[route("/")]
    Loading {},
    #[route("/register")]
    Register {}
}

fn main() {
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
            link {
                rel: "stylesheet",
                href: "./public/tailwind.css"
            },
            Router::<Route> {}
    })
}
