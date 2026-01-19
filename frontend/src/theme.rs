use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::JsCast; // <-- for dyn_into
use web_sys::window;

#[derive(Deserialize, Debug)]
pub struct Shades {
    #[serde(rename = "50")]
    pub _50: String,
    #[serde(rename = "100")]
    pub _100: String,
    #[serde(rename = "200")]
    pub _200: String,
    #[serde(rename = "300")]
    pub _300: String,
    #[serde(rename = "400")]
    pub _400: String,
    #[serde(rename = "500")]
    pub _500: String,
    #[serde(rename = "600")]
    pub _600: String,
    #[serde(rename = "700")]
    pub _700: String,
    #[serde(rename = "800")]
    pub _800: String,
    #[serde(rename = "900")]
    pub _900: String,
    #[serde(rename = "950")]
    pub _950: String,
}

#[derive(Deserialize, Debug)]
pub struct Palette(pub HashMap<String, Shades>);

/// Load the palette from colors.json at compile time and apply as CSS variables
pub fn apply_palette() {
    let palette_str = include_str!("../design/colors.json");
    let palette: Palette = serde_json::from_str(palette_str).expect("Invalid colors.json");

    let document = window().unwrap().document().unwrap();
    let root = document
        .document_element()
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>() // <-- cast to HtmlElement
        .unwrap();

    for (name, shades) in palette.0.iter() {
        root.style()
            .set_property(&format!("--{}-50", name), &shades._50)
            .unwrap();
        root.style()
            .set_property(&format!("--{}-500", name), &shades._500)
            .unwrap();
        root.style()
            .set_property(&format!("--{}-900", name), &shades._900)
            .unwrap();
    }
}
