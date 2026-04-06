use dioxus::prelude::*;
use dioxus_primitives::tabs::{self, TabContentProps, TabListProps, TabTriggerProps};

/// The props for the [`Tabs`] component.
#[derive(Props, Clone, PartialEq)]
pub struct TabsProps {
    /// The class of the tabs component.
    #[props(default)]
    pub class: String,

    /// The controlled value of the active tab.
    pub value: ReadSignal<Option<String>>,

    /// The default active tab value when uncontrolled.
    #[props(default)]
    pub default_value: String,

    /// Callback fired when the active tab changes.
    #[props(default)]
    pub on_value_change: Callback<String>,

    /// Whether the tabs are disabled.
    #[props(default)]
    pub disabled: ReadSignal<bool>,

    /// Whether the tabs are horizontal.
    #[props(default)]
    pub horizontal: ReadSignal<bool>,

    /// Whether focus should loop around when reaching the end.
    #[props(default = ReadSignal::new(Signal::new(true)))]
    pub roving_loop: ReadSignal<bool>,

    /// The variant of the tabs component.
    #[props(default)]
    pub variant: TabsVariant,

    /// Additional attributes to apply to the tabs element.
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the tabs component.
    pub children: Element,
}

/// The variant of the tabs component.
#[derive(Clone, Copy, PartialEq, Default)]
pub enum TabsVariant {
    /// The default variant.
    #[default]
    Default,
    /// The ghost variant.
    Ghost,
}

impl TabsVariant {
    /// Convert the variant to a string for use in class names
    fn to_class(self) -> &'static str {
        match self {
            TabsVariant::Default => "default",
            TabsVariant::Ghost => "ghost",
        }
    }
}

#[component]
pub fn Tabs(props: TabsProps) -> Element {
    rsx! {
        tabs::Tabs {
            class: props.class + " tabs flex w-full flex-col gap-2 [&[data-variant=default]_.tabs-list]:bg-gray-100 dark:[&[data-variant=default]_.tabs-list]:bg-gray-800 [&[data-variant=default]_.tabs-trigger[data-state=active]]:bg-white [&[data-variant=default]_.tabs-trigger[data-state=active]]:shadow-[0_1px_2px_rgba(0,0,0,0.18)] dark:[&[data-variant=default]_.tabs-trigger[data-state=active]]:bg-gray-800 dark:[&[data-variant=default]_.tabs-trigger[data-state=active]]:shadow-[inset_0_0_0_1px_#4b5563] [&[data-variant=default]_.tabs-content-themed]:border [&[data-variant=default]_.tabs-content-themed]:border-gray-200 [&[data-variant=default]_.tabs-content-themed]:rounded-lg [&[data-variant=default]_.tabs-content-themed]:bg-white [&[data-variant=default]_.tabs-content-themed]:shadow-[0_1px_2px_rgba(0,0,0,0.18)] dark:[&[data-variant=default]_.tabs-content-themed]:border-gray-600 dark:[&[data-variant=default]_.tabs-content-themed]:bg-gray-900 dark:[&[data-variant=default]_.tabs-content-themed]:shadow-none",
            "data-variant": props.variant.to_class(),
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            horizontal: props.horizontal,
            roving_loop: props.roving_loop,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabList(props: TabListProps) -> Element {
    rsx! {
        tabs::TabList { class: "tabs-list flex w-fit box-border flex-1 flex-row p-1 border-none rounded-lg gap-1", attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn TabTrigger(props: TabTriggerProps) -> Element {
    rsx! {
        tabs::TabTrigger {
            class: "tabs-trigger py-1 px-2 border-none bg-none text-gray-500 dark:text-gray-400 cursor-pointer rounded-[calc(0.5rem-0.25rem)] data-[state=active]:text-gray-900 dark:data-[state=active]:text-white data-[disabled=true]:text-gray-500 dark:data-[disabled=true]:text-gray-400 data-[disabled=true]:cursor-not-allowed hover:not-data-[disabled=true]:text-gray-400 focus-visible:text-gray-400",
            id: props.id,
            value: props.value,
            index: props.index,
            disabled: props.disabled,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn TabContent(props: TabContentProps) -> Element {
    rsx! {
        tabs::TabContent {
            class: props.class.unwrap_or_default() + " tabs-content tabs-content-themed w-full box-border p-1 data-[state=inactive]:hidden",
            value: props.value,
            id: props.id,
            index: props.index,
            attributes: props.attributes,
            {props.children}
        }
    }
}
