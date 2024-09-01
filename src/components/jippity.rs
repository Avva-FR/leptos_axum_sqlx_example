use serde::{Deserialize, Serialize};
use leptos::{component, view, IntoView, create_signal, create_action, create_effect, ReadSignal, create_node_ref};
use leptos::html::{Div, Input};
use cfg_if::cfg_if;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub text: String,
    pub from_llm: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>,
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation {
            messages: Vec::new(),
        }
    }
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use llm::models::Llama;
        use tokio::sync::mpsc;
        use std::sync::Arc;
        use leptos::{component, view, IntoView, create_signal, create_action, create_effect, ReadSignal, create_node_ref};
        use leptos::html::{Div, Input};

        // Assuming `tx` is created and used properly within the scope of the function
        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            tx: mpsc::Sender<String>,
            runtime: &'a mut tokio::runtime::Runtime,
        ) -> impl FnMut(llm::InferenceResponse) -> Result<llm::InferenceFeedback, std::convert::Infallible> + 'a {
            use llm::InferenceFeedback::{Halt, Continue};

            move |resp| -> Result<llm::InferenceFeedback, std::convert::Infallible> {
                match resp {
                    llm::InferenceResponse::InferredToken(t) => {
                        let mut reverse_buf = buf.clone();
                        reverse_buf.push_str(t.as_str());
                        if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                            buf.clear();
                            return Ok(Halt);
                        } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                            buf.push_str(t.as_str());
                            return Ok(Continue);
                        }

                        let text_to_send = if buf.is_empty() {
                            t.clone()
                        } else {
                            reverse_buf
                        };

                        let tx_cloned = tx.clone();
                        runtime.block_on(async move {
                            tx_cloned.send(text_to_send).await.expect("issue sending on channel");
                        });

                        Ok(Continue)
                    }
                    llm::InferenceResponse::EotToken => Ok(Halt),
                    _ => Ok(Continue),
                }
            }
        }
    }
}

#[server(Jippity, "/jippity")]
pub async fn converse(
    Extension(model): Extension<Arc<Llama>>,  // Extract the model from state
    prompt: Conversation,
) -> Result<String, ServerFnError> {
    let mut runtime = Runtime::new().expect("Failed to create runtime");

    let jippity = "Jippity:";
    let user_name = "User";
    let mut chat = String::new();

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let cur_ln = if message.from_llm {
            format!("{user_name}: {msg}\n")
        } else {
            format!("{jippity}: {msg}\n")
        };
        chat.push_str(&cur_ln);
    }

    let mut res = String::new();
    let mut rng = rand::thread_rng();
    let mut buf = String::new();
    let mut session = model.start_session(Default::default());

    let (tx, mut rx) = mpsc::channel(1);

    let stop_sequence = String::from(user_name);
    session
        .infer(
            model.as_ref(),
            &mut rng,
            &InferenceRequest {
                prompt: format!("{jippity}\n{chat}\n{user_name}").as_str().into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(
                stop_sequence,
                &mut buf,
                tx, 
                &mut runtime,
            ),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    while let Some(message) = rx.recv().await {
        res.push_str(&message);
    }

    Ok(res)
}

// Client-side components

#[component]
pub fn Jippity() -> impl IntoView {
    let (conversation, set_conversation) = create_signal(Conversation::new());

    let send = create_action(move |new_msg: &String| {
        let user_msg = Message {
            text: new_msg.clone(),
            from_llm: false,
        };
        set_conversation.update(move |conv| 
            conv.messages.push(user_msg));
        // Call the server function here
        converse(conversation.get())
    });

    create_effect(move |_| {
        if let Some(_) = send.input().get() {
            let response_msg = Message {
                text: String::from("..."),
                from_llm: true,
            };
            set_conversation.update(move |conv| {
                conv.messages.push(response_msg);
            });
        }
    });

    create_effect(move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |conv| {
                conv.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! {
        <Nav />
        <h1>"The I in LLM stands for Intelligence"</h1>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();

    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
          <div class="b-screen pb-24 w-full flex flex-col overflow-y-auto border border-gray-300 rounded p-5 border-zinc-700 bg-zinc-900" node_ref=chat_div_ref>
          {move || conversation.get().messages.iter().map(move |message| {
              let class_str = if !message.from_llm { format!("max-w-md p-4 mb-5 rounded-lg self-end bg-blue-500 text-white") }
              else { format!("max-w-md p-4 mb-5 rounded-lg self-start bg-zinc-700 text-white") };
              view! {
                <div class={class_str}>
                  {message.text.clone()}
                </div>
              }
            }).collect::<Vec<_>>()
          }
        </div>
    }
}

#[component]
pub fn TypeArea(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref = create_node_ref::<Input>();

    view! {
        <div class = "h-24 w-full fixed bottom-0 flex justify-center items-center p-5 border-t bg-zinc-900 border-zinc-700">
            <form on:submit = move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("Input doesn't exist");
                send.dispatch(input.value());
                input.set_value("");
            }>
            <input class="w-2/3 p-4 border rounded-full input-field bg-zinc-700 border-zinc-700 text-white" type="text" placeholder="Enter your prompt" node_ref=input_ref/>
            <button class="h-full p-4 rounded-full cursor-pointer bg-green-700 text-white" type="submit">
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                <path stroke-linecap="round" stroke-linejoin="round" d="M4.5 12h15m0 0l-6.75-6.75M19.5 12l-6.75 6.75" />
                </svg>
            </button>
        </form>
    </div>
    }
}