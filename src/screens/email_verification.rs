use dioxus::prelude::*;

pub fn EmailVerification(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            class: "bg-black text-white relative min-h-screen w-screen text-center",
            form {
                onsubmit: move |e| {
                    log::info!("{:?}", e.values);
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
                }
            }
        }
    })
}
