use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct User {
    username: String,
    email: String,
    password: String,
}

// adding user to db
#[cfg(feature = "ssr")]
pub async fn add_user_to_db(user: &User) -> Result<(), ServerFnError>  {
    use self::ssr::*;
    let pool = create_db_conn().await?;
    // fake API delay
    std::thread::sleep(std::time::Duration::from_millis(1250));

    // Insert new user into db
    let query = "INSERT INTO user_table (username, email, password) VALUES ($1, $2, $3)";
    let query_result  = sqlx::query(&query)
    .bind(&user.username)
    .bind(&user.email)
    .bind(&user.password)
    .execute(&pool)
    .await;
    match query_result {
        Ok(_row) => Ok(()),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
    }

// this might need to be a single connection instead of a pool at creation time 
#[cfg(feature = "ssr")]
pub mod ssr {
    use leptos::ServerFnError;
    use sqlx::postgres::PgPool;
    pub async fn create_db_conn() -> Result<PgPool, ServerFnError> {
        let url = "postgres://avva:SomeFancyPwd@localhost:5432/leptos_proj";
        let pool = PgPool::connect(url).await?;
        Ok(pool)
    }
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
    view! {
        <Nav />
        <form on:submit = move |ev| {
            ev.prevent_default();
            if pwd.get() == confirmpwd.get() {
                #[cfg(feature = "ssr")]
                spawn_local(async move {
                    let user = User {
                        username: username.get(),
                        email: email.get(),
                        password: pwd.get(),
                    };
                    match add_user_to_db(&user).await {
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