use dioxus::prelude::*;

const INPUT_CLASS: &str = "w-full rounded-xl border border-border bg-panel px-4 py-3 text-sm font-medium text-foreground placeholder:text-foreground-muted focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand disabled:cursor-not-allowed disabled:opacity-60";
const INPUT_WITH_SUFFIX_CLASS: &str = "w-full rounded-xl border border-border bg-panel py-3 pl-4 pr-12 text-sm font-medium text-foreground placeholder:text-foreground-muted focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand disabled:cursor-not-allowed disabled:opacity-60";
const INPUT_WITH_ICON_CLASS: &str = "block w-full appearance-none rounded-xl border border-border bg-panel py-3 pr-4 pl-10 text-sm font-medium text-foreground placeholder:text-foreground-muted focus:border-brand focus:outline-none focus:ring-2 focus:ring-brand disabled:cursor-not-allowed disabled:opacity-60";

#[component]
pub fn FormLabel(children: Element) -> Element {
    rsx! {
        label { class: "mb-2 block text-sm font-semibold text-foreground-soft",
            {children}
        }
    }
}

#[component]
pub fn FormField(
    label: &'static str,
    #[props(default = "text")] r#type: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] placeholder: String,
    #[props(default)] min: &'static str,
    #[props(default)] max: &'static str,
    #[props(default)] step: &'static str,
    #[props(default)] maxlength: &'static str,
    #[props(default)] required: bool,
    #[props(default)] disabled: bool,
    /// Optional inline suffix rendered inside the input on the right
    /// (e.g., `%`, `KAIA`, `tokens`). Purely visual — doesn't affect
    /// the underlying value.
    #[props(default)]
    suffix: &'static str,
) -> Element {
    let has_suffix = !suffix.is_empty();
    rsx! {
        div {
            FormLabel { {label} }
            if has_suffix {
                div { class: "relative",
                    input {
                        r#type,
                        value,
                        oninput: move |e| oninput.call(e),
                        placeholder,
                        min,
                        max,
                        step,
                        maxlength,
                        required,
                        disabled,
                        class: INPUT_WITH_SUFFIX_CLASS,
                    }
                    span { class: "pointer-events-none absolute inset-y-0 right-3 flex items-center text-sm font-semibold text-foreground-muted",
                        {suffix}
                    }
                }
            } else {
                input {
                    r#type,
                    value,
                    oninput: move |e| oninput.call(e),
                    placeholder,
                    min,
                    max,
                    step,
                    maxlength,
                    required,
                    disabled,
                    class: INPUT_CLASS,
                }
            }
        }
    }
}

#[component]
pub fn FormFieldWithIcon(
    label: &'static str,
    id: &'static str,
    r#type: &'static str,
    value: String,
    oninput: EventHandler<FormEvent>,
    #[props(default)] placeholder: String,
    #[props(default)] autocomplete: &'static str,
    #[props(default = true)] required: bool,
    icon: Element,
) -> Element {
    rsx! {
        div {
            label {
                r#for: id,
                class: "block text-sm font-semibold text-foreground-soft",
                {label}
            }
            div { class: "relative mt-1",
                div { class: "pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3 text-foreground-muted",
                    {icon}
                }
                input {
                    id,
                    name: id,
                    r#type,
                    autocomplete,
                    required,
                    value,
                    oninput: move |e| oninput.call(e),
                    class: INPUT_WITH_ICON_CLASS,
                    placeholder,
                }
            }
        }
    }
}
