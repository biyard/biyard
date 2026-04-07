use dioxus::prelude::*;

#[component]
pub fn DataTable(children: Element) -> Element {
    // `overflow-x-auto` lets the table scroll horizontally on narrow
    // viewports instead of truncating or squishing columns. The
    // `min-w-[640px]` on the table itself keeps columns from collapsing
    // even when the wrapper is narrower than 640px — the scrollbar
    // takes over.
    rsx! {
        div { class: "overflow-x-auto rounded-[28px] border border-border bg-panel shadow-[0_18px_40px_rgba(15,23,42,0.05)]",
            table { class: "w-full min-w-[640px] text-left",
                {children}
            }
        }
    }
}

#[component]
pub fn TableHead(children: Element) -> Element {
    rsx! {
        thead { class: "bg-panel-muted",
            tr { {children} }
        }
    }
}

#[component]
pub fn TableHeadCell(children: Element) -> Element {
    rsx! {
        th { class: "px-6 py-4 text-[11px] font-semibold uppercase tracking-[0.14em] text-foreground-muted",
            {children}
        }
    }
}

#[component]
pub fn TableBody(children: Element) -> Element {
    rsx! {
        tbody { class: "divide-y divide-border bg-panel",
            {children}
        }
    }
}

#[component]
pub fn TableCell(#[props(default)] class: &'static str, children: Element) -> Element {
    rsx! {
        td { class: "px-6 py-4 align-middle {class}",
            {children}
        }
    }
}
