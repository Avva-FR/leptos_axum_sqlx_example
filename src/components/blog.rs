use crate::components::nav::Nav;
use leptos::*;

#[component]
pub fn Blog() -> impl IntoView {
    view! {
        <Nav />
        <div class="blog-container">
            // Content Wrapper (Main Blog Section + Sidebar)
            <div class="content-wrapper">
                // Main Blog Section
                <div class="main-blog">
                    <h1 class="blog-title">"The Title of the Blog Post"</h1>
                    <img src="/static/images/blog-header.jpg" alt="Blog Header" class="blog-header-image"/>
                    <p class="blog-content">
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
                        Quisque sit amet accumsan tortor. Maecenas at turpis neque. 
                        Etiam et sapien a orci pharetra sodales in nec mauris. Donec sed lacus magna. 
                        Aliquam erat volutpat. Nam vitae suscipit risus. Vivamus quis nulla lectus."
                    </p>
                </div>

            // comment section
            <div class="form-floating">
                <textarea class="form-control" placeholder="Leave a comment here" id="floatingTextarea"></textarea>
                <label for="floatingTextarea">Comments</label>
            </div>

        </div>
    </div>
    }
}