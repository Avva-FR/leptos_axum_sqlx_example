#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    use axum::Router;
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use leptos_axum_proj::app::ssr::create_db_conn;
    use leptos_axum_proj::app::*;
    use leptos_axum_proj::fileserv::file_and_error_handler;
    // db conn defined in app.rs
    
    // Load configuration
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

     // DB: Connection pool for PostgreSQL
    let db_conn_pool = create_db_conn().await.expect("DB connection failed");
 
    match sqlx::migrate!("./migrations").run(&db_conn_pool).await {
         Ok(_) => println!("Migrations applied successfully"),
         Err(err) => eprintln!("Migration error: {:?}", err),
    }
 
    // Build our application with a route
    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

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
