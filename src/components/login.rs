use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div style="
            min-height:100vh;
            width:100vw;
            background:#FDF6F0;
            display:flex;
            align-items:center;
            justify-content:center;
            font-family:'Playfair Display',serif;
        ">
            <div style="
                background:rgba(255,255,255,0.7);
                border:1px solid #E3B6B6;
                border-radius:24px;
                padding:48px 40px;
                width:100%;
                max-width:400px;
                box-shadow:0 10px 30px rgba(180,74,74,0.1);
                display:flex;
                flex-direction:column;
                align-items:center;
                gap:8px;
            ">
                <div style="
                    font-family:'Parisienne',cursive;
                    font-size:42px;
                    color:#B44A4A;
                    margin-bottom:4px;
                ">
                    {"Chatify"}
                </div>

                <div style="
                    font-size:13px;
                    font-style:italic;
                    color:#C07070;
                    margin-bottom:24px;
                ">
                    {"⊹ ࣪ ˖ Enter your name to begin ⊹ ࣪ ˖"}
                </div>

                <form style="width:100%;display:flex;flex-direction:column;gap:14px;">
                    <input
                        {oninput}
                        placeholder="your name ..."
                        style="
                            width:100%;
                            padding:12px 18px;
                            border-radius:999px;
                            border:1px solid #E3B6B6;
                            background:#fffaf7;
                            font-family:'Playfair Display',serif;
                            font-style:italic;
                            font-size:14px;
                            color:#5A3A32;
                            outline:none;
                            box-sizing:border-box;
                        "
                    />
                    <Link<Route> to={Route::Chat}>
                        <button
                            {onclick}
                            disabled={username.len()<1}
                            style="
                                width:100%;
                                padding:12px;
                                border-radius:999px;
                                border:none;
                                background:#B44A4A;
                                color:white;
                                font-family:'Playfair Display',serif;
                                font-size:15px;
                                font-style:italic;
                                cursor:pointer;
                                box-shadow:0 4px 12px rgba(180,74,74,0.3);
                                transition:background 0.2s;
                            "
                        >
                            {"Enter room chat"}
                        </button>
                    </Link<Route>>
                </form>

                <div style="
                    margin-top:20px;
                    font-size:12px;
                    font-style:italic;
                    color:#C07070;
                    opacity:0.8;
                ">
                    {"Tutorial 3 Modul 10 - Nadia Aisyah Fazila"}
                </div>
            </div>
        </div>
    }
}