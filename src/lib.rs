mod utils;
mod tetris_game;
mod tetris_part;
mod tetris_piece;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::utils::{canvas, messsage, request_animation_frame, window};
use crate::tetris_game::{TetrisGame};
use instant::Instant;
use std::sync::{Arc, Mutex};


#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let canvas : web_sys::HtmlCanvasElement = canvas();
    canvas.set_width(600);
    canvas.set_height(1000);

    let mut i = 0;
    let mut tetris = TetrisGame::new();
    let animate_cb = Rc::new(RefCell::new(None));
    let animate_cb2 = animate_cb.clone();

    let mut last_tick_time = Instant::now();

    let left = Arc::new(Mutex::new(false));
    let left2 = Arc::clone(&left);
    let left3 = Arc::clone(&left);

    let right = Arc::new(Mutex::new(false));
    let right2 = Arc::clone(&right);
    let right3 = Arc::clone(&right);

    let down = Arc::new(Mutex::new(false));
    let down2 = Arc::clone(&down);
    let down3 = Arc::clone(&down);

    let space = Arc::new(Mutex::new(false));
    let space2 = Arc::clone(&space);

    let shift = Arc::new(Mutex::new(false));
    let shift2 = Arc::clone(&shift);
    let shift3 = Arc::clone(&shift);

    *animate_cb2.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let move_left = *left.lock().unwrap();
        let move_right = *right.lock().unwrap();
        let move_down = *down.lock().unwrap();
        let rotate = *space.lock().unwrap();
        let shift = *shift.lock().unwrap();
        if move_left {
            tetris.move_left();
            if !shift {
                *left.lock().unwrap() = false;
            }
            tetris.draw();
        } else if move_right {
            tetris.move_right();
            if !shift {
                *right.lock().unwrap() = false;
            }
            tetris.draw();
        } else if move_down {
            tetris.move_down();
            if !shift {
                *down.lock().unwrap() = false;
            }
            tetris.draw();
        } else if rotate {
            tetris.rotate();
            *space.lock().unwrap() = false;
            tetris.draw();
        } else {
            let duration = last_tick_time.elapsed();
            if duration.as_millis() > 1000 {
                tetris.tick();
                last_tick_time = Instant::now();
            }
        }

        i += 1;
        let text = format!("requestAnimationFrame has been called {} times.", i);
        messsage().set_text_content(Some(&text));
        request_animation_frame(animate_cb.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));
    request_animation_frame(animate_cb2.borrow().as_ref().unwrap());

    let key_down_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let code = event.code();

        if code == "ArrowLeft" {
            *left2.lock().unwrap() = true;
        } else if code == "ArrowRight" {
            *right2.lock().unwrap() = true;
        } else if code == "ArrowDown" {
            *down2.lock().unwrap() = true;
        } else if code == "ShiftLeft" {
            *shift2.lock().unwrap() = true;
        } else {
            log!("{}", code);
        }

    }) as Box<dyn FnMut(_)>);

    window().add_event_listener_with_callback("keydown", key_down_closure.as_ref().unchecked_ref())?;
    key_down_closure.forget();

    let key_up_closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        let code = event.code();

        if code == "ArrowLeft" {
            *left3.lock().unwrap() = false;
        } else if code == "ArrowRight" {
            *right3.lock().unwrap() = false;
        } else if code == "ArrowDown" {
            *down3.lock().unwrap() = false;
        } else if code == "Space" {
            *space2.lock().unwrap() = true;
        } else if code == "ShiftLeft" {
            *shift3.lock().unwrap() = false;
        } else {
            log!("{}", code);
        }

    }) as Box<dyn FnMut(_)>);

    window().add_event_listener_with_callback("keyup", key_up_closure.as_ref().unchecked_ref())?;
    key_up_closure.forget();

    Ok(())
}