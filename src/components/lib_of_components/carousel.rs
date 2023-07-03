use yew::prelude::*;
use yew::{html, Html};

pub struct CarouselContent {
    name: AttrValue,
    img_url: AttrValue,
    img_alt: AttrValue,
}

pub struct CarouselComponent {
    bg_color: AttrValue,
}

pub struct CarouselComponentProperties {
    margin: AttrValue,
    offset_from_top: AttrValue,
    width: AttrValue,
    height: AttrValue,
    roundness: AttrValue,
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

    let carousel_component_styles = CarouselComponentProperties {
        margin: AttrValue::from("10px"),
        offset_from_top: AttrValue::from("10px"),
        roundness: AttrValue::from("10px"),
        height: AttrValue::from("5em"),
        width: AttrValue::from("3em"),
    };

    let carousel_components = vec![
        CarouselComponent {
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
        CarouselComponent {
            bg_color: AttrValue::from("rgb(240,255,255)"),
        },
    ];

    fn s() {

    }

    html! {
        <div class="carousel-component">
        // todo - either gradient or image
        {
            carousel_content.iter().enumerate().map(|(index, item)| {
            // todo - will crash if carousel_content isn't the same in size as carousel_component
                html! {
                    <div
                        class=""
                        style={format!(
                            "margin: {};\
                             width: {};\
                             height: {};\
                             border-radius: {};\
                             background-color: {};",
                            carousel_component_styles.margin /*- carousel_component[index].offset_from_top*/,
                            carousel_component_styles.width,
                            carousel_component_styles.height,
                            carousel_component_styles.roundness,
                            carousel_components[index].bg_color,
                        )}
                        key={item.name.to_string()}>
                        // {carousel_component[index].roundness.to_string()}
                        // {item.img_url.clone()}
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}
