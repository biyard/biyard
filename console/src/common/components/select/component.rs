use dioxus::prelude::*;
use dioxus_primitives::icon;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectGroupProps, SelectListProps, SelectOptionProps, SelectProps,
    SelectTriggerProps, SelectValueProps,
};

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        select::Select {
            class: "select relative [&[data-state=open]_.select-trigger]:pointer-events-none [&[data-disabled=true]_.select-trigger]:text-gray-500 [&[data-disabled=true]_.select-trigger]:cursor-not-allowed dark:[&[data-disabled=true]_.select-trigger]:text-gray-400 [&_.select-trigger_span[data-placeholder=true]]:text-gray-500 dark:[&_.select-trigger_span[data-placeholder=true]]:text-gray-400",
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            name: props.name,
            placeholder: props.placeholder,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectTrigger(props: SelectTriggerProps) -> Element {
    rsx! {
        select::SelectTrigger { class: "select-trigger relative flex box-border flex-row items-center justify-between p-1 px-3 py-2 border-none rounded-lg bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-300 cursor-pointer gap-1 transition-colors shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#4b5563] hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 hover:not-data-[disabled=true]:outline-none focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-100/10 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-100/10 dark:focus-visible:text-white", attributes: props.attributes,
            {props.children}
            icon::Icon {
                width: "20px",
                height: "20px",
                stroke: "var(--primary-color-7)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn SelectValue(props: SelectValueProps) -> Element {
    rsx! {
        select::SelectValue { attributes: props.attributes }
    }
}

#[component]
pub fn SelectList(props: SelectListProps) -> Element {
    rsx! {
        select::SelectList {
            class: "select-list absolute z-[1000] top-full left-0 min-w-full box-border p-1 rounded-lg mt-1 bg-white dark:bg-gray-100/10 opacity-0 pointer-events-none origin-top will-change-transform shadow-[inset_0_0_0_1px_#e5e7eb] dark:shadow-[inset_0_0_0_1px_#4b5563]",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroup(props: SelectGroupProps) -> Element {
    rsx! {
        select::SelectGroup {
            class: "select-group",
            disabled: props.disabled,
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    rsx! {
        select::SelectGroupLabel {
            class: "select-group-label px-3 py-1 text-gray-500 dark:text-gray-400 text-xs",
            id: props.id,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + 'static>(props: SelectOptionProps<T>) -> Element {
    rsx! {
        select::SelectOption::<T> {
            class: "select-option flex items-center justify-between px-3 py-2 rounded-sm cursor-pointer text-sm data-[disabled=true]:text-gray-500 data-[disabled=true]:cursor-not-allowed dark:data-[disabled=true]:text-gray-400 hover:not-data-[disabled=true]:bg-gray-200 hover:not-data-[disabled=true]:text-gray-900 focus-visible:bg-gray-200 focus-visible:text-gray-900 focus-visible:outline-none dark:hover:not-data-[disabled=true]:bg-gray-600 dark:hover:not-data-[disabled=true]:text-white dark:focus-visible:bg-gray-600 dark:focus-visible:text-white",
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn SelectItemIndicator() -> Element {
    rsx! {
        select::SelectItemIndicator {
            icon::Icon {
                width: "1rem",
                height: "1rem",
                stroke: "var(--secondary-color-5)",
                path { d: "M5 13l4 4L19 7" }
            }
        }
    }
}
