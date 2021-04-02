use std::cell::Cell;
use std::cell::RefCell;
use std::f64;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

fn get_window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    get_window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn get_document() -> web_sys::Document {
    get_window()
        .document()
        .expect("should have a document on window")
}

fn get_body() -> web_sys::HtmlElement {
    get_document().body().expect("document should have a body")
}

fn get_canvas() -> web_sys::HtmlCanvasElement {
	get_document().get_element_by_id("canvas").expect("document should have a canvas")
        .dyn_into::<web_sys::HtmlCanvasElement>().expect("canvas element should be an HtmlCanvasElement")
}

fn get_context() -> web_sys::CanvasRenderingContext2d {
	get_canvas()
		.get_context("2d")
		.expect("should find a 2d context")
		.expect("should find a 2d context")
		.dyn_into::<web_sys::CanvasRenderingContext2d>()
		.expect("context should be a CanvasRenderingContext2d")
}

fn disp_debug(s: &str) {
	get_document().get_element_by_id("debug").unwrap().set_inner_html(s);
}

#[derive(Debug)]
struct Piece {
	border: web_sys::Path2d,
	x: f64,
	y: f64,
	center_x: f64,
	center_y: f64,
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
	let selected = Rc::<Option<Piece>>::new(None);
    let pressed = Rc::new(Cell::new(false));
    let offset_x = Rc::new(Cell::new(0.0));
    let offset_y = Rc::new(Cell::new(0.0));

    fn render(offset_x: f64, offset_y: f64) {
    	let canvas = get_canvas();
    	let context = get_context();
    	let w = canvas.width() as f64;
    	let h = canvas.height() as f64;
    	context.clear_rect(0.0, 0.0, w, h);
    	context.save();
    	context.translate(offset_x + w / 2.0, offset_y + h / 2.0);
    	context.begin_path();

	    // Draw the outer circle.
	    context
	        .arc(0.0, 0.0, 50.0, 0.0, f64::consts::PI * 2.0)
	        .unwrap();

	    // Draw the mouth.
	    context.move_to(35.0, 0.0);
	    context.arc(0.0, 0.0, 35.0, 0.0, f64::consts::PI).unwrap();

	    // Draw the left eye.
	    context.move_to(-10.0, -10.0);
	    context
	        .arc(-15.0, -10.0, 5.0, 0.0, f64::consts::PI * 2.0)
	        .unwrap();

	    // Draw the right eye.
	    context.move_to(20.0, -10.0);
	    context
	        .arc(15.0, -10.0, 5.0, 0.0, f64::consts::PI * 2.0)
	        .unwrap();

	    context.stroke();
	    context.restore();

	    // disp_debug(&format!("offset: ({}, {})", offset_x, offset_y));
    }

    {
    	let window = get_window();
    	let canvas = get_canvas();
    	let mut closure = Box::new(move || {
    		let w = window.inner_width().unwrap().as_f64().unwrap() as u32;
    		let h = window.inner_height().unwrap().as_f64().unwrap() as u32;
	        canvas.set_width(w);
	        canvas.set_height(h);
	    }) as Box<dyn FnMut()>;
	    closure();
    	let closure = Closure::wrap(closure);
	    get_window().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
	    closure.forget();
    }

    {
    	let pressed = pressed.clone();
    	let offset_x = offset_x.clone();
    	let offset_y = offset_y.clone();

    	let f = Rc::new(RefCell::new(None));
	    let g = f.clone();

	    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
	    	render(offset_x.get(), offset_y.get());

	        request_animation_frame(f.borrow().as_ref().unwrap());
	    }) as Box<dyn FnMut()>));

	    request_animation_frame(g.borrow().as_ref().unwrap());
	}

    {
        let context = get_context();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            let x = event.offset_x() as f64;
            let y = event.offset_y() as f64;
            pressed.set(true);
        }) as Box<dyn FnMut(_)>);
        get_canvas().add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = get_context();
        let pressed = pressed.clone();
        let offset_x = offset_x.clone();
    	let offset_y = offset_y.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            if pressed.get() {
            	(*offset_x).set(offset_x.get() + event.movement_x() as f64);
            	(*offset_y).set(offset_y.get() + event.movement_y() as f64);
            }
        }) as Box<dyn FnMut(_)>);
        get_canvas().add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let context = get_context();
        let pressed = pressed.clone();
        let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
            pressed.set(false);
        }) as Box<dyn FnMut(_)>);
        get_canvas().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}