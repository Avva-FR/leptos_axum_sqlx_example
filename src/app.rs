use crate::components::{
    about::About, add_blog::BlogAddForm, blog::Blog, home::Home, login::Login, register::Register,
};
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
        // stuff this into an envvar for prod
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
        //Bootstrap
        //<Stylesheet id="bootstrap" href="/bootstrap/css/bootstrap.css"/>
        // extra css
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
        // app
            <main>
                <Routes>
                    <Route path="" view=Home/>
                    <Route path="/blog" view=Blog/>
                    <Route path="/register" view=Register/>
                    <Route path="/login" view=Login/>
                    // <Route path="/jippity" view=Jippity/>
                    <Route path="/about" view=About/>
                    <Route path="/addblog" view=BlogAddForm/>
                </Routes>
            </main>
        </Router>
        // bootstrap js dependencies
    }
}
#[cfg(test)]
mod tests {
    use super::ssr::create_db_conn;
    use sqlx::PgPool;
    // test if db connection Pool is established
    #[tokio::test]
    async fn test_create_db_conn_success() {
        let result = create_db_conn().await;
        assert!(result.is_ok());
    }
    // test that db conn wont be established for wrong url
    #[tokio::test]
    async fn test_create_db_conn_failure() {
        let incorrect_url = "postgres://wronguser:wrongpwd@localhost:5432/wrong_db";
        let pool = PgPool::connect(incorrect_url).await;
        assert!(pool.is_err());
    }
}
