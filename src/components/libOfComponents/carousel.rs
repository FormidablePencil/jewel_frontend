use yew::prelude::*;
use yew::{html, Html};

pub struct CarouselContent {
    name: AttrValue,
    img_url: AttrValue,
    img_alt: AttrValue,
}

pub struct CarouselComponent {
    width: AttrValue,
    height: AttrValue,
    roundness: AttrValue,
    bg_color: AttrValue,
}

#[function_component]
pub(crate) fn Carousel() -> Html {
    let carousel_content = vec![
        CarouselContent {
            name: AttrValue::from("Flowers"),
            img_url: AttrValue::from("https://i.ibb.co/jfffKwQ/flowers.jpg"),
            img_alt: AttrValue::from("Flowers")
        },
        CarouselContent {
            name: AttrValue::from("Flowers"),
            img_url: AttrValue::from("https://i.ibb.co/jfffKwQ/flowers.jpg"),
            img_alt: AttrValue::from("Flowers")
        },
        CarouselContent {
            name: AttrValue::from("Flowers"),
            img_url: AttrValue::from("https://i.ibb.co/jfffKwQ/flowers.jpg"),
            img_alt: AttrValue::from("Flowers")
        },
        CarouselContent {
            name: AttrValue::from("Flowers"),
            img_url: AttrValue::from("https://i.ibb.co/jfffKwQ/flowers.jpg"),
            img_alt: AttrValue::from("Flowers")
        },
    ];

    let carousel_component = vec![
        CarouselComponent {
            roundness: AttrValue::from("10px"),
            height: AttrValue::from("5em"),
            width: AttrValue::from("3em"),
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            roundness: AttrValue::from("10px"),
            height: AttrValue::from("5em"),
            width: AttrValue::from("3em"),
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            roundness: AttrValue::from("10px"),
            height: AttrValue::from("5em"),
            width: AttrValue::from("3em"),
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            roundness: AttrValue::from("10px"),
            height: AttrValue::from("5em"),
            width: AttrValue::from("3em"),
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
    ];

    html! {
        <div class="carousel-component">
        {"hello"}
        // todo - either gradient or image
        {
            carousel_content.iter().enumerate().map(|(index, item)| {
            // todo - will crash if carousel_content isn't the same in size as carousel_component
                html! {
                    <div
                        class="content"
                        style={format!(
                            "width: {};\
                             height: {};\
                             border-radius: {};\
                             background-color: {}",
                            carousel_component[index].width,
                            carousel_component[index].height,
                            carousel_component[index].roundness,
                            carousel_component[index].bg_color,
                        )}
                        key={item.name.to_string()}>
                        {carousel_component[index].roundness.to_string()}
                        {item.img_url.clone()}
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}
