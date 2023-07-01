use yew::prelude::*;
use yew::{html, Html};

#[function_component]
pub(crate) fn Login() -> Html {
    html! {
        <div class="login">
            <img class="background-image" src="img/paint.jpg" />
            <div class="container">
                <p>{"Jewel"}</p>
                <div class="inputs">
                    <input placeholder="Username" type="text" />
                    <input placeholder="Password" type="text" />
                </div>
                <div class="action">
                    <button class="button">{"Login"}</button>
                    <button class="button">{"Signup"}</button>
                </div>
            </div>
        </div>
    }
}
