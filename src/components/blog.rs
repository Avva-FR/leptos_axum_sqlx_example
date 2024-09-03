use crate::components::nav::Nav;
use leptos::*;

// basic model of a blog post 
#[derive(Debug, Clone)]
pub struct BlogEntry {
    pub img_path: String,
    pub title: String,
    pub author: String,
    pub date: String,
    pub content: String,
}

pub fn parse_md_content(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}


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
                    <label for="floatingTextarea">Leave a comment here</label>
                </div>
            </div>
        </div>

        

    }
}