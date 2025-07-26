const MOBILE_BREAKPOINT: f64 = 768.0;

pub fn is_mobile_screen() -> bool {
    if let Some(window) = leptos::web_sys::window() {
        if let Ok(width) = window.inner_width() {
            if let Some(width_num) = width.as_f64() {
                return width_num < MOBILE_BREAKPOINT;
            }
        }
    }
    false
}