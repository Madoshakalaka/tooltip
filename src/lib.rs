use std::rc::Rc;

use bounce::{use_slice, use_slice_dispatch, use_slice_value, Slice};
use web_sys::{Element, SvgAnimateMotionElement, SvgAnimationElement};
use yew::prelude::*;
use yew_hooks::use_timeout;

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
pub struct QuestionMarkState(Option<NodeRef>);

pub enum QuestionMarkAction {
    Set(NodeRef),
}

impl Reducible for QuestionMarkState {
    type Action = QuestionMarkAction;

    fn reduce(mut self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let question_mark = Rc::make_mut(&mut self);
        match action {
            QuestionMarkAction::Set(node_ref) => {
                question_mark.0 = Some(node_ref);
            }
        }
        self
    }
}

#[derive(PartialEq, Default, Slice, Clone)]
pub struct TooltipGroupState {
    tooltips: Vec<NodeRef>,
    // question_mark: Option<NodeRef>,
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
}

#[function_component]
pub fn QuestionMark(props: &QuestionMarkProps) -> Html {
    let node_ref = use_node_ref();
    let dispatch_group_state = use_slice_dispatch::<QuestionMarkState>();
    dispatch_group_state(QuestionMarkAction::Set(node_ref.clone()));

    html! {
        <svg viewBox="0 0 512 512" class={props.classes.clone()} ref={node_ref}>
            <path
                fill="#676a6f"
                d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM169.8 165.3c7.9-22.3 29.1-37.3 52.8-37.3h58.3c34.9 0 63.1 28.3 63.1 63.1c0 22.6-12.1 43.5-31.7 54.8L280 264.4c-.2 13-10.9 23.6-24 23.6c-13.3 0-24-10.7-24-24V250.5c0-8.6 4.6-16.5 12.1-20.8l44.3-25.4c4.7-2.7 7.6-7.7 7.6-13.1c0-8.4-6.8-15.1-15.1-15.1H222.6c-3.4 0-6.4 2.1-7.5 5.3l-.4 1.2c-4.4 12.5-18.2 19-30.6 14.6s-19-18.2-14.6-30.6l.4-1.2zM224 352a32 32 0 1 1 64 0 32 32 0 1 1 -64 0z"
            />
        </svg>
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
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

#[function_component]
pub fn Tooltip(props: &Props) -> Html {
    let node_ref = use_node_ref();
    let dispatch_group_state = use_slice_dispatch::<TooltipGroupState>();
    let question_mark = use_slice_value::<QuestionMarkState>();

    let hiding = use_state(|| false);
    let before_hiding_params = use_mut_ref(|| None);
    use_effect({
        let hiding = *hiding;
        let before_hiding_params = before_hiding_params.clone();
        let node_ref = node_ref.clone();
        move || {
            if !hiding {
                let svg = node_ref.cast::<Element>().unwrap();
                let rect = svg.get_bounding_client_rect();
                *(before_hiding_params.borrow_mut()) = Some(rect);
            }
        }
    });

    let timeout = {
        let hiding = hiding.setter();
        use_timeout(
            move || {
                hiding.set(true);
            },
            1000,
        )
    };

    dispatch_group_state(Action::AddTooltip(node_ref.clone()));

    let v_height = 100f64;
    let font_size = props.font_height_ratio * v_height;

    let padding_inline_sum = v_height * (1.0 - props.font_height_ratio) * props.inline_padding;

    let stalk_width = padding_inline_sum + calculate_text_width(&props.text, font_size);

    let tip_height = v_height * props.tip_height_ratio;
    let tip_width = tip_height * props.tip_aspect_ratio;

    let v_width = stalk_width + tip_width;

    let (view_box, translate_x, height, translate_y, v_drift, h_drift) = if !*hiding {
        (
            format!("0 0 {} {}", v_width, v_height),
            if !props.mirror {1.0} else {-1.0},
            props.height.clone(),
            -0.5,
            None,
            None
        )
    } else {
        let question_mark = question_mark.0.as_ref().unwrap().cast::<Element>().unwrap();
        let rect = question_mark.get_bounding_client_rect();
        let tooltip_rect = before_hiding_params.borrow().as_ref().unwrap().clone();
        let new_v_width = v_width / tooltip_rect.width() * (tooltip_rect.right() - rect.x());
        let h_drift = v_width / tooltip_rect.width()
            * ((rect.x() + rect.width() / 2.0)
                - (tooltip_rect.x() + tooltip_rect.width() / 2.0));


        let new_v_height = v_height / tooltip_rect.height() * (rect.bottom() - tooltip_rect.top());
        let v_drift = v_height / tooltip_rect.height()
            * ((rect.bottom() - rect.height() / 2.0)
                - ( tooltip_rect.bottom()
                    - tooltip_rect.height() / 2.0 ));

        let new_x = -1.0 * (new_v_width - v_width);
        (
            format!("{new_x} 0 {new_v_width} {new_v_height}"),
            // v_width / new_v_width,
            if !props.mirror {v_width / new_v_width} else {-1.0},
            format!("calc({} * {})", props.height, new_v_height / v_height).into(),
            -1.0 * v_height / 2.0 / new_v_height,
            Some(v_drift),
            Some(h_drift)
        )
    };

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

    let motion_ref = use_node_ref();
    let rounding_ref = use_node_ref();
    let transparency_ref = use_node_ref();
    
    use_effect_with(*hiding, {
        let motion_ref = motion_ref.clone();
        let rounding_ref = rounding_ref.clone();
        let transparency_ref = transparency_ref.clone();
        move |hiding| {
            if *hiding {
                let _ = motion_ref
                    .cast::<SvgAnimateMotionElement>()
                    .unwrap()
                    .begin_element();
                // let _ = rounding_ref
                //     .cast::<SvgAnimationElement>()
                //     .unwrap()
                //     .begin_element();
                let _ = transparency_ref
                    .cast::<SvgAnimationElement>()
                    .unwrap()
                    .begin_element();
            }
        }
    });
    let dur = "0.5s";
    let animate_motion = hiding.then(|| {
        let v_drift = v_drift.unwrap();
        let h_drift = h_drift.unwrap();
        html! {
            <@{"animateMotion"} ref={motion_ref} {dur} path={format!( "M 0 0 v {v_drift} h {h_drift}" )} fill="freeze" />

        }
    });
    let animate_rounding = hiding.then(|| {
        html! {
            <animate ref={rounding_ref} attributeName="rx" {dur} to={(v_height/2.0).to_string()} fill="freeze" />
        }
    });
    let animate_transparency = hiding.then(|| {
        html! {
            <animate ref={transparency_ref} attributeName="fill-opacity" {dur} to="0" fill="freeze" />
        }
    });

    html! {
        <svg
            viewBox={view_box}
            class={props.classes.clone()}
            ref={node_ref}
            style={format!("transform: translate({}%, {}%); height: {};", translate_x * 100.0, translate_y * 100.0, height)}
        >
            <g>
                { animate_motion }
                { animate_transparency }
                <rect
                    x={if !props.mirror {tip_width.to_string()} else {0.0.to_string()}}
                    width={stalk_width.to_string()}
                    height={v_height.to_string()}
                    rx={( props.border_radius_ratio * v_height ).to_string()}
                    fill={props.background_color.clone()}
                >
                // { animate_rounding }
                </rect>
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
            </g>
        </svg>
    }
}
