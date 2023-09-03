use chrono::NaiveDate;
use dioxus::prelude::*;
use hyper::{header::CONTENT_TYPE, HeaderMap};
use log::error;
use reqwest::Client;

use crate::{
    utils::{app_error::AppError, custom_request::custom_post_request, register::RegisterUser},
    API_URL,
};

pub fn Register(cx: Scope) -> Element {
    let username = use_state(cx, || "".to_string());
    let email = use_state(cx, || "".to_string());
    let password = use_state(cx, || "".to_string());
    let confirm_password = use_state(cx, || "".to_string());
    let date = use_state(cx, || "2023-10-29".to_string());
    let is_male = use_state(cx, || None::<bool>);
    let client_state =
        use_shared_state::<Client>(cx).expect("Erreur lors de la récupération du state Client");

    cx.render(rsx! {
        div {
            class: "w-screen min-h-screen bg-black relative",
            form {
                class: "bg-gray-900 text-white rounded-xl md:p-8 p-4 text-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                onsubmit: move |_| {
                to_owned![date, username, email, password, is_male, client_state];
                async move {
                    let date = match NaiveDate::parse_from_str(date.get(), "%Y-%m-%d") {
                        Ok(date) => date,
                        Err(e) => {
                            log::warn!("Erreur lors de la convertion de la date en secondes depuis l'Epoch : {}", e);
                            return;
                        }
                    };
                    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                    let duration = date.signed_duration_since(epoch);
                    let seconds_since_epoch = duration.num_seconds();
                    let user = RegisterUser {
                            username: username.get(),
                            email: email.get(),
                            password: password.get(),
                            birthdate: seconds_since_epoch,
                            is_male: *is_male.get()
                        };

                    let mut headers = HeaderMap::new();
                    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
                    if let Err(e) = custom_post_request(&*client_state.read(), format!("{}/register", API_URL), serde_json::to_string(&user).unwrap(), headers).await {
                            match e {
                                AppError::InternalServerError => error!("Internal Server Error"),
                                AppError::Forbidden(e) => error!("Forbidden : {e}"),
                                AppError::UnknownError(e) => error!("Unknown error, status code {e}")
                            }
                        };
                }},
                h1 {
                    class: "text-3xl mb-4",
                    "Créer un compte"
                },
                input {
                    required: "",
                    r#type: "text",
                    class: "bg-black p-3 border-none mb-2 block w-full",
                    value: "{username}",
                    oninput: move |e| username.set(e.value.clone()),
                    placeholder: "Nom d'utilisateur",
                },
                input {
                    required: "",
                    r#type: "email",
                    class: "bg-black p-3 border-none mb-2 block w-full",
                    value: "{email}",
                    oninput: move |e| email.set(e.value.clone()),
                    placeholder: "Email",
                },
                input {
                    required: "",
                    r#type: "password",
                    class: "bg-black p-3 border-none mb-2 block w-full",
                    value: "{password}",
                    oninput: move |e| password.set(e.value.clone()),
                    placeholder: "Mot de passe",
                },
                input {
                    required: "",
                    r#type: "password",
                    class: "bg-black p-3 border-none mb-2 block w-full",
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
                    class: "mt-1 mb-2 text-black",
                    oninput: move |e| {
                        let gender = e.data.value.clone();
                        is_male.set(match gender.as_str() {
                            "male" => Some(true),
                            "female" => Some(false),
                            _ => None
                        });
                    },
                    option {
                        class: "appearance-none bg-black",
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
                    },
                    option {
                        value: "null",
                        "Je ne souhaite pas répondre"
                    }
                },
                input {
                    class: "bg-black block mb-4 mx-auto p-2",
                    r#type: "date",
                    value: "{date}",
                    oninput: move |e| date.set(e.value.clone()),
                },
                input {
                    class: "bg-black w-full text-lg font-bold text-white border-solid border-2 border-white hover:bg-white hover:text-black p-3 block mx-auto cursor-pointer duration-500",
                    r#type: "submit",
                    "Créer un compte"
                }
            }
        }
    })
}
