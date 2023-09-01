use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    render! {
        div {
            class: "w-screen min-h-full bg-black",
            div {
                class: "min-h-full m-4 bg-gray-700",
                h1 {
                    class: "text-white",
                    "slt"
                }
            }
        }
    }
}
