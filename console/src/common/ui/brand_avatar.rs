use dioxus::prelude::*;

/// Visual size variants for `BrandAvatar`.
///
/// Picked from the actual sizes already in use across the console
/// (dashboard tile, list card, detail header, overview tab, editor
/// preview). Keeping them as named variants prevents a new magic
/// number every time we drop an avatar onto a new page.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
pub enum BrandAvatarSize {
    /// 44px — dashboard "recent brands" row, tight list rows.
    Sm,
    /// 64px — most section headers (project detail, overview tab).
    #[default]
    Md,
    /// 72px — editor live preview.
    Lg,
    /// 80px — primary brand cards in the brand list.
    Xl,
}

impl BrandAvatarSize {
    fn box_class(self) -> &'static str {
        match self {
            Self::Sm => "h-11 w-11 rounded-2xl",
            Self::Md => "h-16 w-16 rounded-[20px]",
            Self::Lg => "h-[4.5rem] w-[4.5rem] rounded-[22px]",
            Self::Xl => "h-20 w-20 rounded-[24px]",
        }
    }

    fn initial_text_class(self) -> &'static str {
        match self {
            Self::Sm => "font-display text-sm font-bold",
            Self::Md => "font-display text-2xl font-bold",
            Self::Lg => "font-display text-3xl font-bold",
            Self::Xl => "font-display text-3xl font-bold",
        }
    }
}

/// Brand visual identity block used wherever a Project/Brand is
/// referenced in the UI: lists, section headers, live editor preview.
///
/// - If `logo_url` is `Some` and non-empty → renders the uploaded logo
///   with a subtle border and `object-cover` cropping.
/// - Otherwise → renders the first character of `name` on the brand
///   soft background as a fallback "initial tile".
///
/// Prefer this component over hand-written `if let Some(logo) { img }
/// else { div }` blocks so that every surface stays visually
/// consistent and future changes (e.g. dark-mode border, hover
/// treatment) land in one place.
#[component]
pub fn BrandAvatar(
    name: String,
    #[props(default)] logo_url: Option<String>,
    #[props(default)] size: BrandAvatarSize,
) -> Element {
    let trimmed = logo_url
        .as_ref()
        .map(|u| u.trim())
        .filter(|u| !u.is_empty())
        .map(|u| u.to_string());

    let initial = name
        .chars()
        .next()
        .unwrap_or('B')
        .to_uppercase()
        .next()
        .unwrap_or('B');

    let box_class = size.box_class();
    let text_class = size.initial_text_class();

    rsx! {
        if let Some(url) = trimmed {
            img {
                src: "{url}",
                alt: "{name}",
                class: "{box_class} shrink-0 border border-border object-cover bg-panel",
            }
        } else {
            div {
                class: "{box_class} shrink-0 flex items-center justify-center bg-brand-soft text-brand {text_class}",
                "{initial}"
            }
        }
    }
}
