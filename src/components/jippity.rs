use leptos::*;
use serde::{Serialize, Deserialize};
use crate::components::nav::Nav;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub text: String,
    pub from_llm: bool,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}
impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new()
       }
    }
}
#[server(Converse, "/jippity")]
pub async fn converse(prompt: Conversation) -> Result<String, ServerFnError> {
    use llm::models::Llama;
    use llm::KnownModel;
    use axum::extract;
    use axum::ConnectInfo;

    let model = extract(|data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await.unwrap();

    let jippity = "Jippity:";
    let user_name = "User";
    let mut chat = String::new();
    // switch 
    for message in prompt.message.into_iter() {
        let msg = message.text;
        let cur_ln = if message.user {
            format!("{jippity}:{mgs}\n");
        } else {
            format!("{user_name}:{msg}\n");
        };
        chat.push(&cur_ln);
    }
    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    let mut session = model.start_session(Default::default());

    session.infer(
        model.as_ref(),
        &mut rng,
        &llm::InferenceRequest {
            prompt: format!("{jippity}\n{history}\n{user_name}")
            .as_str()
            .into(),
            parameters: Some(&llm::InferenceParameters::default()),
            play_back_previous_tokens: false,
            maximum_token_count: None,
        },
        &mut Default::default(),
        inference_callback(String::from(user_name), &mut self, &mut res),
    )
    .unwrap_or_else(|e| panic!("{e}"));


    Ok(String::from("shoulndt be reachable"));
}

#[component]
pub fn Chat() -> impl IntoView {
    let (conversation, set_conversation) = create_signal(Conversation::new());
    let send_msg = create_action(move |new_msg : &String| {
        let user_msg = Message {
            text: new_msg.clone(),
            from_llm: false,
            };
        set_conversation.update(move |conv| {
            conv.messages.push(user_msg)
        });
        // TODO
    });
    
    view! {
        <Nav />
        <h1>"The I in LLM stands for Intelligence"</h1>
        <ChatArea conversation/>
        <InputArea/>
    }
}

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    view! {
        <div class=chat>
            { move || conversation.get().messages.iter().map(move |message| {
                let class_str = if message.user {}
            }) }
        <\div>
    }
}