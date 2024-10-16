use yew::prelude::*;

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
    #[prop_or(0.1)]
    pub control_point_0_ratio: f64,
    #[prop_or(0.1)]
    pub control_point_1_ratio: f64,
    #[prop_or(0.31)]
    pub font_height_ratio: f64,

    #[prop_or("#ffffff".into())]
    pub text_color: AttrValue,
    pub text: AttrValue,
}

#[function_component]
pub fn Tooltip(props: &Props) -> Html {
    let v_height = 100f64;
    let font_size = props.font_height_ratio * v_height;

    // estimate the width of the text:
    // assuming the width of the text is equal to its height
    // (seems to be the case for uppercase letters)
    let padding_inline_sum = v_height * (1.0 - props.font_height_ratio);
    let stalk_width = props.text.len() as f64 * font_size + padding_inline_sum;

    let style = format!("height: {};", props.height);

    let tip_height = v_height * props.tip_height_ratio;
    let tip_width = tip_height * props.tip_aspect_ratio;

    let v_width = stalk_width + tip_width;
    let view_box = format!("0 0 {} {}", v_width + 1.0, v_height);

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

    html! {
        <svg viewBox={view_box} {style} class={props.classes.clone()}>
            <rect
                x={tip_width.to_string()}
                width={stalk_width.to_string()}
                height={v_height.to_string()}
                rx={( props.border_radius_ratio * v_height ).to_string()}
                fill={props.background_color.clone()}
            />
            <text
                x={(tip_width + stalk_width / 2.0).to_string()}
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
            <path {d} fill={props.background_color.clone()} />
        </svg>
    }
}
