use web_sys::HtmlInputElement;
use yew::prelude::*;

use tooltip::Tooltip;

const BUTTON_SIZE: f64 = 60.0;

#[function_component]
pub fn MockButton() -> Html {
    html! {
        // center the svg
        <div
            style={format!(r#"
                border-radius: 20%;
                border: 2px solid rgb(209,213,219);
                display: flex;
                justify-content: center;
                align-items: center;
                height: {}px;
                aspect-ratio: 1/1;
                "#, BUTTON_SIZE)}
        >
            <svg style="height: 50%; aspect-ratio: 14/16;" viewBox="0 0 448 512">
                <path
                    fill="black"
                    d="M256 80c0-17.7-14.3-32-32-32s-32 14.3-32 32V224H48c-17.7 0-32 14.3-32 32s14.3 32 32 32H192V432c0 17.7 14.3 32 32 32s32-14.3 32-32V288H400c17.7 0 32-14.3 32-32s-14.3-32-32-32H256V80z"
                />
            </svg>
        </div>
    }
}

#[function_component]
pub fn App() -> Html {
    let height_percentage = use_state(|| 0.69); // nice
    let border_radius = use_state(|| 0.12);
    // initial is pure blue
    let background_color = use_state(|| "#676a6f".to_string());

    let tip_height_ratio = use_state(|| 1.0/6.0);
    let tip_aspect_ratio = use_state(|| 4.0/3.0);

    let tip_pointness = use_state(|| 0.1);
    let contact_convergence = use_state(|| 0.1);

    let font_height_ratio = use_state(|| 0.31);
    let text_color = use_state(|| "#ffffff".to_string());
    let text = use_state(|| AttrValue::from("ADD"));

    html! {
        <>
            <div
                style=r#"
            display: flex;
            justify-content: center;
            align-items: center;
            gap: 8px;
            margin-top: 20vh;
            "#
            >
                <MockButton />
                <Tooltip
                    border_radius_ratio={*border_radius}
                    tip_height_ratio={*tip_height_ratio}
                    tip_aspect_ratio={*tip_aspect_ratio}
                    background_color={AttrValue::from(( *background_color ).clone())}
                    height={format!("{}px", *height_percentage * BUTTON_SIZE)}
                    control_point_0_ratio={*tip_pointness}
                    control_point_1_ratio={*contact_convergence}
                    font_height_ratio={*font_height_ratio}
                    text_color={AttrValue::from(( *text_color ).clone())}
                    text={(*text).clone()}
                />
            </div>
            <div
                style=r#"
            display: flex;
            flex-direction: column;
            margin-inline: auto;
            margin-top: 32px;
            width: 180px;
            gap: 10px;
                 font-size: smaller;
            "#
            >
                <label>
                    { format!{"Height Percentage: {:.2}%", *height_percentage * 100.0} }
                    <br />
                    <input
                        type="range"
                        value={height_percentage.to_string()}
                        min=0.4
                        max=1.0
                        step=0.01
                        oninput={move |e: InputEvent| {
                          let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                          height_percentage.set(target.value_as_number());
                  }}
                    />
                </label>
                <label>
                    { format!{"Border Radius: {:.2}%", *border_radius * 100.0} }
                    <br />
                    <input
                        type="range"
                        value={border_radius.to_string()}
                        min=0.05
                        max=0.20
                        step=0.005
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            border_radius.set(target.value_as_number());
                    }}
                    />
                </label>
                <label>
                    { format!("Tip Height Ratio: {:.2}% ", *tip_height_ratio * 100.0) }
                    <br />
                    <input
                        type="range"
                        value={tip_height_ratio.to_string()}
                        min=0.1
                        max=0.5
                        step=0.01
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            tip_height_ratio.set(target.value_as_number());
                    }}
                    />
                </label>
                <label>
                  { format!("Tip Aspect Ratio: {:.2}", *tip_aspect_ratio) }
                    <br />
                    <input
                        type="range"
                        value={tip_aspect_ratio.to_string()}
                        min=0.2
                        max=3.0
                        step=0.01
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            tip_aspect_ratio.set(target.value_as_number());
                    }}
                    />
                </label>
                <label>
                    { format!("Tip Pointness A: {:.2}% ", *tip_pointness * 100.0) }
                    <br />
                    <input
                        type="range"
                        value={tip_pointness.to_string()}
                        min=0.001
                        max=0.3
                        step=0.001
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            tip_pointness.set(target.value_as_number());
                    }}
                    />
                </label>
                <label>
                    { format!("Tip Pointness B: {:.2}% ", *contact_convergence * 100.0) }
                    <br />
                    <input
                        type="range"
                        value={contact_convergence.to_string()}
                        min=0.001
                        max=0.3
                        step=0.001
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            contact_convergence.set(target.value_as_number());
                    }}
                    />
                </label>
                <label>
                    { format!("Font Height Ratio: {:.2}% ", *font_height_ratio * 100.0) }
                    <br />
                    <input
                        type="range"
                        value={font_height_ratio.to_string()}
                        min=0.1
                        max=0.8
                        step=0.01
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            font_height_ratio.set(target.value_as_number());
                    }}
                    />
                </label>

                <label>
                    { format!{"Backgroud Color: {}", *background_color} }
                    <br />
                    <input
                        type="color"
                        value={background_color.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            background_color.set(target.value());
                    }}
                    />
                </label>
                <label>
                    { format!{"Text Color: {}", *text_color} }
                    <br />
                    <input
                        type="color"
                        value={text_color.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            text_color.set(target.value());
                    }}
                    />
                </label>
                <label>
                    { format!{"Text: {}", *text} }
                    <br />
                    <input
                        type="text"
                        value={text.to_string()}
                        oninput={move |e: InputEvent| {
                            let target = e.target_dyn_into::<HtmlInputElement>().unwrap();
                            text.set(AttrValue::from(target.value()));
                    }}
                    />
                </label>

            </div>
        </>
    }
}
