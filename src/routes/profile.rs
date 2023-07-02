use yew::prelude::*;
use yew::{html, Html};
use crate::components::libOfComponents::carousel::Carousel;
use crate::components::libOfComponents::banner::Banner;
use crate::components::libOfComponents::text_component::TextComponent;

#[function_component]
pub(crate) fn Profile() -> Html {

    html! {
        <div>
        {"hi hi"}
        // todo - onclick feature
            <Carousel />
            <Banner />
            <TextComponent />
        </div>
    }
}
