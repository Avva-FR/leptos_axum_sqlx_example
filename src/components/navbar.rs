use leptos::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li><a href="#home">"Home"</a></li>
                <li><a href="#about">"About"</a></li>
                <li><a href="#contact">"Contact"</a></li>
            </ul>
        </nav>
    }
}