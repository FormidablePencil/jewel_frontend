use yew::prelude::*;
use yew::{html, Html};
use crate::components::lib_of_components::carousel::Carousel;
use crate::components::lib_of_components::banner::Banner;
use crate::components::lib_of_components::text_component::TextComponent;
use crate::components::lib_of_components::profile_image::ProfileImage;

#[function_component]
pub(crate) fn Profile() -> Html {

    html! {
        <div>
        {"hi hi"}
        // todo - onclick feature
            <ProfileImage />
            <Carousel />
            <Banner />
            <TextComponent />
        </div>
    }
}
