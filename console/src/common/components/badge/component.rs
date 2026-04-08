use dioxus::prelude::*;
use dioxus_primitives::icon;

const BADGE_BASE: &str = "inline-flex min-w-[20px] h-[20px] items-center justify-center rounded-[10px] shadow-[0_0_0_1px] shadow-white dark:shadow-gray-800 text-xs gap-1 px-2";

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum BadgeVariant {
    #[default]
    Primary,
    Secondary,
    Destructive,
    Outline,
}

impl BadgeVariant {
    pub fn class(&self) -> &'static str {
        match self {
            BadgeVariant::Primary => "bg-blue-600 text-white",
            BadgeVariant::Secondary => "bg-gray-100 dark:bg-gray-800 text-gray-900 dark:text-white",
            BadgeVariant::Destructive => "bg-red-600 text-white",
            BadgeVariant::Outline => {
                "border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 text-gray-700 dark:text-gray-300"
            }
        }
    }
}

/// The props for the [`Badge`] component.
#[derive(Props, Clone, PartialEq)]
pub struct BadgeProps {
    #[props(default)]
    pub variant: BadgeVariant,

    /// Additional attributes to extend the badge element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the badge element
    pub children: Element,
}

#[component]
pub fn Badge(props: BadgeProps) -> Element {
    rsx! {
        span {
            class: "{BADGE_BASE} {props.variant.class()}",
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn VerifiedIcon() -> Element {
    rsx! {
        // Badge icon from lucide https://lucide.dev/icons/badge-check
        icon::Icon {
            width: "12px",
            height: "12px",
            stroke: "var(--secondary-color-4)",
            path { d: "M3.85 8.62a4 4 0 0 1 4.78-4.77 4 4 0 0 1 6.74 0 4 4 0 0 1 4.78 4.78 4 4 0 0 1 0 6.74 4 4 0 0 1-4.77 4.78 4 4 0 0 1-6.75 0 4 4 0 0 1-4.78-4.77 4 4 0 0 1 0-6.76Z" }
            path { d: "m9 12 2 2 4-4" }
        }
    }
}
