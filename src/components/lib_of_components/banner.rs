use yew::prelude::*;
use yew::{html, Html};

struct BannerComponent {
    bg_color: AttrValue,
    height: AttrValue,
}

#[function_component]
pub(crate) fn Banner() -> Html {
    let banner_component = {
        BannerComponent {
            height: AttrValue::from("10px"),
            bg_color: AttrValue::from("orange")
        }
    };

    html! {
        <div
            style={format!("height: {}; background-color: {};", banner_component.height, banner_component.bg_color)}
            class="banner-component">
        // todo - either gradient or image
        {"hi hi"}
        </div>
    }
}
