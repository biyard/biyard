use dioxus::prelude::*;

#[derive(Copy, Clone, PartialEq, Default)]
pub enum BadgeColor {
    #[default]
    Green,
    Red,
    Yellow,
    Blue,
    Purple,
    Gray,
}

impl BadgeColor {
    fn classes(&self) -> &'static str {
        match self {
            BadgeColor::Green => "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
            BadgeColor::Red => "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
            BadgeColor::Yellow => "bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-300",
            BadgeColor::Blue => "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
            BadgeColor::Purple => "bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200",
            BadgeColor::Gray => "bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200",
        }
    }
}

#[component]
pub fn StatusBadge(color: BadgeColor, children: Element) -> Element {
    rsx! {
        span {
            class: "inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-semibold {color.classes()}",
            {children}
        }
    }
}
