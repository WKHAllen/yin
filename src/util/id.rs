use js_sys::Math;

/// Generates a random ID for an element.
pub fn new_id() -> String {
    let value = Math::random().to_bits();
    let hex_value = format!("{value:x}");
    hex_value
}
