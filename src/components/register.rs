use crate::components::nav::Nav;
use leptos::*;
use leptos_router::*;
use leptos::{create_server_action, ServerFnError};
use leptos::ev::SubmitEvent;
use serde::{Deserialize, Serialize};
use regex::Regex;
#[cfg(feature = "ssr")]
use crate::app::ssr::create_db_conn;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    username: String,
    email: String,
    pwd: String,
}

pub fn validate_email(email: &str) -> Result<(), &'static str>{
    // refer to https://www.ietf.org/rfc/rfc5322.txt
    let re = Regex::new("(?:[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+(?:\\.[a-zA-Z0-9!#$%&'*+/=?^_`{|}~-]+)*
    |\"(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21\\x23-\\x5b\\x5d-\\x7f]|\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])*\")
    @(?:(?:[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?\\.)+[a-zA-Z0-9](?:[a-zA-Z0-9-]*[a-zA-Z0-9])?
    |\\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\\.){3}
    (?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])
    |[a-zA-Z0-9-]*[a-zA-Z0-9]:(?:[\\x01-\\x08\\x0b\\x0c\\x0e-\\x1f\\x21-\\x5a\\x53-\\x7f]
    |\\\\[\\x01-\\x09\\x0b\\x0c\\x0e-\\x7f])+)\\])").unwrap();
    if re.is_match(email) {
        Ok(())
    } else {
        Err("Invalid Username")
    }
}
pub fn validate_password(pwd: &str) -> Result<(), &'static str> {
    // BSI recommendation
    let re = Regex::new("^(?=.*[a-z]+)(?=.*[A-Z]+)(?=.*\\d+)(?=.*\\W+).{8,}$").unwrap();
    if re.is_match(pwd) {
        Ok(())
    } else {
        Err("Invalid password. It must be at least 8 charact    ers long and contain at least one letter and one number.")
    }
}

// this is not redudnant dont call add_user_to_db or 
#[server(RegisterUser, "/register")]
pub async fn pass_register_input(user: User) -> Result<(), ServerFnError> {
    add_user_to_db(user).await
}

#[cfg(feature = "ssr")]
pub async fn add_user_to_db(user: User) -> Result<(), ServerFnError> {    
    let pool = create_db_conn().await?;

    // check if the user allready exists
    let exists: (bool,) = sqlx::query_as(
        "SELECT EXISTS (SELECT 1 FROM user_table WHERE username = $1)"
    )
    .bind(&user.username)
    .fetch_one(&pool)
    .await?;

    if exists.0 {
        eprintln!("Error: User already exists: username = '{}'", user.username);
        return Err(ServerFnError::ServerError("User already exists".to_string()));
    }
    // if the username does not allready exist we store it in the db
    let query = "INSERT INTO user_table (username, email, pwd) VALUES ($1, $2, $3)";
    sqlx::query(query)
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.pwd)
        .execute(&pool)
        .await?;
    
    println!("User added successfully: {:?}", user);

    Ok(())
}

// Register Page Component
// dont rename name part of html components or the hook method will kill it self
// the html name of the input field needs have the exact name as the argument of the server hook fn
#[component]
pub fn Register() -> impl IntoView {
    let register_action = create_server_action::<RegisterUser>();

    let (username, set_username) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (pwd, set_pwd) = create_signal(String::new());
    let (confirmpwd, set_confirmpwd) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default(); 

        if pwd.get() != confirmpwd.get() {
            println!("Passwords do not match!");
        } else {
            // dont touch the app::RegisterUser
            register_action.dispatch(RegisterUser { user: User {
                username: username.get().clone(),
                email: email.get().clone(),
                pwd: pwd.get().clone(),
        } });

        }
    };

    view! {
        <Nav />
        <h2>"Register"</h2>
        <ActionForm action=register_action on:submit=on_submit>
            <label for="username"><b>"Username"</b></label>
            <input
                type="text"
                placeholder="Enter Username"
                id="username"
                name="username"
                on:input=move |ev| set_username(event_target_value(&ev))
                required
            />

            <label for="email"><b>"Email"</b></label>
            <input
                type="email"
                placeholder="Enter Email"
                id="email"
                name="email"
                on:input=move |ev| set_email(event_target_value(&ev))
                required
            />

            <label for="pwd"><b>"Password"</b></label>
            <input
                type="password"
                placeholder="Enter Password"
                id="pwd"
                name="pwd"
                on:input=move |ev| set_pwd(event_target_value(&ev))
                required
            />

            <label for="confirmpwd"><b>"Confirm Password"</b></label>
            <input
                type="password"
                placeholder="Please reenter the Password"
                id="confirmpwd"
                name="confirmpwd"
                on:input=move |ev| set_confirmpwd(event_target_value(&ev))
                required
            />

            <button type="submit">"Register"</button>
        </ActionForm>
    }
}