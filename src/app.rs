pub mod components;

use leptos::*;
use leptos_meta::*;

use crate::api::converse;
use crate::app::components::chat_area::ChatArea;
use crate::app::components::type_area::TypeArea;
use crate::model::conversation::{Conversation, Message};

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);

    let (conversation, set_conversation) = create_signal(cx, Conversation::new());

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            text: new_message.clone(),
            user: true,
        };
        set_conversation.update(move |c| {
            c.messages.push(user_message);
        });

        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| {
                c.messages.push(model_message);
            });
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| {
                c.messages.last_mut().unwrap().text = response;
            });
        }
    });

    view! { cx,
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Rusty Llama!"/>
        <ChatArea conversation/>
        <TypeArea send/>
    }
}
