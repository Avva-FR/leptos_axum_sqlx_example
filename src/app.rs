use crate::components::{home::Home, about::About, register::Register, login::Login};
use crate::error_template::{AppError, ErrorTemplate};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

// dont touch this sqlx imports into any file with #[components] will break compilation 
// import this config instead
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
                    <Route path="" view=Home/>
                    <Route path="/register" view=Register/>
                    <Route path="/login" view=Login/>
                    <Route path="/about" view=About/> 
                </Routes>
            </main>
        </Router>
    }
}

