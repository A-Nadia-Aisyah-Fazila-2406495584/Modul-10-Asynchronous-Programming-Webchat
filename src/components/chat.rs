use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{User, services::websocket::WebsocketService};
use crate::services::event_bus::EventBus;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
    #[serde(skip)]
    timestamp: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn get_time() -> String {
    let date = js_sys::Date::new_0();
    let hours = date.get_hours();
    let minutes = date.get_minutes();
    format!("{:02}:{:02}", hours, minutes)
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

fn create(ctx: &Context<Self>) -> Self {
    let (user, _) = ctx
        .link()
        .context::<User>(Callback::noop())
        .expect("context to be set");
    let wss = WebsocketService::new();
    let username = user.username.borrow().clone();

    let message = WebSocketMessage {
        message_type: MsgTypes::Register,
        data: Some(username.to_string()),
        data_array: None,
    };

    if let Ok(_) = wss
        .tx
        .clone()
        .try_send(serde_json::to_string(&message).unwrap())
    {
        log::debug!("message sent successfully");
    }

    Self {
        users: vec![],
        messages: vec![],
        chat_input: NodeRef::default(),
        wss,
        _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
    }
}

fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
        Msg::HandleMsg(s) => {
            let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
            match msg.message_type {
                MsgTypes::Users => {
                    let users_from_message = msg.data_array.unwrap_or_default();
                    self.users = users_from_message
                        .iter()
                        .map(|u| UserProfile {
                            name: u.into(),
                            avatar: format!(
                                "https://robohash.org/{}.png?set=set4",
                                u
                            )
                            .into(),
                        })
                        .collect();
                    return true;
                }
                MsgTypes::Message => {
                    let mut message_data: MessageData =
                        serde_json::from_str(&msg.data.unwrap()).unwrap();
                    message_data.timestamp = get_time();
                    self.messages.push(message_data);
                    return true;
                }
                _ => {
                    return false;
                }
            }
        }
        Msg::SubmitMessage => {
            let input = self.chat_input.cast::<HtmlInputElement>();
            if let Some(input) = input {
                let message = WebSocketMessage {
                    message_type: MsgTypes::Message,
                    data: Some(input.value()),
                    data_array: None,
                };
                if let Err(e) = self
                    .wss
                    .tx
                    .clone()
                    .try_send(serde_json::to_string(&message).unwrap())
                {
                    log::debug!("error sending to channel: {:?}", e);
                }
                input.set_value("");
            };
            false
        }
    }
}

fn view(&self, ctx: &Context<Self>) -> Html {
    let submit = ctx.link().callback(|_| Msg::SubmitMessage);
    html! {
        <div class="flex w-screen" style="background:#FDF6F0; font-family:'Playfair Display',serif;">

            <div style="
                width:220px;
                flex-shrink:0;
                height:100vh;
                background:#F9EAE1;
                border-right:1px solid #E3B6B6;
                display:flex;
                flex-direction:column;
            ">
                <div style="
                    padding:20px 16px 12px;
                    font-family:'Parisienne',cursive;
                    font-size:26px;
                    color:#B44A4A;
                    border-bottom:1px solid #E3B6B6;
                    text-align:center;
                ">
                    {"Friends"}
                </div>
                {
                    self.users.clone().iter().map(|u| {
                        html!{
                            <div style="
                                display:flex;
                                align-items:center;
                                margin:10px 12px;
                                background:rgba(255,255,255,0.7);
                                border:1px solid #E3B6B6;
                                border-radius:16px;
                                padding:8px 10px;
                            ">
                                <img
                                    style="width:40px;height:40px;border-radius:50%;border:2px solid #E3B6B6;"
                                    src={u.avatar.clone()}
                                    alt="avatar"
                                />
                                <div style="margin-left:10px;">
                                    <div style="font-size:13px;color:#5A3A32;font-weight:bold;">
                                        {capitalize(&u.name)}
                                    </div>
                                </div>
                            </div>
                        }
                    }).collect::<Html>()
                }
            </div>

            <div style="flex:1;display:flex;flex-direction:column;height:100vh;">

                <div style="
                    height:56px;
                    background:#F2D6D6;
                    border-bottom:1px solid #E3B6B6;
                    display:flex;
                    align-items:center;
                    padding:0 20px;
                ">
                    <span style="
                        font-family:'Parisienne',cursive;
                        font-size:28px;
                        color:#B44A4A;
                    ">
                        {"Chatify"}
                    </span>
                </div>

                <div style="
                    flex:1;
                    overflow-y:auto;
                    padding:16px;
                    border-bottom:1px solid #E3B6B6;
                ">
                    {
                        self.messages.iter().map(|m| {
                            let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                            html!{
                                <div style="
                                    display:flex;
                                    align-items:flex-end;
                                    margin:12px 0;
                                    max-width:60%;
                                ">
                                    <img
                                        style="width:32px;height:32px;border-radius:50%;border:2px solid #E3B6B6;margin-right:10px;"
                                        src={user.avatar.clone()}
                                        alt="avatar"
                                    />
                                    <div style="
                                        background:rgba(255,255,255,0.75);
                                        border:1px solid #E3B6B6;
                                        border-radius:16px 16px 16px 4px;
                                        padding:10px 14px;
                                        box-shadow:0 2px 8px rgba(180,74,74,0.08);
                                    ">
                                        <div style="font-size:12px;color:#B44A4A;font-style:italic;margin-bottom:4px;">
                                            {capitalize(&m.from)}
                                            <span style="font-size:10px;color:#C07070;margin-left:8px;">
                                                {m.timestamp.clone()}
                                            </span>
                                        </div>
                                        <div style="font-size:13px;color:#5A3A32;">
                                            if m.message.ends_with(".gif") {
                                                <img style="max-width:200px;border-radius:8px;" src={m.message.clone()}/>
                                            } else {
                                                {m.message.clone()}
                                            }
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                <div style="
                    height:64px;
                    display:flex;
                    align-items:center;
                    padding:0 16px;
                    background:#F9EAE1;
                    gap:10px;
                ">
                    <input
                        ref={self.chat_input.clone()}
                        type="text"
                        placeholder="Write something... "
                        name="message"
                        required=true
                        style="
                            flex:1;
                            padding:10px 18px;
                            border-radius:999px;
                            border:1px solid #E3B6B6;
                            background:#fffaf7;
                            font-family:'Playfair Display',serif;
                            font-style:italic;
                            font-size:14px;
                            color:#5A3A32;
                            outline:none;
                        "
                    />
                    <button
                        onclick={submit}
                        style="
                            width:42px;
                            height:42px;
                            border-radius:50%;
                            border:none;
                            background:#B44A4A;
                            display:flex;
                            align-items:center;
                            justify-content:center;
                            cursor:pointer;
                            box-shadow:0 2px 8px rgba(180,74,74,0.3);
                        "
                    >
                        <svg fill="#000000" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" style="fill:white;width:18px;height:18px;">
                            <path d="M0 0h24v24H0z" fill="none"></path>
                            <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"></path>
                        </svg>
                    </button>
                </div>

            </div>
        </div>
    }
}
}
