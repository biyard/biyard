use dioxus::dioxus_core::{Attribute, AttributeValue};

/// Merges default CSS classes into an attributes vector.
///
/// If a `class` attribute already exists, prepends `default_classes` to its value.
/// Otherwise, pushes a new `class` attribute with `default_classes`.
pub fn setup_class_attribute(attributes: &mut Vec<Attribute>, default_classes: &str) {
    if let Some(class_attr) = attributes.iter_mut().find(|a| a.name == "class") {
        if let AttributeValue::Text(ref mut value) = class_attr.value {
            *value = format!("{default_classes} {value}");
        }
    } else {
        attributes.push(Attribute::new("class", default_classes, None, true));
    }
}
