use dioxus::prelude::*;

/// Three.js 3D cube canvas container.
/// The actual 3D scene is rendered by /assets/three-cube.js
#[component]
pub(super) fn HeroCubeGroup() -> Element {
    rsx! {
        div {
            id: "cube-canvas-container",
            style: "width: 100%; height: 100%;",
        }
    }
}
