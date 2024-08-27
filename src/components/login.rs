use leptos::ev::SubmitEvent;
use leptos::*;
use leptos_router::*;
use crate::components::nav::Nav;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use crate::app::ssr::create_db_conn;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Login {
    username: String,
    pwd: String,
}

#[cfg(feature = "ssr")]
pub async fn check_user_credentials(login: Login) -> Result<bool, ServerFnError> {
    let pool = create_db_conn().await?;

    // query user & pwd
    let result: Option<(String,)> = sqlx::query_as(
        "SELECT username FROM user_table WHERE username = $1 AND pwd = $2"
    )
    .bind(&login.username)
    .bind(&login.pwd)
    .fetch_optional(&pool)
    .await?;

    if let Some(_) = result {
        println!("User found in db");
        Ok(true)
    } else {
        println!("User not found in db");
        Ok(false)
    }
}

#[server(LoginAction, "/login")]
pub async fn pass_login_input(login: Login) -> Result<(), ServerFnError> {
    let user_exists = check_user_credentials(login.clone()).await?;
    
    if user_exists {
        println!("Login successful for user: {}", login.username);
        Ok(())
    } else {
        eprintln!("Login failed: invalid username or password");
        Err(ServerFnError::ServerError("Invalid username or password".to_string()))
    }
}

#[component]
pub fn Login() -> impl IntoView {
    let login_action = create_server_action::<LoginAction>();
    
    let (username, set_username) = create_signal(String::new());
    let (pwd, set_pwd) = create_signal(String::new());

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
    
        login_action.dispatch(LoginAction { 
            login: Login {
                username: username.get(),
                pwd: pwd.get(),
            }
        });
    };
    
    view! {
        <Nav />
        <h2>"Login"</h2>
        <ActionForm action=login_action on:submit=on_submit>
            <label for="username"><b>"Username"</b></label>
            <input
                type="text"
                placeholder="Enter Username"
                id="username"
                name="username"
                on:input= move |ev| set_username(event_target_value(&ev))
                required
            />

            <label for="pwd"><b>"Password"</b></label>
            <input
                type="password"
                placeholder="Enter Password"
                id="pwd"
                name="pwd"
                on:input = move |ev| set_pwd(event_target_value(&ev))
                required
            />
            <button type="submit">"Login"</button>
        </ActionForm>
    }
}