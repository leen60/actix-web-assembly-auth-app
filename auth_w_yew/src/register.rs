use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use reqwasm::http::Request;
use yew::prelude::*;
use yewdux::prelude::*;
use yewdux_functional::use_store;
use gloo::console::log;
use serde_json::json;
use crate::store::YewduxStore;

#[function_component(Register)]
pub fn view() -> Html {
    let store = use_store::<PersistentStore<YewduxStore>>();
    let error = Box::new(use_state(|| None));
    let users_array = Box::new(use_state(|| None));
    let handle_form_submit = store
        .dispatch()
        .reduce_callback_with(move|state, event: FocusEvent| {
            event.prevent_default();
            let error = error.clone();
            let users_array = users_array.clone();
         
            let body = json!({
              "id": "0000",  
              "name": state.username.clone(),
              "email": state.email.clone(),
              "password": state.password.clone()
            });

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users = Request::post("http://localhost:3000/api/register")
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
                    }
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

    let handle_email_change = store
        .dispatch()
        .reduce_callback_with(|state, event: Event| {
            let email = event
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();
            state.email = email;
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

    html! {
      <form onsubmit={handle_form_submit}>
        <h1>{"Register"}</h1>
        <div>
            <input type="text" placeholder="username" onchange={handle_username_change} />
        </div>
        <div>
            <input type="email" placeholder="email" onchange={handle_email_change} />
        </div>
        <div>
            <input type="password" placeholder="password" onchange={handle_password_change} />
        </div>
        <div>
            <button>{"Register"}</button>
        </div>
      </form>
    }
}