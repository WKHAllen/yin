use super::*;
use crate::util::*;
use yew::prelude::*;

/// Slider properties.
#[derive(Properties, PartialEq, Clone)]
pub struct SliderProps<N: Number> {
    /// The slider state.
    pub state: UseStateHandle<N>,
    /// The slider label.
    #[prop_or_default]
    pub label: String,
    /// The minimum value.
    #[prop_or(N::NUMBER_MIN)]
    pub min: N,
    /// The maximum value.
    #[prop_or(N::NUMBER_MAX)]
    pub max: N,
    /// The step size.
    #[prop_or(N::NUMBER_STEP)]
    pub step: N,
    /// Whether the slider is disabled.
    #[prop_or(false)]
    pub disabled: bool,
}

/// A slider component.
#[function_component]
pub fn Slider<N: Number + 'static>(props: &SliderProps<N>) -> Html {
    let SliderProps {
        state,
        label,
        min,
        max,
        step,
        disabled,
    } = props.clone();

    let id_state = use_state(new_id);
    let id = (*id_state).clone();
    let value = *state;
    let progress = (value.as_f64() - min.as_f64()) / (max.as_f64() - min.as_f64());
    let width_percentage =
        ((value.as_f64() - min.as_f64()) * 100.0f64) / (max.as_f64() - min.as_f64());
    let thumb_transform_style = format!("left: {width_percentage}%");
    let oninput = move |event: InputEvent| {
        let value_str = input_event_value(event);
        let value = value_str
            .parse::<N>()
            .map_err(|_| format!("failed to parse '{value_str}' as a number"))
            .unwrap();
        state.set(value);
    };

    html! {
        <div class={classes!("base-slider-container", disabled.then_some("base-slider-disabled"))}>
            <label for={id.clone()} class="base-slider-label">{label}</label>
            <div class="base-slider">
                <div class="base-slider-track">
                    <ProgressBar {progress} {disabled} />
                </div>
                <div class="base-slider-thumb-container">
                    <div class="base-slider-thumb" style={thumb_transform_style}></div>
                </div>
                <input
                    type="range"
                    {id}
                    value={value.to_string()}
                    min={min.to_string()}
                    max={max.to_string()}
                    step={step.to_string()}
                    {oninput}
                    {disabled}
                    class="base-slider-input"
                />
            </div>
        </div>
    }
}
