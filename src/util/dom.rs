#![allow(dead_code)]

use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Event, HtmlElement, HtmlInputElement, HtmlTextAreaElement, InputEvent, MouseEvent};
use yew::prelude::*;

/// Gets the value of an input element from an event.
pub fn input_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a textarea element from an event.
pub fn textarea_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlTextAreaElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

/// Gets the value of a content-editable element from an event.
pub fn content_editable_event_value(e: InputEvent) -> String {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlElement = event_target.dyn_into().unwrap_throw();
    target.inner_text()
}

/// Gets the value of a checkbox from a mouse click event.
pub fn checkbox_checked(e: MouseEvent) -> bool {
    let event: Event = e.dyn_into().unwrap_throw();
    let event_target = event.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.checked()
}

/// Focuses an element in the DOM.
pub fn focus_element(node: &NodeRef) {
    if let Some(node) = node.get() {
        node.dyn_ref::<HtmlElement>().unwrap().focus().unwrap();
    }
}

/// Selects the content of an element in the DOM.
pub fn select_element_content(node: &NodeRef) {
    if let Some(node) = node.get() {
        let window = web_sys::window().unwrap();

        let range = web_sys::Range::new().unwrap();
        range.select_node_contents(&node).unwrap();

        let selection = window.get_selection().unwrap().unwrap();
        selection.remove_all_ranges().unwrap();
        selection.add_range(&range).unwrap();
    }
}

/// Sets the cursor position to the end within an element in the DOM.
pub fn go_to_end(node: &NodeRef) {
    if let Some(node) = node.get() {
        let window = web_sys::window().unwrap();

        let selection = window.get_selection().unwrap().unwrap();
        selection.set_position_with_offset(Some(&node), 1).unwrap();
    }
}

/// Clears all selections.
pub fn clear_selections() {
    let window = web_sys::window().unwrap();

    let selection = window.get_selection().unwrap().unwrap();
    selection.remove_all_ranges().unwrap();
}

/// Sets the inner text of an element in the DOM.
pub fn set_inner_text(node: &NodeRef, text: &str) {
    if let Some(node) = node.get() {
        node.set_text_content(Some(text));
    }
}
