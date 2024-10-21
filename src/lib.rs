use yew::prelude::*;

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
    let v_height = 100f64;
    let font_size = props.font_height_ratio * v_height;

    let padding_inline_sum = v_height * (1.0 - props.font_height_ratio) * props.inline_padding;

    let stalk_width = padding_inline_sum + calculate_text_width(&props.text, font_size);

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
    }
}
