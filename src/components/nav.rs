use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <nav>
            |
            <a href="/">Home</a>
            |
            <a href="/register">Register</a>
            |
            <a href="/login">Login</a>
            | 
            <a href="/about">About</a>
            |
        </nav> 
    }
}