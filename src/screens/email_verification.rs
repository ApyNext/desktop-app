use dioxus::prelude::*;
use hyper::HeaderMap;
use reqwest::Client;

use crate::utils::custom_request::custom_post_request;
use crate::API_URL;

pub fn EmailVerification(cx: Scope) -> Element {
    let token = use_state(cx, || String::new());
    let client = use_shared_state::<Client>(cx).unwrap();
    cx.render(rsx! {
        div {
            class: "bg-black text-white relative min-h-screen w-screen text-center",
            form {
                onsubmit: move |_| {
                    to_owned![token, client];
                    async move {
                        custom_post_request(&*client.read(), format!("{}/register/email_confirm", API_URL), token.get(), HeaderMap::new()).await.unwrap();
                    }
                },
                class: "bg-grey-700 rounded-lg m-4 absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                h1 {
                    class: "text-xl font-bold",
                    "Un email a été envoyé à l'adresse email renseignée, veuillez copier-coller le code d'activation reçu par mail ci-dessous pour l'activer, le code est valable pendant 10 minutes. Si le compte n'est pas activé après 10 minutes, il sera supprimé définitivement."
                },
                input {
                    r#type: "password",
                    name: "token",
                    class: "text-lg",
                    oninput: move |e| token.set(e.value.clone()),
                    value: "{token}"
                },
                input {
                    r#type: "submit",
                    value: "Valider"
                }
            }
        }
    })
}
