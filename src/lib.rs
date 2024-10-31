use std::rc::Rc;

use bounce::{use_slice, use_slice_value, Slice};

use js_sys::{Array, Object, Reflect};
use stylist::css;
use stylist::yew::Global;
use wasm_bindgen::JsValue;
use web_sys::{DomRect, Element, FillMode, KeyframeAnimationOptions};
use yew::prelude::*;
use yew_hooks::{use_raf, use_timeout};

/// https://gist.github.com/aminnj/5ca372aa2def72fb017b531c894afdca
fn calculate_text_width(text: &str, font_size: f64) -> f64 {
    let char_height = 13.328125; // M is close to a square

    text.chars().fold(0.0, |acc, c| {
        let char_width = match c {
            ' ' => 4.4453125,
            '!' => 4.4453125,
            '"' => 5.6796875,
            '#' => 8.8984375,
            '$' => 8.8984375,
            '%' => 14.2265625,
            '&' => 10.671875,
            '\'' => 3.0546875,
            '(' => 5.328125,
            ')' => 5.328125,
            '*' => 6.2265625,
            '+' => 9.34375,
            ',' => 4.4453125,
            '-' => 5.328125,
            '.' => 4.4453125,
            '/' => 4.4453125,
            '0' => 8.8984375,
            '1' => 7.7228125,
            '2' => 8.8984375,
            '3' => 8.8984375,
            '4' => 8.8984375,
            '5' => 8.8984375,
            '6' => 8.8984375,
            '7' => 8.8984375,
            '8' => 8.8984375,
            '9' => 8.8984375,
            ':' => 4.4453125,
            ';' => 4.4453125,
            '<' => 9.34375,
            '=' => 9.34375,
            '>' => 9.34375,
            '?' => 8.8984375,
            '@' => 16.2421875,
            'A' => 10.671875,
            'B' => 10.671875,
            'C' => 11.5546875,
            'D' => 11.5546875,
            'E' => 10.671875,
            'F' => 9.7734375,
            'G' => 12.4453125,
            'H' => 11.5546875,
            'I' => 4.4453125,
            'J' => 8.0,
            'K' => 10.671875,
            'L' => 8.8984375,
            'M' => 13.328125,
            'N' => 11.5546875,
            'O' => 12.4453125,
            'P' => 10.671875,
            'Q' => 12.4453125,
            'R' => 11.5546875,
            'S' => 10.671875,
            'T' => 9.7734375,
            'U' => 11.5546875,
            'V' => 10.671875,
            'W' => 15.1015625,
            'X' => 10.671875,
            'Y' => 10.671875,
            'Z' => 9.7734375,
            '[' => 4.4453125,
            '\\' => 4.4453125,
            ']' => 4.4453125,
            '^' => 7.5078125,
            '_' => 8.8984375,
            '`' => 5.328125,
            'a' => 8.8984375,
            'b' => 8.8984375,
            'c' => 8.0,
            'd' => 8.8984375,
            'e' => 8.8984375,
            'f' => 4.15921875,
            'g' => 8.8984375,
            'h' => 8.8984375,
            'i' => 3.5546875,
            'j' => 3.5546875,
            'k' => 8.0,
            'l' => 3.5546875,
            'm' => 13.328125,
            'n' => 8.8984375,
            'o' => 8.8984375,
            'p' => 8.8984375,
            'q' => 8.8984375,
            'r' => 5.328125,
            's' => 8.0,
            't' => 4.4453125,
            'u' => 8.8984375,
            'v' => 8.0,
            'w' => 11.5546875,
            'x' => 8.0,
            'y' => 8.0,
            'z' => 8.0,
            '{' => 5.34375,
            '|' => 4.15625,
            '}' => 5.34375,
            '~' => 9.34375,
            _ => 8.0, // who knows
        };
        acc + char_width * font_size / char_height
    })
}

#[derive(PartialEq, Default, Slice, Clone)]
pub struct QuestionMarkState {
    node_ref: Option<NodeRef>,
    display_tooltips: Option<bool>,
}

pub enum QuestionMarkAction {
    Set(NodeRef),
    SetDisplayTooltips(bool),
}

impl Reducible for QuestionMarkState {
    type Action = QuestionMarkAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let question_mark = Rc::make_mut(&mut self);
        match action {
            QuestionMarkAction::Set(node_ref) => {
                question_mark.node_ref = Some(node_ref);
            }
            QuestionMarkAction::SetDisplayTooltips(display_tooltips) => {
                question_mark.display_tooltips = Some(display_tooltips);
            }
        }
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct QuestionMarkProps {
    #[prop_or_default]
    pub classes: Classes,
    #[prop_or_default]
    pub animation: Option<(AnimationParams, Callback<()>)>,
    pub height: AttrValue,
}

#[function_component]
pub fn QuestionMark(props: &QuestionMarkProps) -> Html {
    let anim_enabled = props.animation.is_some();
    let waiting_for_initial_anim = use_state(move || anim_enabled);
    let cursor = if *waiting_for_initial_anim {
        "wait"
    } else {
        "pointer"
    };

    let _timeout = use_timeout(
        {
            let playing_initial_animation = waiting_for_initial_anim.setter();
            let animation_callback = props
                .animation
                .as_ref()
                .map_or_else(|| Callback::noop(), |(_, cb)| cb.clone());
            move || {
                playing_initial_animation.set(false);
                animation_callback.emit(());
            }
        },
        props
            .animation
            .as_ref()
            .map_or(100, |ap| ap.0.start_time + ap.0.duration),
    );

    let prev_anim_was_reverse = use_state(|| false);

    let node_ref = use_node_ref();
    let inner_ref = use_node_ref();
    let background_ref = use_node_ref();
    let border_ref = use_node_ref();
    let q_s = use_slice::<QuestionMarkState>();
    q_s.dispatch(QuestionMarkAction::Set(node_ref.clone()));

    let onclick = use_callback((prev_anim_was_reverse, *waiting_for_initial_anim), {
        let inner_ref = inner_ref.clone();
        let background_ref = background_ref.clone();
        let border_ref = border_ref.clone();

        let q_s = q_s.clone();
        move |_, (r, waiting)| {
            if !waiting {
                let play = |n: &NodeRef, prop: &'static str, value: &'static str| {
                    let keyframes = Array::new();
                    let keyframe = Object::new();
                    let _ = Reflect::set(&keyframe, &prop.into(), &value.into());
                    keyframes.push(&keyframe);

                    let n = n.cast::<Element>().unwrap();
                    let options = KeyframeAnimationOptions::new();
                    options.set_duration(&JsValue::from(500));
                    options.set_fill(FillMode::Forwards);
                    let _ = n.animate_with_keyframe_animation_options(Some(&keyframes), &options);
                };

                if !**r {
                    play(&inner_ref, "color", "#ffffff");
                    play(&background_ref, "color", "#676a6f");
                    play(&border_ref, "color", "transparent");
                    q_s.dispatch(QuestionMarkAction::SetDisplayTooltips(true));

                    r.set(true);
                } else {
                    play(&inner_ref, "color", "#000000");
                    play(&background_ref, "color", "#ffffff");
                    play(&border_ref, "color", "rgb(209,213,219)");
                    q_s.dispatch(QuestionMarkAction::SetDisplayTooltips(false));
                    r.set(false);
                }
            }
        }
    });

    let ( circle_animation , path_animation, outline_animation) = match &props.animation {
        Some(_) =>(  "animation-name: question-mark-inside-invert; animation-duration: 0.8s; animation-fill-mode: forwards; animation-delay: 3s;" , "animation-name: question-mark-background-invert; animation-duration: 0.8s; animation-fill-mode: forwards; animation-delay: 3s;", "animation-name: question-mark-outer-stroke-appear; animation-duration: 0.8s; animation-fill-mode: forwards; animation-delay: 3s;"),
        None => Default::default(),
    };
    html! {
        <>
            <Global
                css={css!{
                "@keyframes question-mark-inside-invert { to { color: #000000; } } @keyframes question-mark-background-invert { to { color: #ffffff; } }  @keyframes question-mark-outer-stroke-appear { to { color: rgb(209,213,219); } }"
            }}
            />
            <svg
                viewBox="0 0 512 512"
                class={props.classes.clone()}
                ref={node_ref}
                // todo: I don't think this is very flexible
                style={format!("height: {}; position: absolute; left: 50%; bottom: -4%; transform: translate(-50%, 100%);", props.height)}
            >
                <circle
                    ref={inner_ref}
                    fill="currentColor"
                    cx="256"
                    cy="256"
                    r="250"
                    onclick={onclick.clone()}
                    style={circle_animation}
                    class={css!{
                color: ${ if props.animation.is_some() { "#ffffff" } else { "#000000" } };
                cursor: ${cursor};
            }}
                />
                <path
                    ref={background_ref}
                    {onclick}
                    style={path_animation}
                    class={css!{
                color: ${ if props.animation.is_some() { "#676a6f" } else { "#ffffff" } };
                cursor: ${cursor};
            }}
                    fill="currentColor"
                    d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM169.8 165.3c7.9-22.3 29.1-37.3 52.8-37.3h58.3c34.9 0 63.1 28.3 63.1 63.1c0 22.6-12.1 43.5-31.7 54.8L280 264.4c-.2 13-10.9 23.6-24 23.6c-13.3 0-24-10.7-24-24V250.5c0-8.6 4.6-16.5 12.1-20.8l44.3-25.4c4.7-2.7 7.6-7.7 7.6-13.1c0-8.4-6.8-15.1-15.1-15.1H222.6c-3.4 0-6.4 2.1-7.5 5.3l-.4 1.2c-4.4 12.5-18.2 19-30.6 14.6s-19-18.2-14.6-30.6l.4-1.2zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z"
                />
                <circle
                    style={outline_animation}
                    ref={border_ref}
                    fill="none"
                    stroke="currentColor"
                    stroke-width="20"
                    cx="256"
                    cy="256"
                    r="246"
                    class={css!{
                    color: ${ if props.animation.is_some() { "transparent" } else { "rgb(209,213,219)" } };

                    }}
                />
            </svg>
        </>
    }
}

#[derive(PartialEq)]
pub struct AnimationParams {
    pub duration: u32,
    pub start_time: u32,
}

impl Default for AnimationParams {
    fn default() -> Self {
        Self {
            duration: 500,
            start_time: 3000,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub show: bool,
    #[prop_or_default]
    pub animation_params: Option<AnimationParams>,
    #[prop_or_default]
    pub static_bottom: Option<AttrValue>,
    #[prop_or_default]
    pub static_top: Option<AttrValue>,
    #[prop_or_default]
    pub static_left: Option<AttrValue>,
    #[prop_or_default]
    pub static_right: Option<AttrValue>,

    #[prop_or_default]
    pub classes: Classes,
    #[prop_or(0.12)]
    pub border_radius_ratio: f64,
    #[prop_or(1.0/6.0)]
    pub tip_height_ratio: f64,
    #[prop_or(4.0/3.0)]
    pub tip_aspect_ratio: f64,

    #[prop_or("#676a6f".into())]
    pub background_color: AttrValue,
    pub height: AttrValue,
    #[prop_or(0.5)]
    pub inline_padding: f64,
    #[prop_or(0.1)]
    pub control_point_0_ratio: f64,
    #[prop_or(0.1)]
    pub control_point_1_ratio: f64,
    #[prop_or(0.31)]
    pub font_height_ratio: f64,

    #[prop_or("#ffffff".into())]
    pub text_color: AttrValue,
    pub text: AttrValue,
    #[prop_or(false)]
    pub mirror: bool,
}

struct StaticReference {
    question_mark: DomRect,
    tooltip: DomRect,
}

#[function_component]
pub fn Tooltip(props: &Props) -> Html {
    let node_ref = use_node_ref();

    let elapsed = {
        let (duration, start_time) = props
            .animation_params
            .as_ref()
            .map_or((5, 5), |ap| (ap.duration, ap.start_time));
        use_raf(duration, start_time)
    };

    let elapsed = if !props.animation_params.is_some() {
        1.0
    } else {
        elapsed
    };

    let question_mark = use_slice_value::<QuestionMarkState>();

    let before_hiding_params = use_mut_ref(|| None);
    let context = use_mut_ref(|| None::<Element>);

    use_effect({
        let before_hiding_params = before_hiding_params.clone();
        let node_ref = node_ref.clone();
        let q = question_mark.clone();
        let context = context.clone();
        move || {
            if elapsed == 0.0 {
                // we go up the tree until we hit a <dialog> element or the body
                let mut parent = node_ref.cast::<Element>().unwrap().parent_element();
                while parent.is_some() {
                    let p = parent.unwrap();
                    if p.tag_name() == "DIALOG" {
                        *(context.borrow_mut()) = Some(p);
                        break;
                    }
                    parent = p.parent_element();
                }

                if let Some(q) = &q.node_ref {
                    let svg = node_ref.cast::<Element>().unwrap();
                    let tooltip_rect = svg.get_bounding_client_rect();
                    let q = q.cast::<Element>().unwrap();
                    let question_mark_rect = q.get_bounding_client_rect();
                    *(before_hiding_params.borrow_mut()) = Some(StaticReference {
                        question_mark: question_mark_rect,
                        tooltip: tooltip_rect,
                    });
                }
            }
        }
    });

    let v_height = 100f64;
    let font_size = props.font_height_ratio * v_height;

    let padding_inline_sum = v_height * (1.0 - props.font_height_ratio) * props.inline_padding;

    let stalk_width = padding_inline_sum + calculate_text_width(&props.text, font_size);

    let tip_height = v_height * props.tip_height_ratio;
    let tip_width = tip_height * props.tip_aspect_ratio;

    let v_width = stalk_width + tip_width;

    let view_box = format!("0 0 {} {}", v_width, v_height);
    let translate_x = if !props.mirror { 1.0 } else { -1.0 };
    let height = props.height.clone();
    let translate_y = -0.5;

    let touch_point_x = tip_width;
    let upper_touch_point_y = (1.0 - props.tip_height_ratio) * v_height / 2.0;
    let lower_touch_point_y = (0.5 + props.tip_height_ratio / 2.0) * v_height;

    let control_point_0_x = tip_width * props.control_point_0_ratio;
    let tip_height = v_height * props.tip_height_ratio;
    let control_point_1_size = tip_height * props.control_point_1_ratio;

    let d = format!(
        "M 0 {} C {} {}, {} {}, {} {} h 2 v {} h -2 C {} {}, {} {}, {} {}",
        v_height / 2.0,
        control_point_0_x,
        v_height / 2.0,
        touch_point_x,
        upper_touch_point_y + control_point_1_size,
        touch_point_x,
        upper_touch_point_y,
        tip_height,
        touch_point_x,
        lower_touch_point_y - control_point_1_size,
        control_point_0_x,
        v_height / 2.0,
        0,
        v_height / 2.0,
    );

    let mut svg_style = if elapsed <= 0.0 || elapsed == 1.0 {
        let x_coeff = match question_mark.display_tooltips {
            Some(true) => 1.0,
            // Some(false) => 0.9,
            Some(false) => {
                if props.show {
                    1.0
                } else {
                    0.9
                }
            }
            None => {
                if elapsed == 0.0 {
                    1.0
                } else {
                    if props.show {
                        1.0
                    } else {
                        0.9
                    }
                }
            }
        };

        let mut s = format!(
            "transform: translate({}%, {}%); position: absolute; ",
            translate_x * x_coeff * 100.0,
            translate_y * 100.0,
        );
        if let Some(bottom) = &props.static_bottom {
            s.push_str(&format!("bottom: {}; ", bottom));
        }
        if let Some(top) = &props.static_top {
            s.push_str(&format!("top: {}; ", top));
        }
        if let Some(left) = &props.static_left {
            s.push_str(&format!("left: {}; ", left));
        }
        if let Some(right) = &props.static_right {
            s.push_str(&format!("right: {}; ", right));
        }
        s
    } else {
        let before_hiding_params = before_hiding_params.borrow();
        let reference = before_hiding_params.as_ref().unwrap();
        // the user might have scrolled and/or zoomed.
        // we figure out the affine transformation
        let q_now = question_mark.node_ref.as_ref().unwrap().cast::<Element>();
        let q_rect_now = q_now.unwrap().get_bounding_client_rect();
        // say new_coord = old_coord * scale + translate
        let scale = q_rect_now.width() / reference.question_mark.width();
        let translate_x = q_rect_now.x() - reference.question_mark.x() * scale;
        let translate_y = q_rect_now.y() - reference.question_mark.y() * scale;
        let transform = |x: f64, y: f64| (x * scale + translate_x, y * scale + translate_y);

        let reference_h_drift = (reference.question_mark.x()
            + reference.question_mark.width() / 2.0)
            - (reference.tooltip.x() + reference.tooltip.width() / 2.0);
        let reference_v_drift = (reference.question_mark.bottom()
            - reference.question_mark.height() / 2.0)
            - (reference.tooltip.bottom() - reference.tooltip.height() / 2.0);

        let inflection =
            reference_v_drift.abs() / (reference_v_drift.abs() + reference_h_drift.abs());

        let (left, top) = if elapsed <= inflection {
            let t = elapsed / inflection;
            let top = reference.tooltip.top() + reference_v_drift * t;
            transform(reference.tooltip.x(), top)
        } else {
            let t = (elapsed - inflection) / (1.0 - inflection);
            let left = reference.tooltip.x() + reference_h_drift * t;
            transform(left, reference.tooltip.top() + reference_v_drift)
        };

        format!("position:fixed; top: {top}px; left: {left}px;")
    };
    svg_style.push_str("height: ");
    svg_style.push_str(&height);
    svg_style.push_str("; ");
    if elapsed == 0.0 || question_mark.display_tooltips.is_some() || props.show {
        svg_style.push_str("animation: tooltip-fade-in 0.2s ease-out, ");
        if !props.mirror {
            svg_style.push_str("tooltip-slide-in-left 0.2s ease-out;");
        } else {
            svg_style.push_str("tooltip-slide-in-right 0.2s ease-out;");
        }
    }

    let opacity = match question_mark.display_tooltips {
        Some(true) => 1.0,
        // Some(false) => 0.0,
        Some(false) => {
            if props.show {
                1.0
            } else {
                0.0
            }
        }
        None => {
            if elapsed < 1.0 {
                1.0 - elapsed
            } else {
                if props.show {
                    1.0
                } else {
                    0.0
                }
            }
        }
    };
    svg_style.push_str(&format!("opacity: {}; ", opacity));
    svg_style.push_str("pointer-events: none; z-index: 1000;");

    // if question_mark.display_tooltips.is_some() || props.show {
    if elapsed == 1.0 {
        svg_style.push_str("transition: opacity 0.2s ease-out, transform 0.2s ease-out;");
    }

    let ret = html! {
        <>
            <Global
                css={css!{
    "@keyframes tooltip-fade-in { 0% { opacity: 0; } } @keyframes tooltip-slide-in-left { 0% { transform: translate(90%, -50%); } } @keyframes tooltip-slide-in-right { 0% { transform: translate(-90%, -50%); } }"
            }}
            />
            <svg viewBox={view_box} class={props.classes.clone()} ref={node_ref} style={svg_style}// opacity={opacity.to_string()}
            >
                <rect
                    x={if !props.mirror {tip_width.to_string()} else {0.0.to_string()}}
                    width={stalk_width.to_string()}
                    height={v_height.to_string()}
                    rx={( props.border_radius_ratio * v_height ).to_string()}
                    fill={props.background_color.clone()}
                />
                <text
                    x={if !props.mirror {(tip_width + stalk_width / 2.0).to_string()}else{(stalk_width / 2.0).to_string()}}
                    y={(v_height / 2.0).to_string()}
                    text-anchor="middle"
                    dominant-baseline="central"
                    fill={props.text_color.clone()}
                    font-size={font_size.to_string()}
                    // sans-serif
                    font-family="Arial"
                >
                    { props.text.clone() }
                </text>
                <path
                    {d}
                    fill={props.background_color.clone()}
                    transform={props.mirror.then_some( format!("scale(-1, 1) translate({}, 0)", -1.0 * tip_width - stalk_width) )}
                />
            </svg>
        </>
    };
    if elapsed <= 0.0 || elapsed == 1.0 {
        ret
    } else {
        // html bug moment dawg!
        // when any predecessor of Tooltip is removed has a transform set, position: fixed is gonna
        // be relative to the fucking dumb predecessor. So we have to render out of place here.

        let host = match (context.borrow()).as_ref() {
            Some(dialog) => (*dialog).clone(),
            None => {
                let document = web_sys::window().unwrap().document().unwrap();
                let host = document.get_elements_by_tag_name("body").item(0).unwrap();
                host
            }
        };

        create_portal(ret, host)
    }
}
