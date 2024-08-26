#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::{Router, Extension};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_axum_proj::app::*;
    use leptos_axum_proj::fileserv::file_and_error_handler;
    use sqlx::postgres::PgPoolOptions;
    //use axum::AddExtensionLayer;

    // Load configuration
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

     // DB: Connection pool for PostgreSQL
     let url = "postgres://avva:SomeFancyPwd@localhost:5432/leptos_proj";
     let pool = PgPoolOptions::new()
         .max_connections(5) // Set a maximum number of connections
         .connect(url)
         .await?;
 
     match sqlx::migrate!("./migrations").run(&pool).await {
         Ok(_) => println!("Migrations applied successfully"),
         Err(err) => eprintln!("Migration error: {:?}", err),
     }
 
    // Build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options)
        .layer(Extension(pool)); // Add pool to the app state

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    logging::log!("listening on http://{}", &addr);
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
    
    Ok(())
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
