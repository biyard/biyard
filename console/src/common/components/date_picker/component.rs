use dioxus::prelude::*;

use dioxus_primitives::icon;
use dioxus_primitives::{
    ContentAlign,
    date_picker::{self, DatePickerInputProps, DatePickerProps, DateRangePickerProps},
    popover::{PopoverContentProps, PopoverTriggerProps},
};

use super::super::calendar::*;
use super::super::popover::*;

const DATE_PICKER_TW: &str = "date-picker relative inline-flex items-center \
    [&_.date-picker-group]:flex [&_.date-picker-group]:w-fit [&_.date-picker-group]:min-w-[150px] \
    [&_.date-picker-group]:flex-row [&_.date-picker-group]:items-center [&_.date-picker-group]:justify-between \
    [&_.date-picker-group]:p-2 [&_.date-picker-group]:border-none [&_.date-picker-group]:rounded-lg \
    [&_.date-picker-group]:bg-white [&_.date-picker-group]:text-gray-700 [&_.date-picker-group]:gap-1 \
    [&_.date-picker-group]:shadow-[inset_0_0_0_1px_theme(colors.gray.200)] \
    [&_.date-picker-group]:transition-colors [&_.date-picker-group]:duration-100 \
    dark:[&_.date-picker-group]:text-gray-300 dark:[&_.date-picker-group]:bg-gray-900 \
    dark:[&_.date-picker-group]:shadow-[inset_0_0_0_1px_theme(colors.gray.600)] \
    [&_.date-picker-group_.popover-trigger]:inline-flex [&_.date-picker-group_.popover-trigger]:items-center \
    [&_.date-picker-group_.popover-trigger]:justify-center [&_.date-picker-group_.popover-trigger]:p-0 \
    [&_.date-picker-group_.popover-trigger]:border-none [&_.date-picker-group_.popover-trigger]:bg-transparent \
    [&_.date-picker-group_.popover-trigger]:cursor-pointer \
    [&_.date-picker-group_.popover-trigger]:transition-[rotate] \
    [&_.date-picker-group_.popover-trigger]:duration-150 \
    [&_.date-picker-group_.popover-trigger]:ease-[cubic-bezier(0.4,0,0.2,1)] \
    [&_.date-picker-group_.popover-content]:max-w-none [&_.date-picker-group_.popover-content]:p-0 \
    [&_.date-segment]:caret-transparent \
    [&_.date-segment[no-date=true]]:text-gray-500 dark:[&_.date-segment[no-date=true]]:text-gray-400 \
    [&_.date-segment[is-separator=true]]:p-0 \
    [&_.date-segment:focus-visible]:rounded [&_.date-segment:focus-visible]:bg-gray-400 \
    [&_.date-segment:focus-visible]:text-white [&_.date-segment:focus-visible]:outline-none";

#[component]
pub fn DatePicker(props: DatePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            date_picker::DatePicker {
                class: DATE_PICKER_TW,
                on_value_change: props.on_value_change,
                selected_date: props.selected_date,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DateRangePicker(props: DateRangePickerProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            date_picker::DateRangePicker {
                class: DATE_PICKER_TW,
                on_range_change: props.on_range_change,
                selected_range: props.selected_range,
                disabled: props.disabled,
                read_only: props.read_only,
                min_date: props.min_date,
                max_date: props.max_date,
                month_count: props.month_count,
                disabled_ranges: props.disabled_ranges,
                roving_loop: props.roving_loop,
                attributes: props.attributes,
                date_picker::DatePickerPopover { popover_root: PopoverRoot, {props.children} }
            }
        }
    }
}

#[component]
pub fn DatePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DatePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DatePickerCalendar { calendar: Calendar,
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DateRangePickerInput(props: DatePickerInputProps) -> Element {
    rsx! {
        date_picker::DateRangePickerInput {
            on_format_day_placeholder: props.on_format_day_placeholder,
            on_format_month_placeholder: props.on_format_month_placeholder,
            on_format_year_placeholder: props.on_format_year_placeholder,
            attributes: props.attributes,
            {props.children}
            DatePickerPopoverTrigger {}
            DatePickerPopoverContent { align: ContentAlign::Center,
                date_picker::DateRangePickerCalendar { calendar: RangeCalendar,
                    CalendarView {
                        CalendarHeader {
                            CalendarNavigation {
                                CalendarPreviousMonthButton {}
                                CalendarSelectMonth {}
                                CalendarSelectYear {}
                                CalendarNextMonthButton {}
                            }
                        }
                        CalendarGrid {}
                    }
                }
            }
        }
    }
}

#[component]
pub fn DatePickerPopoverTrigger(props: PopoverTriggerProps) -> Element {
    rsx! {
        PopoverTrigger { aria_label: "Show Calendar", attributes: props.attributes,
            icon::Icon {
                width: "20px",
                height: "20px",
                stroke: "var(--primary-color-7)",
                polyline { points: "6 9 12 15 18 9" }
            }
        }
    }
}

#[component]
pub fn DatePickerPopoverContent(props: PopoverContentProps) -> Element {
    rsx! {
        PopoverContent {
            class: "popover-content",
            id: props.id,
            side: props.side,
            align: props.align,
            attributes: props.attributes,
            {props.children}
        }
    }
}
