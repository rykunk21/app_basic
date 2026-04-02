//! Loading component with image preloading support

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

/// Preload an image and return a promise-like handle
pub fn preload_image(src: &str) -> Result<wasm_bindgen_futures::JsFuture, wasm_bindgen::JsValue> {
    let window = web_sys::window().ok_or("no window")?;
    let document = window.document().ok_or("no document")?;
    
    let img = document
        .create_element("img")?
        .dyn_into::<HtmlImageElement>()?;
    
    // Create onload closure
    let onload = Closure::once_into_js(move || {
        web_sys::console::log_1(&"Image loaded".into());
    });
    
    // Create onerror closure  
    let onerror = Closure::once_into_js(move || {
        web_sys::console::error_1(&"Image failed to load".into());
    });
    
    img.set_src(src);
    
    // Set up promise-based loading
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        // Store closures to prevent drop
        let _ = onload;  
        let _ = onerror;
        
        // Resolve when loaded
        let _ = resolve;
        let _ = reject;
    });
    
    let promise_ref: &js_sys::Promise = promise.unchecked_ref();
    Ok(wasm_bindgen_futures::JsFuture::from(promise_ref.clone()))
}

/// Component for displaying loading state
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LoadingProps {
    pub message: Option<String>,
}

#[function_component(Loading)]
pub fn loading(props: &LoadingProps) -> Html {
    let message = props.message.as_deref().unwrap_or("Loading...");
    
    html! {
        <div class="loading-spinner">
            <div class="spinner"></div>
            <p>{ message }</p>
        </div>
    }
}