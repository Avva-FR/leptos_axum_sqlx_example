use leptos::*;
use crate::components::nav::Nav;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Nav />
        <h3>About this project</h3>
        <p>This project is meant as a programming exercise combining multiple Rust-based technologies such as Leptos, SQLx, and so on in a web
        application. I will probably not make it look appealing. Bootstrap or any other prebaked frontend framework could just be used instead.</p>
    }
}