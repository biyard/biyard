use dioxus::prelude::*;
use dioxus_primitives::slider::{
    self, SliderProps, SliderRangeProps, SliderThumbProps, SliderTrackProps,
};

#[component]
pub fn Slider(props: SliderProps) -> Element {
    rsx! {
        slider::Slider {
            class: "slider relative flex w-[200px] items-center py-2 touch-none data-[orientation=vertical]:w-auto data-[orientation=vertical]:h-[200px] data-[orientation=vertical]:flex-col [&[data-orientation=vertical]_.slider-track]:w-1 [&[data-orientation=vertical]_.slider-track]:h-full [&[data-orientation=vertical]_.slider-range]:w-full [&[data-orientation=vertical]_.slider-thumb]:left-1/2 [&[data-orientation=vertical]_.slider-thumb]:-translate-x-1/2 [&[data-orientation=vertical]_.slider-thumb]:translate-y-1/2 data-[disabled=true]:cursor-not-allowed data-[disabled=true]:opacity-50 [&[data-disabled=true]_.slider-thumb]:cursor-not-allowed",
            value: props.value,
            default_value: props.default_value,
            min: props.min,
            max: props.max,
            step: props.step,
            disabled: props.disabled,
            horizontal: props.horizontal,
            inverted: props.inverted,
            on_value_change: props.on_value_change,
            label: props.label,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SliderTrack(props: SliderTrackProps) -> Element {
    rsx! {
        slider::SliderTrack { class: "slider-track relative h-2 box-border grow rounded-full bg-gray-100 dark:bg-gray-800", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderRange(props: SliderRangeProps) -> Element {
    rsx! {
        slider::SliderRange { class: "slider-range absolute h-full rounded-full bg-blue-600", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn SliderThumb(props: SliderThumbProps) -> Element {
    rsx! {
        slider::SliderThumb {
            class: "slider-thumb absolute top-1/2 block w-4 h-4 border border-solid border-blue-600 rounded-full bg-[#fafafa] dark:bg-[#0a0a0a] cursor-pointer -translate-x-1/2 -translate-y-1/2 transition-[border-color] duration-150 hover:shadow-[0_0_0_4px_rgba(176,176,176,0.5)] focus-visible:shadow-[0_0_0_4px_rgba(176,176,176,0.5)] dark:hover:shadow-[0_0_0_4px_rgba(62,62,62,0.5)] dark:focus-visible:shadow-[0_0_0_4px_rgba(62,62,62,0.5)] hover:transition-shadow hover:duration-150",
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
