use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub stalk_aspect_ratio: f64,
    // ratio to width
    pub rx: AttrValue,
    pub tip_width_ratio: f64,
    pub tip_height_ratio: f64,
    pub color: AttrValue,
    pub height: AttrValue,
    pub control_point_0_ratio: f64,
    pub control_point_1_ratio: f64,
    pub font_size_to_width: f64,
    pub text_color: AttrValue,
    pub letter_spacing_ratio: f64,
}

#[function_component]
pub fn Tooltip(props: &Props) -> Html {
    // suppose the width is 1
    // we want the whole width over height
    // the width of the tip is 1 * tip_width_ratio
    // the whole width is 1 + 1 * tip_width_ratio = 1 + tip_width_ratio
    // the height is 1 / stalk_aspect_ratio
    // the ratio then becomes (1 + tip_width_ratio) / (1 / stalk_aspect_ratio) = (1 +
    // tip_width_ratio) * stalk_aspect_ratio
    let aspect_ratio = (1.0 + props.tip_width_ratio) * props.stalk_aspect_ratio;

    let style = format!("height: {}; aspect-ratio: {};", props.height, aspect_ratio);

    let v_height = 100f64;
    let v_width = v_height * aspect_ratio;
    let view_box = format!("0 0 {} {}", v_width, v_height);

    let stalk_width = v_width * (1.0 / (1.0 + props.tip_width_ratio));
    let tip_width = v_width - stalk_width;
    let stalk_height = stalk_width / props.stalk_aspect_ratio;

    let touch_point_x = tip_width + 1.0;
    let upper_touch_point_y = (1.0 - props.tip_height_ratio) * stalk_height / 2.0;
    let lower_touch_point_y = (0.5 + props.tip_height_ratio / 2.0) * stalk_height;

    let control_point_0_x = tip_width * props.control_point_0_ratio;
    let tip_height = stalk_height * props.tip_height_ratio;
    let control_point_1_size = tip_height * props.control_point_1_ratio;

    let d = format!(
        "M 0 {} C {} {}, {} {}, {} {} v {} C {} {}, {} {}, {} {}",
        stalk_height / 2.0,

        control_point_0_x,
        stalk_height / 2.0,
        touch_point_x,
        upper_touch_point_y + control_point_1_size,
        touch_point_x,
        upper_touch_point_y,

        tip_height,

        touch_point_x,
        lower_touch_point_y - control_point_1_size,
        control_point_0_x,
        stalk_height / 2.0,
        0,
        stalk_height / 2.0,
    );

    html! {
        <svg viewBox={view_box} {style}>
            <rect
                x={tip_width.to_string()}
                width={stalk_width.to_string()}
                height={stalk_height.to_string()}
                rx={props.rx.clone()}
                fill={props.color.clone()}
            >
            </rect>
            <text
                x={(tip_width + stalk_width / 2.0).to_string()}
                y={(stalk_height / 2.0).to_string()}
                text-anchor="middle"
                dominant-baseline="central"
                fill={props.text_color.clone()}
                font-size={(props.font_size_to_width * stalk_width).to_string()}
                // sans-serif
                font-family="Arial"
                letter-spacing={(props.letter_spacing_ratio * stalk_width).to_string()}
            >
                {"ADD"}
            </text>
            <path {d} fill={props.color.clone()} />
        </svg>
    }
}
