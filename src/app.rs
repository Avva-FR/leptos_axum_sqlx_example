use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct User {
    username: String,
    email: String,
    password: String,
}

// Entry point for the application
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/leptos-axum-proj.css"/>
        <Title text="Welcome to Leptos"/>
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! {
                <ErrorTemplate outside_errors/>
            }
            .into_view()
        }>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/register" view=Register/>
                    <Route path="/about" view=About/> 
                </Routes>
            </main>
        </Router>
    }
}

// Navigation Component
#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <a href="/">Home</a> |
            <a href="/register">Register</a> | 
            <a href="/about">About</a> |
        </nav> 
    }
}

// Home Page Component
#[component]
fn HomePage() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    view! {
        <Nav />
        <h1>"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}

// Register Page Component
#[component]
fn Register() -> impl IntoView {
    let (email, set_email) = create_signal(String::new());
    let (username, set_username) = create_signal(String::new());
    let (pwd, set_pwd) = create_signal(String::new());
    let (confirmpwd, set_confirmpwd) = create_signal(String::new());

    // Use conditional compilation to access PgPool only on SSR
    #[cfg(feature = "ssr")]
    let pool = use_context::<PgPool>().expect("Pool not found");

    view! {
        <Nav />
        <form on:submit = move |ev| {
            ev.prevent_default();
            let username = username.get();
            let email = email.get();
            let pwd = pwd.get();
            let confirmpwd = confirmpwd.get();

            if pwd == confirmpwd {
                #[cfg(feature = "ssr")]
                spawn_local(async move {
                    match add_user_to_db(pool.clone(), username, email, pwd).await {
                        Ok(_) => logging::log!("Registered user"),
                        Err(err) => logging::log!("Failed to register user: {:?}", err),
                    }
                });
            } else {
                logging::log!("Passwords do not match!");
            }
        }>
        <label for="username"><b>Username</b></label>
        <input
            type="text"
            placeholder="Enter Username"
            name="username"
            id="username"
            on:input=move |ev| set_username(event_target_value(&ev)) 
            prop:value=username
            required
        />

        <label for="email"><b>Email</b></label>
        <input
            type="text"
            placeholder="Enter Email"
            name="email"
            id="email"
            on:input=move |ev| set_email(event_target_value(&ev))
            prop:value=email
            required
        />

        <label for="pwd"><b>Password</b></label>
        <input
            type="password"
            placeholder="Enter Password"
            name="pwd"
            id="pwd"
            on:input=move |ev| set_pwd(event_target_value(&ev))
            prop:value=pwd
            required
        />
        <label for="confirmpwd"><b>Confirm Password</b></label>
        <input
            type="password"
            placeholder="Please reenter the Password"
            name="confirmpwd"
            id="confirmpwd"
            on:input=move |ev| set_confirmpwd(event_target_value(&ev))
            prop:value=confirmpwd
            required
        />
    
        <button type="submit">Register</button>
    </form>
    }
}

#[cfg(feature = "ssr")]
pub async fn add_user_to_db(pool: PgPool, username: String, email: String, pwd: String) -> Result<(), sqlx::Error> {
    // Insert new user into db
    sqlx::query(
        "INSERT INTO users (username, email, pwd) VALUES ($1, $2, $3)"
    )
    .bind(username)
    .bind(email)
    .bind(pwd)
    .execute(&pool)
    .await?;

    Ok(())
}

#[component]
fn About() -> impl IntoView {
    view! {
        <Nav />
        <h3>About this project</h3>
        <p> This project is meant as a programming exercise combining multiple Rust-based technologies such as Leptos, SQLx, and so on in a web
        application. I will probably not make it look appealing. Bootstrap or any other prebaked frontend framework could just be used instead.
        </p>
    }
}