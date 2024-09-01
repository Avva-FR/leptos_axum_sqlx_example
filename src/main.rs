use axum::{
    extract::Extension,
    Router,
    routing::get,
};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use leptos_axum_proj::app::ssr::create_db_conn;
use leptos_axum_proj::app::*;
use leptos_axum_proj::fileserv::file_and_error_handler;
use std::sync::Arc;
use llm::models::Llama;

type AppState = Arc<Llama>;

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load configuration
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    // Load the LLM model
    let model = Arc::new(get_language_model());

    // DB: Connection pool for PostgreSQL
    let db_conn_pool = create_db_conn().await.expect("DB connection failed");
    match sqlx::migrate!("./migrations").run(&db_conn_pool).await {
         Ok(_) => println!("Migrations applied successfully"),
         Err(err) => eprintln!("Migration error: {:?}", err),
    }

    // Build our application with a route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" })) // Add a dummy route for testing
        .app_data(Extension(model.clone())) // Provide the model as application state
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

// Configuration and model loading functions
cfg_if! {
    if #[cfg(feature = "ssr")] {
        use llm::models::Llama;
        use std::env;
        use dotenv::dotenv;
        pub fn get_language_model() -> Llama {
            use std::path::PathBuf;
            dotenv().ok();
            let model_path = env::var("LLM_PATH").expect("LLM_PATH must be set");
            let model_parameters = llm::ModelParameters {
                prefer_mmap: true,
                context_size: 2048,
                lora_adapters: None,
                use_gpu: true,
                gpu_layers: None,
                rope_overrides: None,
                n_gqa: None,
            };

            llm::load::<Llama>(
                &PathBuf::from(&model_path),
                llm::TokenizerSource::Embedded,
                model_parameters,
                llm::load_progress_callback_stdout,
            )
            .unwrap_or_else(|err| {
                panic!("Failed to load model from {model_path:?}: {err}")
            })
        }
    }
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
}