use leptos::*;


#[component]
pub fn register() -> impl IntoView {
    view! {
        <label for="email"><b>Email</b></label>
        <input type="text" placeholder="Enter Email" name="email" id="email" />
    }
}
