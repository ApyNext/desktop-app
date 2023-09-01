use dioxus::prelude::*;

pub fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        p {
            "Home Screen"
        }
    })
}
