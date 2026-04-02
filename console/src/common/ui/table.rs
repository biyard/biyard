use dioxus::prelude::*;

#[component]
pub fn DataTable(children: Element) -> Element {
    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg overflow-hidden",
            table { class: "min-w-full divide-y divide-gray-200 dark:divide-gray-700",
                {children}
            }
        }
    }
}

#[component]
pub fn TableHead(children: Element) -> Element {
    rsx! {
        thead { class: "bg-gray-50 dark:bg-gray-700",
            tr { {children} }
        }
    }
}

#[component]
pub fn TableHeadCell(children: Element) -> Element {
    rsx! {
        th { class: "px-6 py-3 text-left text-xs font-medium text-gray-500 dark:text-gray-300 uppercase",
            {children}
        }
    }
}

#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! {
        tbody { class: "bg-white dark:bg-gray-800 divide-y divide-gray-200 dark:divide-gray-700",
            {children}
        }
    }
}

#[component]
pub fn TableCell(
    #[props(default)] class: &'static str,
    children: Element,
) -> Element {
    rsx! {
        td { class: "px-6 py-4 whitespace-nowrap {class}",
            {children}
        }
    }
}
