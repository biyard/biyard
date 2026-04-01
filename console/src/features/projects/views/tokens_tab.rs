use dioxus::prelude::*;
use dioxus_translate::use_translate;

use crate::common::ProjectPartition;
use crate::features::projects::i18n::ProjectsTranslate;

#[component]
pub fn TokensTab(project_id: ReadSignal<ProjectPartition>) -> Element {
    let t: ProjectsTranslate = use_translate();

    rsx! {
        div { class: "bg-white dark:bg-gray-800 shadow rounded-lg p-12 text-center",
            svg {
                class: "mx-auto h-12 w-12 text-gray-400",
                xmlns: "http://www.w3.org/2000/svg",
                width: "24",
                height: "24",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                circle { cx: "8", cy: "8", r: "6" }
                path { d: "M18.09 10.37A6 6 0 1 1 10.34 18" }
                path { d: "M7 6h1v4" }
                path { d: "m16.71 13.88.7.71-2.82 2.82" }
            }
            h3 { class: "mt-2 text-sm font-medium text-gray-900 dark:text-white",
                {t.no_token_transactions}
            }
            p { class: "mt-1 text-sm text-gray-500 dark:text-gray-400",
                {t.no_token_transactions_desc}
            }
        }
    }
}
