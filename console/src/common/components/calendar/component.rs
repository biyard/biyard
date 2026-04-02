use dioxus::prelude::*;
use dioxus_primitives::calendar::{
    self, CalendarDayProps, CalendarGridProps, CalendarHeaderProps, CalendarMonthTitleProps,
    CalendarNavigationProps, CalendarProps, CalendarSelectMonthProps, CalendarSelectYearProps,
    RangeCalendarProps,
};
use dioxus_primitives::icon::Icon;

// ── Direct class props (Tailwind) ──────────────────────────────────
const CALENDAR: &str = "flex flex-row rounded-lg bg-white border border-gray-200 \
    shadow-[0_2px_10px_rgba(0,0,0,0.1)] font-sans \
    dark:bg-gray-800 dark:border-gray-700 \
    data-[disabled=true]:opacity-60 data-[disabled=true]:pointer-events-none";

const NAV: &str = "relative flex items-center justify-center gap-2 pt-3 px-12 pb-1";

const NAV_BTN: &str = "absolute flex w-7 h-7 items-center justify-center rounded-lg \
    text-gray-500 cursor-pointer text-base leading-6 border border-gray-200 bg-transparent \
    dark:text-gray-400 dark:bg-gray-900 dark:border-gray-700 \
    hover:bg-gray-200 hover:text-gray-700 hover:border-gray-300 \
    dark:hover:bg-gray-700 dark:hover:text-gray-300 dark:hover:border-gray-600 \
    focus-visible:ring-2 focus-visible:ring-blue-500 \
    disabled:bg-white disabled:text-gray-400 disabled:cursor-not-allowed disabled:border-gray-100 \
    dark:disabled:bg-gray-800 dark:disabled:border-gray-800";

const MONTH_TITLE: &str = "flex w-full h-7 items-center justify-center";

const SELECT: &str = "absolute w-full h-full p-1 m-0 inset-0 opacity-0";

#[component]
pub fn Calendar(props: CalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::Calendar {
            class: CALENDAR,
            selected_date: props.selected_date,
            on_date_change: props.on_date_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn RangeCalendar(props: RangeCalendarProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        calendar::RangeCalendar {
            class: CALENDAR,
            selected_range: props.selected_range,
            on_range_change: props.on_range_change,
            on_format_weekday: props.on_format_weekday,
            on_format_month: props.on_format_month,
            view_date: props.view_date,
            today: props.today,
            on_view_change: props.on_view_change,
            disabled: props.disabled,
            first_day_of_week: props.first_day_of_week,
            min_date: props.min_date,
            max_date: props.max_date,
            month_count: props.month_count,
            disabled_ranges: props.disabled_ranges,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn CalendarView(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "flex flex-col", ..attributes, {children} }
    }
}

#[component]
pub fn CalendarHeader(props: CalendarHeaderProps) -> Element {
    rsx! {
        calendar::CalendarHeader { id: props.id, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarNavigation(props: CalendarNavigationProps) -> Element {
    rsx! {
        calendar::CalendarNavigation { class: NAV, attributes: props.attributes, {props.children} }
    }
}

#[component]
pub fn CalendarPreviousMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarPreviousMonthButton { class: "{NAV_BTN} left-3", attributes,
            Icon { width: "20px", height: "20px",
                path { d: "m15 18-6-6 6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarNextMonthButton(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    rsx! {
        calendar::CalendarNextMonthButton { class: "{NAV_BTN} right-3", attributes,
            Icon { width: "20px", height: "20px",
                path { d: "m9 18 6-6-6-6" }
            }
        }
    }
}

#[component]
pub fn CalendarSelectMonth(props: CalendarSelectMonthProps) -> Element {
    rsx! {
        calendar::CalendarSelectMonth { class: SELECT, attributes: props.attributes, DropDownIcon {} }
    }
}

#[component]
pub fn CalendarSelectYear(props: CalendarSelectYearProps) -> Element {
    rsx! {
        calendar::CalendarSelectYear { class: SELECT, attributes: props.attributes, DropDownIcon {} }
    }
}

#[component]
pub fn CalendarGrid(props: CalendarGridProps) -> Element {
    rsx! {
        calendar::CalendarGrid {
            id: props.id,
            show_week_numbers: props.show_week_numbers,
            render_day: props.render_day,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn CalendarMonthTitle(props: CalendarMonthTitleProps) -> Element {
    rsx! {
        calendar::CalendarMonthTitle { class: MONTH_TITLE, attributes: props.attributes }
    }
}

#[component]
pub fn CalendarDay(props: CalendarDayProps) -> Element {
    calendar::CalendarDay(props)
}

#[component]
fn DropDownIcon() -> Element {
    rsx! {
        Icon {
            width: "20px",
            height: "20px",
            stroke: "var(--secondary-color-4)",
            path { d: "m6 9 6 6 6-6" }
        }
    }
}
