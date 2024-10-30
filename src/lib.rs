use std::rc::Rc;

use bounce::{use_slice, use_slice_dispatch, use_slice_value, Slice};
use stylist::css;
use stylist::yew::Global;
use web_sys::{DomRect, Element, SvgAnimateMotionElement, SvgAnimationElement};
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
    fill_animation: Option<NodeRef>,
    background_animation: Option<NodeRef>,
}

pub enum QuestionMarkAction {
    Set(NodeRef, NodeRef, NodeRef),
    StartFilling,
}

impl Reducible for QuestionMarkState {
    type Action = QuestionMarkAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let question_mark = Rc::make_mut(&mut self);
        match action {
            QuestionMarkAction::Set(node_ref, f, b) => {
                question_mark.node_ref = Some(node_ref);
                question_mark.fill_animation = Some(f);
                question_mark.background_animation = Some(b);
            }
            QuestionMarkAction::StartFilling => {
                let _ = self
                    .fill_animation
                    .as_ref()
                    .unwrap()
                    .cast::<SvgAnimationElement>()
                    .unwrap()
                    .begin_element();
                let _ = self
                    .background_animation
                    .as_ref()
                    .unwrap()
                    .cast::<SvgAnimationElement>()
                    .unwrap()
                    .begin_element();
            }
        }
        self
    }
}

#[derive(PartialEq, Default, Slice, Clone)]
pub struct TooltipGroupState {
    tooltips: Vec<NodeRef>,
}

pub enum Action {
    AddTooltip(NodeRef),
    RemoveTooltip(NodeRef),
}

impl Reducible for TooltipGroupState {
    type Action = Action;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let tooltips = Rc::make_mut(&mut self);
        match action {
            Action::AddTooltip(tooltip) => {
                // only add if not already in the list
                if !tooltips.tooltips.contains(&tooltip) {
                    tooltips.tooltips.push(tooltip);
                }
            }
            Action::RemoveTooltip(tooltip) => {
                tooltips.tooltips.retain(|t| t != &tooltip);
            }
        }
        self
    }
}

#[derive(Properties, PartialEq)]
pub struct QuestionMarkProps {
    pub classes: Classes,
    #[prop_or(3500)]
    pub animation_start_time: u32,
}

#[function_component]
pub fn QuestionMark(props: &QuestionMarkProps) -> Html {
    let node_ref = use_node_ref();
    let fill_animation = use_node_ref();
    let background_animation = use_node_ref();
    let q_s = use_slice::<QuestionMarkState>();
    q_s.dispatch(QuestionMarkAction::Set(
        node_ref.clone(),
        fill_animation.clone(),
        background_animation.clone(),
    ));
    let begin = format!("{}ms", props.animation_start_time);

    html! {
        <svg viewBox="0 0 512 512" class={props.classes.clone()} ref={node_ref}>
            <circle fill="#ffffff" cx="256" cy="256" r="250">
                <animate
                    attributeName="fill"
                    from="#ffffff"
                    to="black"
                    dur="0.5s"
                    fill="freeze"
                    begin="3500ms"
                />
            </circle>
            <path
                fill="#676a6f"
                d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM169.8 165.3c7.9-22.3 29.1-37.3 52.8-37.3h58.3c34.9 0 63.1 28.3 63.1 63.1c0 22.6-12.1 43.5-31.7 54.8L280 264.4c-.2 13-10.9 23.6-24 23.6c-13.3 0-24-10.7-24-24V250.5c0-8.6 4.6-16.5 12.1-20.8l44.3-25.4c4.7-2.7 7.6-7.7 7.6-13.1c0-8.4-6.8-15.1-15.1-15.1H222.6c-3.4 0-6.4 2.1-7.5 5.3l-.4 1.2c-4.4 12.5-18.2 19-30.6 14.6s-19-18.2-14.6-30.6l.4-1.2zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z"
            >
                <animate
                    attributeName="fill"
                    to="#ffffff"
                    dur="0.5s"
                    fill="freeze"
                    begin="3500ms"
                />
            </path>
            <circle fill="none" stroke="#676a6f" stroke-width="0" cx="256" cy="256" r="250">
                <animate attributeName="stroke-width" to="5" dur="0.5s" fill="freeze" begin="3500ms" />
            </circle>
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(500)]
    pub animation_duration: u32,
    #[prop_or(3000)]
    pub animation_start_time: u32,
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

    let elapsed = use_raf(props.animation_duration, props.animation_start_time);

    let question_mark = use_slice::<QuestionMarkState>();

    // let timeout = {
    //     let question_mark = question_mark.clone();
    //     use_timeout(
    //         move || {
    //             question_mark.dispatch(QuestionMarkAction::StartFilling);
    //         },
    //         pr
    //     )
    // };

    let before_hiding_params = use_mut_ref(|| None);
    use_effect_with((*question_mark).clone(), {
        let before_hiding_params = before_hiding_params.clone();
        let node_ref = node_ref.clone();
        move |q| {
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

    let mut svg_style = if elapsed <= 0.0 {
        let mut s = format!(
            "transform: translate({}%, {}%); position: absolute; ",
            translate_x * 100.0,
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
    if elapsed == 0.0 {
        svg_style.push_str("animation: tooltip-fade-in 0.2s ease-out, ");
        if !props.mirror {
            svg_style.push_str("tooltip-slide-in-left 0.2s ease-out;");
        } else {
            svg_style.push_str("tooltip-slide-in-right 0.2s ease-out;");
        }
    }

    let ret = html! {
        <>
            <Global
                css={css!{
    "@keyframes tooltip-fade-in { 0% { opacity: 0; } }"
            }}
            />
            <Global
                css={css!{
    "@keyframes tooltip-slide-in-left { 0% { transform: translate(90%, -50%); } }"
            }}
            />
            <Global
                css={css!{
    "@keyframes tooltip-slide-in-right { 0% { transform: translate(-90%, -50%); } }"
            }}
            />
            <svg
                viewBox={view_box}
                class={props.classes.clone()}
                ref={node_ref}
                style={svg_style}
                opacity={(1.0 - elapsed).to_string()}
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
    if elapsed <= 0.0 {
        ret
    } else {
        let document = web_sys::window().unwrap().document().unwrap();
        let host = document.get_elements_by_tag_name("body").item(0).unwrap();

        create_portal(ret, host)
    }
}
