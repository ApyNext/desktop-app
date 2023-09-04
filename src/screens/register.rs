use chrono::NaiveDate;
use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use email_address::EmailAddress;
use hyper::{header::CONTENT_TYPE, HeaderMap};
use log::error;
use reqwest::Client;

use crate::{
    utils::{
        app_error::AppError,
        custom_request::custom_post_request,
        register::{check_username, RegisterUser},
    },
    screens::email_verification::EmailVerification,
    API_URL, Route,
};

pub fn Register(cx: Scope) -> Element {
    let username = use_state(cx, || String::new());
    let username_error = use_state(cx, || String::new());
    let email = use_state(cx, || "".to_string());
    let email_error = use_state(cx, || String::new());
    let password = use_state(cx, || String::new());
    let password_error = use_state(cx, || String::new());
    let confirm_password = use_state(cx, || String::new());
    let confirm_password_error = use_state(cx, || String::new());
    let is_male = use_state(cx, || None::<bool>);
    let is_male_error = use_state(cx, || String::new());
    let birthdate = use_state(cx, || String::from("2023-10-29"));
    let birthdate_error = use_state(cx, || String::new());
    let client_state =
        use_shared_state::<Client>(cx).expect("Erreur lors de la récupération du state Client");

    let nav = use_navigator(cx);

    cx.render(rsx! {
        div {
            class: "w-screen min-h-screen bg-black relative",
            form {
                class: "bg-gray-900 text-white rounded-xl md:p-8 lg:p-12 p-4 text-center absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2",
                onsubmit: move |_| {
                    to_owned![username, email, password, is_male, birthdate, client_state, nav];
                    async move {
                        let date = match NaiveDate::parse_from_str(birthdate.get(), "%Y-%m-%d") {
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
                        if let Err(e) = custom_post_request(&*client_state.read(), format!("{}/register", API_URL), &user, headers).await {
                                match e {
                                    AppError::InternalServerError => error!("Internal Server Error"),
                                    AppError::Forbidden(e) => error!("Forbidden : {e}"),
                                    AppError::UnknownError(e) => error!("Unknown error, status code {e}")
                                }
                            return;
                            };
                        nav.replace(Route::EmailVerification {  });
                        }
                    },
                    h1 {
                        class: "text-3xl mb-4",
                        "Créer un compte"
                    },
                    input {
                        required: "",
                        r#type: "text",
                        value: "{username}",
                    border_color: if username_error.get().is_empty() {
                        if username.get().is_empty() {
                            "gray"
                        } else {"green"}} else {"red"},
                        oninput: move |e| {
                        username.set(e.value.clone());
                        username_error.set(check_username(&e.value));
                    },
                        placeholder: "Nom d'utilisateur",
                    },
                    p {
                    class: "text-red-700 text-center font-bold mb-2",
                    display: if username_error.get().is_empty() {"none"} else {"block"},
                    "{username_error}"
                },
                input {
                    required: "",
                    r#type: "email",
                    value: "{email}",
                    oninput: move |e| {
                        email.set(e.value.clone());
                        email_error.set(match EmailAddress::is_valid(&e.value) {
                            true => String::new(),
                            false => String::from("Email invalide")
                        });
                    },
                    border_color: if email_error.get().is_empty() {if email.get().is_empty() {"gray"} else {"green"}} else {"red"},
                    placeholder: "Email",
                },
                p {
                    class: "text-red-700 text-center font-bold mb-2",
                    display: if email_error.get().is_empty() {"none"} else {"block"},
                    "{email_error}"
                },
                input {
                    required: "",
                    r#type: "password",
                    value: "{password}",
                    oninput: move |e| {
                        password.set(e.value.clone());
                        password_error.set(if e.value.len() >= 8 {String::new()} else {String::from("Mot de passe trop court !")});
                    },
                    border_color: if password_error.get().is_empty() {if password.get().is_empty() {"gray"} else {"green"}} else {"red"},
                    placeholder: "Mot de passe",
                },
                p {
                    class: "text-red-700 text-center font-bold mb-2",
                    display: if password_error.get().is_empty() {"none"} else {"block"},
                    "{password_error}"
                },
                input {
                    required: "",
                    r#type: "password",
                    value: "{confirm_password}",
                    oninput: move |e| {
                        confirm_password.set(e.value.clone());
                        confirm_password_error.set(if password.get() == &e.value {String::new()} else {String::from("Les mots de passe ne correspondent pas.")})
                    },
                    border_color: if confirm_password_error.get().is_empty() {if confirm_password.get().is_empty() {"gray"} else {"green"}} else {"red"},
                    placeholder: "Confirmer le mot de passe",
                },
                p {
                    class: "text-red-700 text-center font-bold mb-2",
                    display: if confirm_password_error.get().is_empty() {"none"} else {"block"},
                    "{confirm_password_error}"
                }
                label {
                    display: "block",
                    r#for: "gender",
                    "Genre"
                },
                select {
                    name: "gender",
                    id: "gender",
                    class: "mt-1 mb-2 text-black",
                    oninput: move |e| {
                        let gender = e.data.value.clone();
                        let mut is_error = false;
                        is_male.set(match gender.as_str() {
                            "male" => Some(true),
                            "female" => Some(false),
                            "null" => None,
                            _ => {
                                is_error = true;
                                None
                            }
                        });
                        
                        is_male_error.set(if is_error {String::from("Veuillez renseigner ce champ.")} else {String::new()});
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
                p {
                    class: "text-red-700 text-center font-bold mb-2",
                    display: if is_male_error.get().is_empty() {"none"} else {"block"},
                    "{is_male_error}"
                },
                label {
                    display: "block",
                    r#for: "birthdate",
                    "Date de naissance"
                }
                input {
                    class: "bg-black block mb-4 mx-auto",
                    r#type: "date",
                    id: "birthdate",
                    value: "{birthdate}",
                    oninput: move |e| birthdate.set(e.value.clone()),
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
