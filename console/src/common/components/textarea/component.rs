use dioxus::prelude::*;

const BASE_CLASS: &str = "w-full min-h-16 box-border py-2 px-3 border-none rounded-lg m-0 appearance-none bg-none text-gray-700 dark:text-gray-300 font-[inherit] leading-relaxed outline-none resize-y transition-all duration-100 disabled:text-gray-500 dark:disabled:text-gray-400 disabled:cursor-not-allowed placeholder:text-gray-500 dark:placeholder:text-gray-400";

#[derive(Copy, Clone, PartialEq, Default)]
#[non_exhaustive]
pub enum TextareaVariant {
    #[default]
    Default,
    Fade,
    Outline,
    Ghost,
}

impl TextareaVariant {
    pub fn class(&self) -> &'static str {
        match self {
            TextareaVariant::Default => "bg-white dark:bg-gray-900 shadow-[inset_0_0_0_1px] shadow-gray-200 dark:shadow-gray-700 hover:enabled:bg-gray-200 dark:hover:enabled:bg-gray-800 hover:enabled:text-gray-900 dark:hover:enabled:text-white focus:bg-gray-200 dark:focus:bg-gray-800 focus:text-gray-900 dark:focus:text-white",
            TextareaVariant::Fade => "bg-white dark:bg-gray-900 hover:enabled:bg-gray-200 dark:hover:enabled:bg-gray-800 hover:enabled:text-gray-900 dark:hover:enabled:text-white focus:bg-gray-200 dark:focus:bg-gray-800 focus:text-gray-900 dark:focus:text-white",
            TextareaVariant::Outline => "border border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 hover:enabled:border-gray-300 dark:hover:enabled:border-gray-600 focus:border-blue-500 invalid:border-red-600 aria-[invalid=true]:border-red-600",
            TextareaVariant::Ghost => "bg-transparent hover:enabled:bg-gray-100 dark:hover:enabled:bg-gray-800 hover:enabled:text-gray-900 dark:hover:enabled:text-white focus:border-blue-500",
        }
    }
}

#[component]
pub fn Textarea(
    oninput: Option<EventHandler<FormEvent>>,
    onchange: Option<EventHandler<FormEvent>>,
    oninvalid: Option<EventHandler<FormEvent>>,
    onselect: Option<EventHandler<SelectionEvent>>,
    onselectionchange: Option<EventHandler<SelectionEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
    onfocusin: Option<EventHandler<FocusEvent>>,
    onfocusout: Option<EventHandler<FocusEvent>>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onkeypress: Option<EventHandler<KeyboardEvent>>,
    onkeyup: Option<EventHandler<KeyboardEvent>>,
    oncompositionstart: Option<EventHandler<CompositionEvent>>,
    oncompositionupdate: Option<EventHandler<CompositionEvent>>,
    oncompositionend: Option<EventHandler<CompositionEvent>>,
    oncopy: Option<EventHandler<ClipboardEvent>>,
    oncut: Option<EventHandler<ClipboardEvent>>,
    onpaste: Option<EventHandler<ClipboardEvent>>,
    #[props(default)] variant: TextareaVariant,
    #[props(extends=GlobalAttributes)]
    #[props(extends=textarea)]
    attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        textarea {
            class: "{BASE_CLASS} {variant.class()}",
            "data-slot": "textarea",
            oninput: move |e| _ = oninput.map(|callback| callback(e)),
            onchange: move |e| _ = onchange.map(|callback| callback(e)),
            oninvalid: move |e| _ = oninvalid.map(|callback| callback(e)),
            onselect: move |e| _ = onselect.map(|callback| callback(e)),
            onselectionchange: move |e| _ = onselectionchange.map(|callback| callback(e)),
            onfocus: move |e| _ = onfocus.map(|callback| callback(e)),
            onblur: move |e| _ = onblur.map(|callback| callback(e)),
            onfocusin: move |e| _ = onfocusin.map(|callback| callback(e)),
            onfocusout: move |e| _ = onfocusout.map(|callback| callback(e)),
            onkeydown: move |e| _ = onkeydown.map(|callback| callback(e)),
            onkeypress: move |e| _ = onkeypress.map(|callback| callback(e)),
            onkeyup: move |e| _ = onkeyup.map(|callback| callback(e)),
            oncompositionstart: move |e| _ = oncompositionstart.map(|callback| callback(e)),
            oncompositionupdate: move |e| _ = oncompositionupdate.map(|callback| callback(e)),
            oncompositionend: move |e| _ = oncompositionend.map(|callback| callback(e)),
            oncopy: move |e| _ = oncopy.map(|callback| callback(e)),
            oncut: move |e| _ = oncut.map(|callback| callback(e)),
            onpaste: move |e| _ = onpaste.map(|callback| callback(e)),
            ..attributes,
            {children}
        }
    }
}
