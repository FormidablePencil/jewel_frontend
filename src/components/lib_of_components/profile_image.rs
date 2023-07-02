use yew::prelude::*;
use yew::{html, Html};

pub struct Profile {
    id: u64,
    bg_img_height: AttrValue,
    img_height: AttrValue,
    img_width: AttrValue,
}

#[function_component]
pub(crate) fn ProfileImage() -> Html {
    let profile = Profile {
        id: 10,
        bg_img_height: AttrValue::from("300px"),
        img_height: AttrValue::from("80px"),
        img_width: AttrValue::from("80px"),
    };

    html! {
        <div class="profile-image-container">
            // <img
            //     style={format!("height: {};", profile.bg_img_height)}
            //     class="background-image" src="img/paint.jpg" />
            <img
                style={format!("height: {}; width: {};", profile.img_height, profile.img_width)}
                class="profile-image" src="img/paint.jpg" />
            {"hi hi text component"}
        </div>
    }
}
