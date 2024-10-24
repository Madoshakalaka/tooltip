use bounce::BounceRoot;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use tooltip::{QuestionMark, Tooltip};

#[cfg(feature = "demo")]
use stylist::yew::styled_component_impl;

#[derive(Properties, PartialEq)]
pub struct MockButtonProps {
    pub flavor: Flavor,
}

#[derive(PartialEq)]
pub enum Flavor {
    Plus,
    Minus,
    RightArrow,
    ResetView,
    Permissions,
}

impl Flavor {
    fn view_box_width(&self) -> f64 {
        match self {
            Flavor::Plus => 448.0,
            Flavor::Minus => 448.0,
            Flavor::RightArrow => 448.0,
            Flavor::ResetView => 512.0,
            Flavor::Permissions => 640.0,
        }
    }
    fn d(&self) -> &'static str {
        match self {
            Flavor::Plus => "M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z",
            Flavor::Minus => "M432 256c0 17.7-14.3 32-32 32L48 288c-17.7 0-32-14.3-32-32s14.3-32 32-32l352 0c17.7 0 32 14.3 32 32z",
            Flavor::RightArrow => "M438.6 278.6c12.5-12.5 12.5-32.8 0-45.3l-160-160c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L338.8 224 32 224c-17.7 0-32 14.3-32 32s14.3 32 32 32l306.7 0L233.4 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0l160-160z",
            Flavor::ResetView => "M386.3 160H336c-17.7 0-32 14.3-32 32s14.3 32 32 32H464c17.7 0 32-14.3 32-32V64c0-17.7-14.3-32-32-32s-32 14.3-32 32v51.2L414.4 97.6c-87.5-87.5-229.3-87.5-316.8 0s-87.5 229.3 0 316.8s229.3 87.5 316.8 0c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0c-62.5 62.5-163.8 62.5-226.3 0s-62.5-163.8 0-226.3s163.8-62.5 226.3 0L386.3 160z",
            Flavor::Permissions => "M144 160A80 80 0 1 0 144 0a80 80 0 1 0 0 160zm368 0A80 80 0 1 0 512 0a80 80 0 1 0 0 160zM0 298.7C0 310.4 9.6 320 21.3 320H234.7c.2 0 .4 0 .7 0c-26.6-23.5-43.3-57.8-43.3-96c0-7.6 .7-15 1.9-22.3c-13.6-6.3-28.7-9.7-44.6-9.7H106.7C47.8 192 0 239.8 0 298.7zM320 320c24 0 45.9-8.8 62.7-23.3c2.5-3.7 5.2-7.3 8-10.7c2.7-3.3 5.7-6.1 9-8.3C410 262.3 416 243.9 416 224c0-53-43-96-96-96s-96 43-96 96s43 96 96 96zm65.4 60.2c-10.3-5.9-18.1-16.2-20.8-28.2H261.3C187.7 352 128 411.7 128 485.3c0 14.7 11.9 26.7 26.7 26.7H455.2c-2.1-5.2-3.2-10.9-3.2-16.4v-3c-1.3-.7-2.7-1.5-4-2.3l-2.6 1.5c-16.8 9.7-40.5 8-54.7-9.7c-4.5-5.6-8.6-11.5-12.4-17.6l-.1-.2-.1-.2-2.4-4.1-.1-.2-.1-.2c-3.4-6.2-6.4-12.6-9-19.3c-8.2-21.2 2.2-42.6 19-52.3l2.7-1.5c0-.8 0-1.5 0-2.3s0-1.5 0-2.3l-2.7-1.5zM533.3 192H490.7c-15.9 0-31 3.5-44.6 9.7c1.3 7.2 1.9 14.7 1.9 22.3c0 17.4-3.5 33.9-9.7 49c2.5 .9 4.9 2 7.1 3.3l2.6 1.5c1.3-.8 2.6-1.6 4-2.3v-3c0-19.4 13.3-39.1 35.8-42.6c7.9-1.2 16-1.9 24.2-1.9s16.3 .6 24.2 1.9c22.5 3.5 35.8 23.2 35.8 42.6v3c1.3 .7 2.7 1.5 4 2.3l2.6-1.5c16.8-9.7 40.5-8 54.7 9.7c2.3 2.8 4.5 5.8 6.6 8.7c-2.1-57.1-49-102.7-106.6-102.7zm91.3 163.9c6.3-3.6 9.5-11.1 6.8-18c-2.1-5.5-4.6-10.8-7.4-15.9l-2.3-4c-3.1-5.1-6.5-9.9-10.2-14.5c-4.6-5.7-12.7-6.7-19-3l-2.9 1.7c-9.2 5.3-20.4 4-29.6-1.3s-16.1-14.5-16.1-25.1v-3.4c0-7.3-4.9-13.8-12.1-14.9c-6.5-1-13.1-1.5-19.9-1.5s-13.4 .5-19.9 1.5c-7.2 1.1-12.1 7.6-12.1 14.9v3.4c0 10.6-6.9 19.8-16.1 25.1s-20.4 6.6-29.6 1.3l-2.9-1.7c-6.3-3.6-14.4-2.6-19 3c-3.7 4.6-7.1 9.5-10.2 14.6l-2.3 3.9c-2.8 5.1-5.3 10.4-7.4 15.9c-2.6 6.8 .5 14.3 6.8 17.9l2.9 1.7c9.2 5.3 13.7 15.8 13.7 26.4s-4.5 21.1-13.7 26.4l-3 1.7c-6.3 3.6-9.5 11.1-6.8 17.9c2.1 5.5 4.6 10.7 7.4 15.8l2.4 4.1c3 5.1 6.4 9.9 10.1 14.5c4.6 5.7 12.7 6.7 19 3l2.9-1.7c9.2-5.3 20.4-4 29.6 1.3s16.1 14.5 16.1 25.1v3.4c0 7.3 4.9 13.8 12.1 14.9c6.5 1 13.1 1.5 19.9 1.5s13.4-.5 19.9-1.5c7.2-1.1 12.1-7.6 12.1-14.9v-3.4c0-10.6 6.9-19.8 16.1-25.1s20.4-6.6 29.6-1.3l2.9 1.7c6.3 3.6 14.4 2.6 19-3c3.7-4.6 7.1-9.4 10.1-14.5l2.4-4.2c2.8-5.1 5.3-10.3 7.4-15.8c2.6-6.8-.5-14.3-6.8-17.9l-3-1.7c-9.2-5.3-13.7-15.8-13.7-26.4s4.5-21.1 13.7-26.4l3-1.7zM472 384a40 40 0 1 1 80 0 40 40 0 1 1 -80 0z",
        }
    }
}

#[function_component]
pub fn MockButton(props: &MockButtonProps) -> Html {
    let aspect_ratio = props.flavor.view_box_width() / 512.0;
    html! {
        <div
            style={format!(r#"
                border-radius: 20%;
                border: 2px solid rgb(209,213,219);
                display: flex;
                justify-content: center;
                align-items: center;
                height: {};
                aspect-ratio: 1/1;
                "#, button_size())}
        >
            <svg
                style={format!("height: 50%; aspect-ratio: {aspect_ratio};")}
                viewBox={format!( "0 0 {} 512", props.flavor.view_box_width() )}
            >
                <path fill="black" d={props.flavor.d()} />
            </svg>
        </div>
    }
}

const EDITOR_SIZE: &str = "min(80vw, 80vh)";

fn button_size() -> String {
    format!("calc({} / 10)", EDITOR_SIZE)
}

#[cfg_attr(feature = "demo", styled_component_impl)]
#[function_component]
pub fn App() -> Html {
    html! {
        <BounceRoot>
            <div
                style=r#"
                width: 100%;
                height: 100vh;
                display: flex;
            justify-content: center;
            align-items: center;
            "#
            >
                <div
                    style={format!( "border-radius: 5%; border: 2px solid rgb(209,213,219); aspect-ratio: 1/1; width: {EDITOR_SIZE}; position: relative;" )}
                >
                    <div
                        style="position: absolute; left: 0; top: 50%; transform: translateY(-50%);"
                    >
                        <div style="position: relative;">
                            <QuestionMark
                                classes={css!{height: ${format!("calc({} * 0.7)", button_size())}; position: absolute; left: 50%; bottom: -4%; transform: translate(-50%, 100%);}}
                            />
                            <div style="position: relative;">
                                <Tooltip
                                    height={format!("calc({} * 0.8)", button_size())}
                                    text="Add"
                                    classes={css!{position: absolute; top: 50%; right: -15%;}}
                                />
                                <MockButton flavor={Flavor::Plus} />
                            </div>

                            <div style="position: relative;">
                                <Tooltip
                                    height={format!("calc({} * 0.8)", button_size())}
                                    text="Remove"
                                    classes={css!{position: absolute; top: 50%; right: -15%;}}
                                />
                                <MockButton flavor={Flavor::Minus} />
                            </div>

                            <div style="position: relative;">
                                <Tooltip
                                    height={format!("calc({} * 0.8)", button_size())}
                                    text="Connect"
                                    classes={css!{position: absolute; top: 50%; right: -15%;}}
                                />
                                <MockButton flavor={Flavor::RightArrow} />
                            </div>

                            <div style="position: relative;">
                                <Tooltip
                                    height={format!("calc({} * 0.8)", button_size())}
                                    text="Reset View"
                                    classes={css!{position: absolute; top: 50%; right: -15%;}}
                                />
                                <MockButton flavor={Flavor::ResetView} />
                            </div>

                        </div>
                    </div>
                    <div
                        style="position: absolute; right: 0; top: 50%; transform: translateY(-50%);"
                    >
                       
                            <div style="position: relative;">
                                <Tooltip
                                    height={format!("calc({} * 0.8)", button_size())}
                                    text="Permissions"
                                    classes={css!{position: absolute; top: 50%; left: -15%;}}
                                    mirror = {true}
                                />
                                <MockButton flavor={Flavor::Permissions} />
                            </div>

                    </div>
                </div>
            </div>
        </BounceRoot>
    }
}
