use yew::prelude::*;
use yew::{html, Html};

pub struct Profile {
    id: u64,
    is_bg_img_full_width: bool,
    bg_img_height: AttrValue,
    bg_img_object_fit: AttrValue,
    img_height: AttrValue,
    img_width: AttrValue,
    top: AttrValue,
}

#[function_component]
pub(crate) fn ProfileImage() -> Html {
    let profile = Profile {
        id: 10,
        is_bg_img_full_width: false,
        bg_img_height: AttrValue::from("300px"),
        bg_img_object_fit: AttrValue::from("cover"),
        img_height: AttrValue::from("80px"),
        img_width: AttrValue::from("80px"),
        top: AttrValue::from("80px"),
    };

    html! {
        <div class="image-container">
            <div
                style={format!("height: {};", profile.bg_img_height)}
                class="bg-img-container">
                <img
                    style={format!(
                    "height: {};\
                    object-fit: {};",
                    profile.bg_img_height,
                    profile.bg_img_object_fit
                    )}
                    class="background-image" src="img/paint.jpg" />
            </div>
            <div class="profile-image-container">
                <img
                    style={format!(
                    "height: {};\
                     width: {};\
                     top: {};",
                    profile.img_height,
                    profile.img_width,
                    profile.top,
                    )}
                    class="profile-image" src="img/paint.jpg" />
                {"hi hi text component"}
            </div>
        </div>
    }
}
