use dioxus::prelude::*;

use crate::Route;
use crate::features::console::components::ConsoleSidebar;

#[component]
pub fn Layout() -> Element {
    rsx! {
        div { class: "min-h-screen bg-gray-50 dark:bg-gray-900",
            ConsoleSidebar {}
            main { class: "flex-1 ml-64",
                div { class: "max-w-7xl mx-auto py-6 px-4 sm:px-6 lg:px-8",
                    SuspenseBoundary {
                        fallback: move |_| rsx! {
                            div { class: "flex justify-center py-12",
                                svg {
                                    class: "h-8 w-8 text-gray-400 animate-spin",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    circle {
                                        class: "opacity-25",
                                        cx: "12",
                                        cy: "12",
                                        r: "10",
                                        stroke: "currentColor",
                                        stroke_width: "4",
                                    }
                                    path {
                                        class: "opacity-75",
                                        fill: "currentColor",
                                        d: "M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z",
                                    }
                                }
                            }
                        },
                        Outlet::<Route> {}
                    }
                }
            }
        }
    }
}
