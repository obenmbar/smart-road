mod vehicle;
mod intersection;
mod stats;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() {
    // 1. كنشعلو الموطور ديال SDL2
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // 2. كنصايبو النافذة (Window) ديالنا، وعطيناها قياس 800x800 بيكسل
    let window = video_subsystem.window("Zone01 Smart Road Intersection", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    // 3. كنصايبو الكانفا (Canvas) لي غنبقاو نرسمو فيها
    let mut canvas = window.into_canvas().build().unwrap();

    // 4. الإيفينتات (Event Pump) باش نحضيو الكلافي والبوطونة د الخروج
    let mut event_pump = sdl_context.event_pump().unwrap();

    println!("🎨 الشاشة تحلات! برك على Echap (Escape) باش تخرج.");

    // 5. حلقة اللعبة (Game Loop) 
    'running: loop {
        // أ. كنقراو شنو دار اليوزر
        for event in event_pump.poll_iter() {
            match event {
                // يلا برك على الكروا (X) د النافذة أو برك على Escape
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running; // كنهرسو الحلقة وكنخرجو
                },
                _ => {}
            }
        }

        // ب. كنلونو الشاشة بالرمادي الغامق (لون الشانطي)
        canvas.set_draw_color(Color::RGB(40, 40, 40));
        canvas.clear(); // كنمشحو الشاشة باش نرسمو من جديد

        // ج. كنبينو داكشي لي رسمنا للمستخدم
        canvas.present();

        // د. باش الميكروبروسيسور مايتحرقش، كنقولو ليه يرتاح شوية (للحفاظ على 60 FPS)
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}