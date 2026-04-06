use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum StatColor {
    #[default]
    Gray,
    Green,
    Red,
    Blue,
    Emerald,
    Indigo,
    Amber,
    Purple,
}

impl StatColor {
    fn bg_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "bg-gray-50 dark:bg-gray-700",
            StatColor::Green => "bg-green-50 dark:bg-green-900/20",
            StatColor::Red => "bg-red-50 dark:bg-red-900/20",
            StatColor::Blue => "bg-blue-50 dark:bg-blue-900/20",
            StatColor::Emerald => "bg-emerald-50 dark:bg-emerald-900/20",
            StatColor::Indigo => "bg-indigo-50 dark:bg-indigo-900/20",
            StatColor::Amber => "bg-amber-50 dark:bg-amber-900/20",
            StatColor::Purple => "bg-purple-50 dark:bg-purple-900/20",
        }
    }

    fn label_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "text-sm font-medium text-gray-500 dark:text-gray-400",
            StatColor::Green => "text-sm font-medium text-green-600 dark:text-green-400",
            StatColor::Red => "text-sm font-medium text-red-600 dark:text-red-400",
            StatColor::Blue => "text-sm font-medium text-blue-600 dark:text-blue-400",
            StatColor::Emerald => "text-sm font-medium text-emerald-600 dark:text-emerald-400",
            StatColor::Indigo => "text-sm font-medium text-indigo-600 dark:text-indigo-400",
            StatColor::Amber => "text-sm font-medium text-amber-600 dark:text-amber-400",
            StatColor::Purple => "text-sm font-medium text-purple-600 dark:text-purple-400",
        }
    }

    fn value_class(&self) -> &'static str {
        match self {
            StatColor::Gray => "mt-1 text-xl font-semibold text-gray-900 dark:text-white",
            StatColor::Green => "mt-1 text-2xl font-semibold text-green-700 dark:text-green-300",
            StatColor::Red => "mt-1 text-2xl font-semibold text-red-700 dark:text-red-300",
            StatColor::Blue => "mt-1 text-2xl font-semibold text-blue-700 dark:text-blue-300",
            StatColor::Emerald => "mt-1 text-2xl font-semibold text-emerald-700 dark:text-emerald-300",
            StatColor::Indigo => "mt-1 text-2xl font-semibold text-indigo-700 dark:text-indigo-300",
            StatColor::Amber => "mt-1 text-2xl font-semibold text-amber-700 dark:text-amber-300",
            StatColor::Purple => "mt-1 text-2xl font-semibold text-purple-700 dark:text-purple-300",
        }
    }
}

#[component]
pub fn StatCard(
    label: String,
    value: String,
    #[props(default)] color: StatColor,
) -> Element {
    rsx! {
        div { class: "{color.bg_class()} rounded-lg p-4",
            dt { class: color.label_class(), "{label}" }
            dd { class: color.value_class(), "{value}" }
        }
    }
}
