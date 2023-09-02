use dioxus::prelude::*;

pub fn Register(cx: Scope) -> Element {
    let username = use_state(cx, || "".to_string());
    let email = use_state(cx, || "".to_string());
    let password = use_state(cx, || "".to_string());
    let confirm_password = use_state(cx, || "".to_string());

    cx.render(rsx! {
        div {
            class: "w-screen min-h-screen bg-black relative",
            form {
                class: "bg-gray-900 text-white rounded-lg p-4 text-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                onsubmit: move |e| println!("{:?}", e.values),
                h1 {
                    class: "text-4xl mb-2",
                    "Créer un compte"
                },
                input {
                    required: "",
                    r#type: "text",
                    class: "bg-black p-2 border-none mb-2 block w-full",
                    value: "{username}",
                    oninput: move |e| username.set(e.value.clone()),
                    placeholder: "Nom d'utilisateur",
                },
                input {
                    required: "",
                    r#type: "email",
                    class: "bg-black p-2 border-none mb-2 block w-full",
                    value: "{email}",
                    oninput: move |e| email.set(e.value.clone()),
                    placeholder: "Email",
                },
                input {
                    required: "",
                    r#type: "password",
                    class: "bg-black p-2 border-none mb-2 block w-full",
                    value: "{password}",
                    oninput: move |e| password.set(e.value.clone()),
                    placeholder: "Mot de passe",
                },
                input {
                    required: "",
                    r#type: "password",
                    class: "bg-black p-2 border-none mb-2 block w-full",
                    value: "{confirm_password}",
                    oninput: move |e| confirm_password.set(e.value.clone()),
                    placeholder: "Confirmer le mot de passe",
                },
                label {
                    class: "block",
                    r#for: "gender",
                    "Genre"
                },
                select {
                    name: "gender",
                    id: "gender",
                    class: "bg-black",
                    oninput: move |e| println!("{e:?}"),
                    option {
                        value: "",
                        "Sélectionnez votre genre"
                    },
                    option {
                        value: "male",
                        "Homme"
                    },
                    option {
                        value: "female",
                        "Femme"
                    }
                },
                input {
                    class: "bg-black text-white border-solid border-2 border-white hover:bg-white hover:text-black p-2 rounded-lg mt-2 block mx-auto cursor-pointer duration-500",
                    r#type: "submit",
                    "Créer un compte "
                }
            }
        }
    })
}
