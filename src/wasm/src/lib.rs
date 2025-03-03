use std::{cell::RefCell, rc::Rc};
use std::f64;
use async_trait::async_trait;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::*;
use rand::prelude::*;
use web_sys::{HtmlCanvasElement, MouseEvent, CanvasRenderingContext2d, window,};

macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

// main : Wasm Access Point

#[wasm_bindgen]
pub fn main() -> Result<(), JsValue>{
    console_error_panic_hook::set_once();

    wasm_bindgen_futures::spawn_local(async move {
        let document = window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas").unwrap();
        let canvas:HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();
        let game = Game::new(canvas);
        GameLoop::start(game)
            .await
            .expect("Start Game");
    });
    Ok(())
}

// Static Game Trait

#[async_trait(?Send)]
pub trait StaticGame {
    fn new (canvas: HtmlCanvasElement) -> Self;
    fn get_canvas(&mut self) -> HtmlCanvasElement;
    fn on_animation_frame(&mut self);
    fn set_click(&mut self, _x: i32, _y: i32);
    fn update(&mut self);
    fn draw(&self);
    fn clear(&self);
}

// Game Loop

struct GameLoop;
impl GameLoop {
    pub async fn start(mut game: impl StaticGame + 'static) -> Result<(), String>{

        log!("START");
        let _canvas = game.get_canvas();
        let closure = Rc::new(RefCell::new(None));
        let closure_cloned = Rc::clone(&closure);

        let ref_game = Rc::new(RefCell::new(game));
        let ref_game_clone = ref_game.clone();

        let mut frame = 0;

        closure_cloned.replace(Some(Closure::wrap(Box::new(move |_time: f64| {
            frame += 1;
            if frame % 5 == 0 {
                ref_game.borrow_mut().on_animation_frame();
            }
            request_animation_frame(closure.borrow().as_ref().unwrap());
        }) as Box<dyn FnMut(f64)>)));
        request_animation_frame(closure_cloned.borrow().as_ref().unwrap());

        let c = Closure::wrap(Box::new(move |e:MouseEvent| {
            ref_game_clone.borrow_mut().set_click(e.client_x(), e.client_y());
        }) as Box<dyn FnMut(_)>);
        _canvas.add_event_listener_with_callback(
            "mousedown",
            c.as_ref().unchecked_ref(),
        ).unwrap();
        c.forget();

        Ok(())
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut(f64)>) {
    window()
        .unwrap()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

// Constant Value for Game Object

const BASE_WIDTH:i32 = 500;       // Canvas Width
const BASE_HEIGHT:i32 = 600;      // Canvas Height
const MAX_NUMBER:usize = 10;       // Maximum Create Object
const GOAL:i32 = 99;              // Goal Gain Point
const MAX_SELECT_NUMBER:i32 = 10; // Maximum Create Object
const MAX_COLOR:i32 = 4;          // Select Colors
const INCREASE_STEP:i32 = 8;      // Movement pixcel
const TEXT_COLOR: &str = "rgb(0 255 255)";
fn get_color(c: i32) -> &'static str {
    match c {
        0 => "rgb(0 128 0)",     // Default
        1 => "rgb(24 255 0)",    //GREEN
        2 => "rgb(131 245 44)",  // Light Green
        3 => "rgb(255 255 0)",   // Light Yellow
        _ => "rgba(0 128 0)",    // Default
    }
}

// Game Object

#[derive(Debug, Clone)]
struct Game{
    canvas: HtmlCanvasElement,
    numbers: Vec<(i32,i32,i32,i32,i32,i32,i32)>, //x, y, w, h, number, color, direction
    screen_width: i32,
    screen_height:i32,
    click_x: i32,
    click_y: i32,
    gain: i32, // We can clear the game with 99 points
    status: bool, // true: start, false: finish
}
impl StaticGame for Game{

    // init
 
    fn new(canvas: HtmlCanvasElement) -> Self{
        let _screen_width = canvas.client_width().into();
        let _screen_height = canvas.client_height().into();
        let mut _numbers:Vec<(i32, i32, i32, i32, i32, i32, i32)> = Vec::with_capacity(10);
        _numbers.push((100, 100, 100, 100, 1, 0, 0));
        Game {
            canvas: canvas,
            numbers: _numbers,
            screen_width: _screen_width,
            screen_height: _screen_height,
            click_x: 0,
            click_y: 0,
            gain: 0,
            status: true,
        }
    }

    // get canvas

    fn get_canvas(&mut self) -> HtmlCanvasElement{
        self.canvas.clone()
    }

    // callback animation

    fn on_animation_frame(&mut self) {
        if self.status { self.update();}
        self.clear();
        self.draw();
    }

    // callback click

    fn set_click(&mut self, _x: i32, _y: i32) {
        let _click_x = (_x - self.canvas.offset_left()) * BASE_WIDTH / self.screen_width;
        let _click_y = (_y - self.canvas.offset_top()) * BASE_HEIGHT / self.screen_height;
        self.click_x = _click_x;
        self.click_y = _click_y;

        // restart
        /*
        if !self.status {
            let mut _numbers:Vec<(i32, i32, i32, i32, i32, i32, i32)> = Vec::with_capacity(10);
            _numbers.push((100, 100, 100, 100, 1, 0, 0));
            self.numbers = _numbers;
            self.gain = 0;
            self.status = true;
        }
        */
    }

    // game controller

    fn update(&mut self){
        let _numbers = self.numbers.clone();
        let mut _gain:i32 = 0;
        self.numbers = _numbers.iter()
            .filter_map(|x| {
                if x.0 > BASE_WIDTH { return None; }
                if x.0 + x.2 < 0 { return None; }
                if self.click_x < x.0 || self.click_x > x.0 + x.2 || self.click_y < x.1 || self.click_y > x.1 + x.3  {
                    match x.6 {
                        0 => Some((x.0 + INCREASE_STEP , x.1, x.2, x.3, x.4, x.5, x.6)),
                        _ => Some((x.0 - INCREASE_STEP , x.1, x.2, x.3, x.4, x.5, x.6)),
                    }
                } else {
                    _gain += x.4;
                    None
                }
            })
            .collect();
        self.gain += _gain;
        if self.numbers.len() < MAX_NUMBER {
            let mut rnd = rand::thread_rng();
            let _x: i32 = rnd.gen_range(1..BASE_WIDTH-100);
            let _y: i32 = rnd.gen_range(1..BASE_HEIGHT-100);
            let _w: i32 = rnd.gen_range(50..400);
            let _h: i32 = rnd.gen_range(50..400);
            let _n: i32 = rnd.gen_range(1..MAX_SELECT_NUMBER);
            let _c: i32 = rnd.gen_range(0..MAX_COLOR);
            let _d: i32 = rnd.gen_range(0..2);
            self.numbers.push((_x, _y, _w, _h, _n, _c, _d));
        }

        // game clear

        if self.gain == GOAL {
            self.status = false;
        }

        if self.gain > GOAL {
            self.gain -= GOAL;
        }
    }

    // draw

    fn draw(&self){
        let _context = self.canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();

        // rect

        for t in &self.numbers {
            let _color:&str = get_color(t.4);
            _context.set_fill_style_str(_color);
            _context.set_global_alpha(0.5);
            _context.begin_path();
            _context.rect( 
                t.0.into(),
                t.1.into(),
                t.2.into(),
                t.3.into(),
            );
            _context.close_path();
            _context.fill();

            _context.set_fill_style_str(TEXT_COLOR);
            _context.set_global_alpha(0.5);
            _context.set_text_align("center");
            _context.set_font("60px, Arial");
            let _ = _context.fill_text(
                &t.4.to_string(),
                (t.0 + t.2/2) as f64,
                (t.1 + t.3/2) as f64,
            );
        }

        // message

        let mut _message = "Click Circle to reach 99";
        if !&self.status { _message = "Congratuation!!"}
        _context.set_fill_style_str(TEXT_COLOR);
        _context.set_global_alpha(1.0);
        _context.set_text_align("left");
        _context.set_font("150px, Arial");
        _context.fill_text(
            &format!("{} / {}  {}", self.gain.to_string(), GOAL, _message),
            50.0,
            50.0,
        ).unwrap();
    }

    // screen clear

    fn clear(&self){
        let _context = self.canvas.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        _context.clear_rect(
            0.0,
            0.0,
            self.screen_width as f64,
            self.screen_height as f64,
        );
    }
}