mod vehicle;
mod intersection;
mod stats;
 use sdl2::event::Event;
 use sdl2::keyboard::Keycode;
 use sdl2::pixels::Color;
 use std::time::Duration;


pub fn main() {
let  sdl_context =  sdl2::init().unwrap();
let video_subsystem = sdl_context.video().unwrap();

let window =  video_subsystem.window("Smart Road",800,800).position_centered().build().unwrap();

let mut canvas =  window.into_canvas().build().unwrap();

}