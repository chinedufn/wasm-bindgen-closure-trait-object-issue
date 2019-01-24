extern crate wasm_bindgen_test;
extern crate web_sys;
use wasm_bindgen_test::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn closure_lifetime_issue_this_works() {
    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = Rc::clone(&clicked);

    {
        let handler = move || {
            clicked_clone.set(true);
        };
        let handler = Closure::wrap(Box::new(handler) as Box<Fn()>);
        let handler: Box<dyn AsRef<JsValue>> = Box::new(handler);

        let click_event = Event::new("click").unwrap();

        let document = web_sys::window().unwrap().document().unwrap();

        let button = document.create_element("button").unwrap();
        button.add_event_listener_with_callback("click", handler.as_ref().as_ref().unchecked_ref());

        let button_target: &EventTarget = button.dyn_ref().unwrap();
        button_target.dispatch_event(&click_event).unwrap();
    }

    assert_eq!(*clicked, Cell::new(true), "This works!");
}

struct ClosureWrapper(pub Box<dyn AsRef<JsValue>>);

#[wasm_bindgen_test]
fn closure_lifetime_issue_this_does_not_work() {
    let mut closure_wrapper: Option<ClosureWrapper> = None;

    let clicked = Rc::new(Cell::new(false));
    let clicked_clone = Rc::clone(&clicked);

    {
        let document = web_sys::window().unwrap().document().unwrap();
        let button = document.create_element("button").unwrap();

        let handler = move || {
            clicked_clone.set(true);
        };
        let handler = Closure::wrap(Box::new(handler) as Box<Fn()>);
        let handler: Box<dyn AsRef<JsValue>> = Box::new(handler);

        // Comment this in and comment out the below for tests to pass
//        button.add_event_listener_with_callback(
//            "click",
//            handler.as_ref().as_ref().unchecked_ref(),
//        );

        // Comment this closure_wrapper and button.add_event_listener
        // out and comment in the above for tests to pass
        closure_wrapper = Some(ClosureWrapper(handler));
        button.add_event_listener_with_callback(
            "click",
            closure_wrapper.unwrap().0.as_ref().as_ref().unchecked_ref(),
        );

        let click_event = Event::new("click").unwrap();

        let button_target: &EventTarget = button.dyn_ref().unwrap();
        button_target.dispatch_event(&click_event).unwrap();
    }

    assert_eq!(*clicked, Cell::new(true), "This test fails");
}

