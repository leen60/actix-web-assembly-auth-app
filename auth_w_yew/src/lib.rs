mod register;
mod store;

use reqwasm::http::Request;
use register::Register;

use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;
use serde_json::json;
use crate::store::YewduxStore;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UsersResponse {
    pub users: Vec<User>,
}

#[function_component(App)]
pub fn view() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let error = Box::new(use_state(|| None));
    let users_array = Box::new(use_state(|| None));

    let handle_form_submit = store
        .dispatch()
        .reduce_callback_with(move|state, event: FocusEvent| {
            event.prevent_default();
               let is_authenticated = event
                .target()
                .unwrap();
            state.is_authenticated = true;

            let error = error.clone();
            let users_array = users_array.clone();

            let body = json!({
              "username": state.username.clone(),
              "password": state.password.clone()
            });

            wasm_bindgen_futures::spawn_local(async move {

            let error = error.clone();
            let users_array = users_array.clone();

                let fetched_users = Request::post("http://localhost:3000/api/login")
                    .header("content-type", "application/json")
                    .header("Content-Length", "<calculated when request is sent>")
                    .body(body.to_string()).send().await;


                match fetched_users {
                    Ok(response) => {
                        let json: Result<String, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                users_array.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    },
                    Err(e) => error.set(Some(e.to_string())),
                }

            });
        });

    let handle_username_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let username = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.username = username;
        });

    let handle_password_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let password = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.password = password;
        });

    let users_array = Box::new(use_state(|| None));
    let error = Box::new(use_state(|| None));
    let onclick = {
        let users_array = users_array.clone();
        let error = error.clone();

        Callback::from(move |_| {
            let users_array = users_array.clone();
            let error = error.clone();
            wasm_bindgen_futures::spawn_local(async move {

                let fetched_users = Request::get("http://localhost:3000/api/users").send().await;

                match fetched_users {
                    Ok(response) => {
                        let json: Result<Vec<User>, _> = response.json().await;
                        match json {
                            Ok(f) => {
                                users_array.set(Some(f));
                            }
                            Err(e) => error.set(Some(e.to_string())),
                        }
                    }
                    Err(e) => error.set(Some(e.to_string())),
                }
            });
        })
    };

    match (*users_array).as_ref() {
        Some(f) => f
                .iter()
                .map(|user| {
                    html! {
                        <div class="user">
                            <div class="name">{user.name.to_owned()}</div>
                            <div class="email">{user.email.to_owned()}</div>
                        </div>
                    }
                })
                .collect(),
        None => match (*error).as_ref() {
            Some(e) => {
                html! {
                    <>
                        <Register />
                              <form onsubmit={handle_form_submit}>
                                <h1>{"Login"}</h1>
                                <div>
                                    <input type="text" placeholder="username" onchange={handle_username_change} />
                                </div>
                                <div>
                                    <input type="password" placeholder="password" onchange={handle_password_change} />
                                </div>
                                <div>
                                    <button>{"Login"}</button>
                                </div>
                              </form>
                        {"error "} {e}
                        <button {onclick}>{"Get users"}</button>
                    </>
                }
            }
            None => {
                html! {
                    <>
                        <Register />
                              <form onsubmit={handle_form_submit}>
                                <h1>{"Login"}</h1>
                                <div>
                                    <input type="text" placeholder="username" onchange={handle_username_change} />
                                </div>
                                <div>
                                    <input type="password" placeholder="password" onchange={handle_password_change} />
                                </div>
                                <div>
                                    <button>{"Login"}</button>
                                </div>
                              </form>
                        <button {onclick}>{"Get users"}</button>
                    </>
                }
            }
        },
    }
}