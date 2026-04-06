use dioxus::prelude::*;
use dioxus_primitives::avatar::{self, AvatarFallbackProps, AvatarImageProps, AvatarState};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarImageSize {
    #[default]
    Small,
    Medium,
    Large,
}

impl AvatarImageSize {
    fn to_class(self) -> &'static str {
        match self {
            AvatarImageSize::Small => "w-8 h-8 text-sm",
            AvatarImageSize::Medium => "w-12 h-12 text-xl",
            AvatarImageSize::Large => "w-16 h-16 text-[1.75rem]",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AvatarShape {
    #[default]
    Circle,
    Rounded,
}

impl AvatarShape {
    fn to_class(self) -> &'static str {
        match self {
            AvatarShape::Circle => "rounded-full",
            AvatarShape::Rounded => "rounded-lg",
        }
    }
}

/// The props for the [`Avatar`] component.
#[derive(Props, Clone, PartialEq)]
pub struct AvatarProps {
    /// Callback when image loads successfully
    #[props(default)]
    pub on_load: Option<EventHandler<()>>,

    /// Callback when image fails to load
    #[props(default)]
    pub on_error: Option<EventHandler<()>>,

    /// Callback when the avatar state changes
    #[props(default)]
    pub on_state_change: Option<EventHandler<AvatarState>>,

    #[props(default)]
    pub size: AvatarImageSize,

    #[props(default)]
    pub shape: AvatarShape,

    /// Additional attributes for the avatar element
    #[props(extends = GlobalAttributes)]
    pub attributes: Vec<Attribute>,

    /// The children of the Avatar component, which can include AvatarImage and AvatarFallback
    pub children: Element,
}

#[component]
pub fn Avatar(props: AvatarProps) -> Element {
    rsx! {
        avatar::Avatar {
            class: "avatar relative inline-flex overflow-hidden shrink-0 items-center justify-center text-gray-700 dark:text-gray-300 cursor-pointer font-medium {props.size.to_class()} {props.shape.to_class()} data-[state=loading]:animate-pulse data-[state=empty]:bg-gray-300 dark:data-[state=empty]:bg-gray-600 [&[data-state=error]_.avatar-fallback]:bg-gray-900 [&[data-state=error]_.avatar-fallback]:text-gray-700 dark:[&[data-state=error]_.avatar-fallback]:text-gray-300",
            on_load: props.on_load,
            on_error: props.on_error,
            on_state_change: props.on_state_change,
            attributes: props.attributes,
            {props.children}
        }
    }
}

#[component]
pub fn AvatarImage(props: AvatarImageProps) -> Element {
    rsx! {
        avatar::AvatarImage {
            class: "w-full h-full aspect-square",
            src: props.src,
            alt: props.alt,
            attributes: props.attributes,
        }
    }
}

#[component]
pub fn AvatarFallback(props: AvatarFallbackProps) -> Element {
    rsx! {
        avatar::AvatarFallback { class: "avatar-fallback flex w-full h-full items-center justify-center bg-white text-gray-700 dark:text-gray-300 text-2xl", attributes: props.attributes, {props.children} }
    }
}
