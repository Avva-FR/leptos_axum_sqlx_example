#[cfg(feature = "ssr")]
use crate::app::ssr::create_db_conn;
use crate::components::nav::Nav;
use leptos::ev::Event;
use leptos::ev::SubmitEvent;
use leptos::*;
use leptos::{create_server_action, ServerFnError};
use leptos_router::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{FileList, FileReader, HtmlInputElement};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct BlogEntry {
    title: String,
    author: String,
    date: String,
    img: Vec<u8>,
    content: String,
}

#[server(AddBlog, "/addblog")]
pub async fn pass_form_input(blog_entry: BlogEntry) -> Result<(), ServerFnError> {
    insert_blog_entry(blog_entry).await
}

pub fn parse_md_content(content: &str) -> String {
    let parser = pulldown_cmark::Parser::new(content);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, parser);
    html_output
}

#[cfg(feature = "ssr")]
pub async fn insert_blog_entry(entry: BlogEntry) -> Result<(), ServerFnError> {
    let pool = create_db_conn().await?;
    let title_exists: (bool,) =
        sqlx::query_as("SELECT EXISTS (SELECT 1 FROM blog_entries WHERE title = $1)")
            .bind(&entry.title)
            .fetch_one(&pool)
            .await?;

    if title_exists.0 {
        eprintln!(
            "Error: Blog entry already exists with title '{}'",
            entry.title
        );
        return Err(ServerFnError::ServerError(
            "Blog entry with this title already exists".to_string(),
        ));
    }

    let query = "
        INSERT INTO blog_entries (title, author, date, content, img_path)
        VALUES ($1, $2, $3, $4, $5)
    ";

    sqlx::query(query)
        .bind(&entry.title)
        .bind(&entry.author)
        .bind(&entry.date)
        .bind(&entry.content)
        .bind(&entry.img)
        .execute(&pool)
        .await?;

    println!("Blog entry added successfully");

    Ok(())
}

#[component]
pub fn BlogAddForm() -> impl IntoView {
    let submit_action = create_server_action::<AddBlog>();

    let (title, set_title) = create_signal(String::new());
    let (author, set_author) = create_signal(String::new());
    let (content, set_content) = create_signal(String::new());
    let (img_file, set_img_file) = create_signal(None::<Vec<u8>>);

    let on_submit = move |ev: SubmitEvent| {
        println!("submit pressed");
        ev.prevent_default();

        submit_action.dispatch(AddBlog {
            blog_entry: BlogEntry {
                title: title.get().clone(),
                author: author.get().clone(),
                date: String::from("this is not a date @Todo"),
                img: img_file.get().clone().unwrap_or_default(),
                content: content.get().clone(),
            },
        });
    };
    // checked db this inserted as intended
    let on_img_file_change = move |ev: Event| {
        let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let files: FileList = input.files().unwrap();

        if files.length() == 0 {
            println!("file not found");
            return;
        }

        let file = files.get(0).unwrap();
        let reader = Rc::new(RefCell::new(FileReader::new().unwrap()));

        let set_img_file_clone = set_img_file.clone();
        let reader_clone = reader.clone();
        let on_load = Closure::wrap(Box::new(move |_: Event| {
            let reader = reader_clone.borrow();
            let array_buffer = reader
                .result()
                .unwrap()
                .dyn_into::<js_sys::ArrayBuffer>()
                .unwrap();
            let vec = js_sys::Uint8Array::new(&array_buffer).to_vec();
            set_img_file_clone.set(Some(vec));
        }) as Box<dyn FnMut(_)>);

        reader
            .borrow_mut()
            .set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();

        reader.borrow_mut().read_as_array_buffer(&file).unwrap();
    };

    // @ this should read the markdown file
    // and read it to a string then store the string
    let on_md_file_change = move |ev: Event| {
        let input = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let files: FileList = input.files().unwrap();
    
        if files.length() == 0 {
            println!("Markdown file not found");
            return;
        }
    
        let file = files.get(0).unwrap();
        let reader = Rc::new(RefCell::new(FileReader::new().unwrap()));
    
        let set_content_clone = set_content.clone();
        let reader_clone = reader.clone();
        let on_load = Closure::wrap(Box::new(move |_: Event| {
            let reader = reader_clone.borrow();
            let result = reader.result().unwrap();
            let text_content = result.as_string().unwrap();
            set_content_clone.set(text_content);
            println!("Markdown content read: {:?}", text_content);
        }) as Box<dyn FnMut(_)>);
    
        reader
            .borrow_mut()
            .set_onload(Some(on_load.as_ref().unchecked_ref()));
        on_load.forget();
    
        reader.borrow_mut().read_as_text(&file).unwrap();
    };

    view! {
        <Nav />
        <ActionForm action=submit_action on:submit=on_submit>
            <div class="row">
                <div class="col">
                    <input type="text" class="form-control" placeholder="Author" aria-label="Author"
                        on:input=move |ev| set_author(event_target_value(&ev))
                    />
                </div>
                <div class="col">
                    <input type="text" class="form-control" placeholder="Title" aria-label="Title"
                        on:input=move |ev| set_title(event_target_value(&ev))
                    />
                </div>
            </div>
            <div class="row">
                <div class="col">
                    <div class="input-group">
                        <label class="input-group-text" for="inputMD">Markdown file</label>
                        <input type="file" class="form-control" id="inputMD"
                            on:change=on_md_file_change
                        />
                    </div>
                </div>
                <div class="col">
                    <div class="input-group">
                        <label class="input-group-text" for="inputPic">Picture</label>
                        <input type="file" class="form-control" id="inputPic"
                            on:change=on_img_file_change
                        />
                    </div>
                </div>
            </div>

            <div class="col-12 justify-content-center">
                <button class="btn btn-primary" type="submit">Submit form</button>
            </div>
        </ActionForm>
    }
}
