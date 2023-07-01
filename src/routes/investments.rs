use yew::prelude::*;
use yew::{html, Html};

#[function_component]
pub fn Investments() -> Html {
    let names = vec!["Sam","Bob","Ray"];

    html! {
        <div class="investments">
            <p>{"My investments"}</p>
        // list of all user's investments
        {
            names.into_iter().map(|name| {
                html!{<div key={name}>{ format!("Hello, I'am {}!",name) }</div>}
            }).collect::<Html>()
        }
        </div>
    }
}
