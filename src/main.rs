#![allow(non_snake_case)]

mod components;
mod screens;
mod utils;

use components::navbar::NavBar;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use reqwest::Client;
use screens::home::Home;
use screens::loading::Loading;
use screens::register::Register;

pub const API_URL: &str = "http://localhost:8000";

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
    simple_logger::init().unwrap();
    dioxus_desktop::launch(App);
}

fn App(cx: Scope) -> Element {
    use_shared_state_provider(cx, || Client::new());

    cx.render(rsx! {
            link {
                rel: "stylesheet",
                href: "./public/tailwind.css"
            },
            Router::<Route> {}
    })
}
