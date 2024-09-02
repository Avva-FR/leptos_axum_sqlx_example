use leptos::*;

#[component]
pub fn Nav() -> impl IntoView {
    let _logo_svg = include_str!("../../style/icons/logo.svg");
    let person_circle_svg = include_str!("../../style/icons/person-circle.svg");

    view! {
        <nav class="navbar navbar-expand-lg bg-body-tertiary">
            <div class="container-fluid">
                // logo
                <a class="navbar-brand" href="/">
                    <svg class="logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 285.07 239.05" width="28.507mm" height="23.905mm">
                        <g transform="translate(-124.88 -75.808)" stroke-width="2">
                            <path d="m127.57 277.6h81.748l58.211-84.15 59.012 84.151 79.863 1.9772-139.97-199.91z" fill="#333" stroke="#333"/>
                            <path d="m407.26 113.06h-81.748l-58.211 84.15-59.012-84.151h-81.489l140.1 200.05z" stroke="#000"/>
                            <path d="m406.46 279.6-81.478-1e-3 -99.349-141.79 40.737-58.259z" fill="#333" stroke="#333"/>
                        </g>
                    </svg>
                </a>
    
                // mobile andies
                <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                </button>
    
                // centered part 
                <div class="collapse navbar-collapse">
                    <ul class="navbar-nav mx-auto d-flex w-100 justify-content-center">
                        <li class="nav-item">
                            <a class="nav-link" href="/blog">Blog</a>
                        </li>
                        <li class="nav-item">
                            <a class="nav-link" href="/about">About</a>
                        </li>
                    </ul>
    
                    // account part
                    <ul class="navbar-nav ms-auto">
                        <li class="nav-item dropdown">
                            <a class="nav-link dropdown-toggle" id="navbarDropdown" aria-expanded="false">
                                <span inner_html=person_circle_svg></span>
                                Account
                            </a>
                            <ul class="dropdown-menu" id = "account-dropdown" aria-labelledby="navbarDropdown">
                                <li><a class="dropdown-item" href="/register">Register</a></li>
                                <li><a class="dropdown-item" href="/login">Login</a></li>
                            </ul>
                        </li>
                    </ul>

                </div>
                
            </div>
        </nav>
    }
}
