use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let _logo_svg = include_str!("../../style/icons/logo.svg");
    let person_circle_svg = include_str!("../../style/icons/person-circle.svg");

    view! {
        
        <nav>
          
        <a href="/">
            <svg class="logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 285.07 239.05" width="28.507mm" height="23.905mm">
                <g transform="translate(-124.88 -75.808)" stroke-width="2">
                    <path d="m127.57 277.6h81.748l58.211-84.15 59.012 84.151 79.863 1.9772-139.97-199.91z" fill="#333" stroke="#333"/>
                    <path d="m407.26 113.06h-81.748l-58.211 84.15-59.012-84.151h-81.489l140.1 200.05z" stroke="#000"/>
                    <path d="m406.46 279.6-81.478-1e-3 -99.349-141.79 40.737-58.259z" fill="#333" stroke="#333"/>
                </g>
            </svg>
        </a>
            
            <a href="/register">
                <span inner_html=person_circle_svg></span>
                Register
            </a>
            |
            <a href="/login">Login</a>
            |
            <a href="/jippity">Jippity</a>
            | 
            <a href="/about">About</a>
            |
        </nav> 
    }
}