use dioxus::prelude::*;
use dioxus_primitives::icon;

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum PaginationLinkSize {
    #[default]
    Icon,
    Default,
}

impl PaginationLinkSize {
    pub fn class(&self) -> &'static str {
        match self {
            PaginationLinkSize::Icon => "icon",
            PaginationLinkSize::Default => "default",
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
#[non_exhaustive]
pub enum PaginationLinkKind {
    Previous,
    Next,
}

impl PaginationLinkKind {
    pub fn attr(&self) -> &'static str {
        match self {
            PaginationLinkKind::Previous => "previous",
            PaginationLinkKind::Next => "next",
        }
    }
}

#[component]
pub fn Pagination(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        nav {
            class: "pagination flex w-full justify-center mx-auto",
            "data-slot": "pagination",
            role: "navigation",
            aria_label: "pagination",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationContent(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        ul {
            class: "pagination-content flex items-center p-0 m-0 gap-1 list-none",
            "data-slot": "pagination-content",
            ..attributes,
            {children}
        }
    }
}

#[component]
pub fn PaginationItem(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        li {
            class: "pagination-item",
            "data-slot": "pagination-item",
            ..attributes,
            {children}
        }

    }
}

#[derive(Props, Clone, PartialEq)]
pub struct PaginationLinkProps {
    #[props(default)]
    pub is_active: bool,
    #[props(default)]
    pub size: PaginationLinkSize,
    #[props(default)]
    pub data_kind: Option<PaginationLinkKind>,
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    pub attributes: Vec<Attribute>,
    pub children: Element,
}

#[component]
pub fn PaginationLink(props: PaginationLinkProps) -> Element {
    let aria_current = if props.is_active { Some("page") } else { None };
    let data_kind = props.data_kind.map(|kind| kind.attr());
    rsx! {
        a {
            class: "pagination-link inline-flex box-border items-center justify-center rounded-[0.625rem] text-gray-700 text-sm font-medium gap-2 leading-none no-underline transition-colors dark:text-gray-300 focus-visible:shadow-[0_0_0_2px_theme(colors.blue.500)] data-[size=icon]:w-8 data-[size=icon]:h-8 data-[size=icon]:p-0 data-[size=default]:h-8 data-[size=default]:py-2 data-[size=default]:px-4 data-[active=true]:border data-[active=true]:border-gray-200 data-[active=true]:bg-white dark:data-[active=true]:border-gray-700 dark:data-[active=true]:bg-gray-900 data-[active=true]:hover:bg-gray-200 dark:data-[active=true]:hover:bg-gray-700 data-[active=false]:hover:bg-gray-100 data-[active=false]:hover:text-gray-900 dark:data-[active=false]:hover:bg-gray-800 dark:data-[active=false]:hover:text-white data-[kind=previous]:pl-2.5 data-[kind=previous]:pr-2.5 data-[kind=previous]:gap-1 data-[kind=next]:pl-2.5 data-[kind=next]:pr-2.5 data-[kind=next]:gap-1",
            "data-slot": "pagination-link",
            "data-active": props.is_active,
            "data-size": props.size.class(),
            "data-kind": data_kind,
            aria_current,
            onclick: move |event| {
                if let Some(f) = &props.onclick {
                    f.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(f) = &props.onmousedown {
                    f.call(event);
                }
            },
            onmouseup: move |event| {
                if let Some(f) = &props.onmouseup {
                    f.call(event);
                }
            },
            ..props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn PaginationPrevious(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: "Go to previous page",
            data_kind: Some(PaginationLinkKind::Previous),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            // ChevronLeft icon from lucide https://lucide.dev/icons/chevron-left
            icon::Icon { width: "1rem", height: "1rem",
                polyline { points: "15 6 9 12 15 18" }
            }
            span { class: "hidden sm:inline", "Previous" }
        }
    }
}

#[component]
pub fn PaginationNext(
    onclick: Option<EventHandler<MouseEvent>>,
    onmousedown: Option<EventHandler<MouseEvent>>,
    onmouseup: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)]
    #[props(extends = a)]
    attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        PaginationLink {
            size: PaginationLinkSize::Default,
            aria_label: "Go to next page",
            data_kind: Some(PaginationLinkKind::Next),
            onclick,
            onmousedown,
            onmouseup,
            attributes,
            span { class: "hidden sm:inline", "Next" }
            // ChevronRight icon from lucide https://lucide.dev/icons/chevron-right
            icon::Icon { width: "1rem", height: "1rem",
                polyline { points: "9 6 15 12 9 18" }
            }
        }
    }
}

#[component]
pub fn PaginationEllipsis(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        span {
            class: "pagination-ellipsis flex items-center justify-center text-gray-700 w-8 h-8 dark:text-gray-300",
            "data-slot": "pagination-ellipsis",
            aria_hidden: "true",
            ..attributes,
            // MoreHorizontal icon from lucide https://lucide.dev/icons/more-horizontal
            icon::Icon { width: "1rem", height: "1rem", fill: "currentColor",
                circle { cx: "5", cy: "12", r: "1.5" }
                circle { cx: "12", cy: "12", r: "1.5" }
                circle { cx: "19", cy: "12", r: "1.5" }
            }
            span { class: "sr-only", "More pages" }
        }
    }
}
