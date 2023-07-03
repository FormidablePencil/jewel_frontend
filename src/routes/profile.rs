use yew::prelude::*;
use yew::{html, Html};
use crate::components::lib_of_components::carousel::Carousel;
use crate::components::lib_of_components::banner::Banner;
use crate::components::lib_of_components::text_component::TextComponent;
use crate::components::lib_of_components::profile_image::ProfileImage;

pub struct ProfileStyles {
    background_color: AttrValue,
    background_image: AttrValue,
    container_color: AttrValue,
    container_image: AttrValue,
}

#[function_component]
pub(crate) fn Profile() -> Html {
    let profile_styles = ProfileStyles {
        background_color: AttrValue::from("orange"),
        background_image: AttrValue::from("qwerty"),
        container_color: AttrValue::from("red"),
        container_image: AttrValue::from("qwerty"),
    };

    html! {
        <div
            style={format!(
            "background-color: {};\
            background-image: {};",
            profile_styles.background_color,
            profile_styles.background_image,
            )}
            class="profile">
            <div class="profile-container">
            {"hi hi"}
            // todo - onclick feature
                <ProfileImage />
                <Carousel />
                <Banner />
                <TextComponent />
            </div>
        </div>
    }
}
