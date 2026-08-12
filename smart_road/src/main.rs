mod intersection;
mod stats;
mod vehicle;

use intersection::Intersection;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::surface::Surface;
use sdl2::messagebox::{show_simple_message_box, MessageBoxFlag};
use std::time::{Duration, Instant};
use vehicle::{Direction, Vehicle};
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Smart Road", 800, 800)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut intersection = Intersection::new();
    let mut last_frame_time = Instant::now();
    let mut car_id = 1;
    

    // صانع الأنسجة
    let texture_creator = canvas.texture_creator();

    let car_surface = Surface::load_bmp("assets/car.bmp").unwrap();
    let car_texture = texture_creator.create_texture_from_surface(&car_surface).unwrap();

    let surface = Surface::load_bmp("assets/grass.bmp").unwrap();

    let grass_texture = texture_creator.create_texture_from_surface(&surface).unwrap();

    'running: loop {
        let current_time = Instant::now();
        let delta_time = current_time.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = current_time;
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    let stats = &intersection.stats;
                    
                    let min_v = if stats.min_speed == std::f32::MAX { 0.0 } else { stats.min_speed };
                    let min_t = if stats.min_time == std::f32::MAX { 0.0 } else { stats.min_time };

                    let msg = format!(
                        "Simulation Statistics:\n\n\
                        - Total Vehicles Passed: {}\n\
                        - Max Velocity: {:.2} px/s\n\
                        - Min Velocity: {:.2} px/s\n\
                        - Max Time Taken: {:.2} s\n\
                        - Min Time Taken: {:.2} s\n\
                        - Close Calls Avoided: {}",
                        stats.total_vehicles_crossed,
                        stats.max_speed,
                        min_v,
                        stats.max_time,
                        min_t,
                        stats.near_misses,
                    );

                    show_simple_message_box(
                        MessageBoxFlag::INFORMATION,
                        "Smart Road - Results",
                        &msg,
                        canvas.window()
                    ).unwrap();

                    break 'running;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    let lane = match rng.gen_range(0..3) {
                        0 => 's',
                        1 => 'l',
                        _ => 'r',
                    };
                    let new_car = Vehicle::new(car_id, 800.0, 10.0, lane, Direction::South);
                    intersection.add_vehicle(new_car);
                    car_id += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    let lane = match rng.gen_range(0..3) {
                        0 => 's',
                        1 => 'l',
                        _ => 'r',
                    };
                    let new_car = Vehicle::new(car_id, 840.0, 10.0, lane, Direction::North);
                    intersection.add_vehicle(new_car);
                    car_id += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Right),
                    ..
                } => {
                    let lane = match rng.gen_range(0..3) {
                        0 => 's',
                        1 => 'l',
                        _ => 'r',
                    };
                    let new_car = Vehicle::new(car_id, 840.0, 10.0, lane, Direction::West);
                    intersection.add_vehicle(new_car);
                    car_id += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Left),
                    ..
                } => {
                    let lane = match rng.gen_range(0..3) {
                        0 => 's',
                        1 => 'l',
                        _ => 'r',
                    };
                    let new_car = Vehicle::new(car_id, 800.0, 10.0, lane, Direction::East);
                    intersection.add_vehicle(new_car);
                    car_id += 1;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => {
                    let random_direction = match rng.gen_range(0..4) {
                        0 => Direction::North,
                        1 => Direction::South,
                        2 => Direction::East,
                        _ => Direction::West,
                    };

                    let random_lane = match rng.gen_range(0..3) {
                        0 => 's',
                        1 => 'l',
                        _ => 'r',
                    };
                    let mut distance = 800.0;
                    if random_direction == Direction::North || random_direction ==  Direction::West {
                        distance = 840.0;
                    }
                    let new_car = Vehicle::new(car_id, distance, 10.0, random_lane, random_direction);
                    intersection.add_vehicle(new_car);
                    car_id += 1;
                }

                _ => {}
            }
        }
        intersection.update(delta_time);
   canvas.copy(&grass_texture, None, None).unwrap();

        canvas.set_draw_color(Color::RGB(80, 80, 80));
        canvas.fill_rect(Rect::new(300, 0, 200, 800)).unwrap();

        canvas.fill_rect(Rect::new(0, 300, 800, 200)).unwrap();

        canvas.set_draw_color(Color::RGB(50, 50, 50));
        canvas.fill_rect(Rect::new(300, 300, 200, 200)).unwrap();

        canvas.set_draw_color(Color::RGB(255, 204, 0)); // صفر

        canvas.fill_rect(Rect::new(399, 0, 2, 300)).unwrap();
        canvas.fill_rect(Rect::new(399, 500, 2, 300)).unwrap();

        canvas.fill_rect(Rect::new(0, 399, 300, 2)).unwrap();
        canvas.fill_rect(Rect::new(500, 399, 300, 2)).unwrap();

        canvas.set_draw_color(Color::RGB(255, 255, 255));

        for x in [300,333, 366, 433, 466, 500].iter() {
            canvas.fill_rect(Rect::new(*x, 0, 2, 301)).unwrap();
            canvas.fill_rect(Rect::new(*x, 500, 2, 301)).unwrap();
        }

        for y in [300,333, 366, 433, 466,500].iter() {
            canvas.fill_rect(Rect::new(0, *y, 301, 2)).unwrap();
            canvas.fill_rect(Rect::new(500, *y, 301, 2)).unwrap();
        }

        for car in &intersection.vehicles {
            let mut x = 0.0;
            let mut y = 0.0;

            let mut rect_w = 24.0;
            let mut rect_h = 40.0;
            
            let mut angle = 0.0; 

            match car.direction {
                Direction::South => { 
                    let base_x: f32 = match car.lane { 'l' => 405.0, 's' => 438.0, _ => 471.0 };
                    
                    if car.has_turned() {
                        if car.lane == 'r' {
                            y = 471.0; 
                            x = base_x + (471.0 - car.distance);
                            rect_w = 40.0; rect_h = 24.0;
                            angle = 90.0;
                        } else if car.lane == 'l' {
                            y = 371.0; 
                            x = base_x - (371.0 - car.distance); 
                            rect_w = 40.0; rect_h = 24.0;
                            angle = 270.0;
                        }
                    } else {
                        x = base_x;
                        y = car.distance;
                        angle = 0.0;
                    }
                },

                Direction::North => { 
                    let base_x = match car.lane { 'l' => 371.0, 's' => 338.0, _ => 305.0 };
                    
                    if car.has_turned() {
                        if car.lane == 'r' {
                            y = 305.0; 
                            x = base_x - (495.0 - car.distance); 
                            rect_w = 40.0; rect_h = 24.0;
                            angle = 270.0; 
                        } else if car.lane == 'l' {
                            y = 405.0; 
                            x = base_x + (395.0 - car.distance); 
                            rect_w = 40.0; rect_h = 24.0;
                            angle = 90.0; 
                        }
                    } else {
                        x = base_x;
                        y = 800.0 - car.distance;
                        angle = 180.0; 
                    }
                },

                Direction::East => { 
                    let base_y = match car.lane { 'l' => 371.0, 's' => 338.0, _ => 305.0 };
                    
                    if car.has_turned() {
                        if car.lane == 'r' {
                            x = 471.0; 
                            y = base_y - (471.0 - car.distance);
                            rect_w = 24.0; rect_h = 40.0;
                            angle = 0.0; 
                        } else if car.lane == 'l' {
                            x = 371.0;
                            y = base_y + (371.0 - car.distance); 
                            rect_w = 24.0; rect_h = 40.0;
                            angle = 180.0; 
                        }
                    } else {
                        x = car.distance;
                        y = base_y;
                        rect_w = 40.0; rect_h = 24.0;
                        angle = 270.0;
                    }
                },

                Direction::West => {
                    let base_y = match car.lane { 'l' => 405.0, 's' => 438.0, _ => 471.0 };
                    
                    if car.has_turned() {
                        if car.lane == 'r' {
                            x = 305.0; 
                            y = base_y + (495.0 - car.distance); 
                            rect_w = 24.0; rect_h = 40.0;
                            angle = 180.0; 
                        } else if car.lane == 'l' {
                            x = 405.0; 
                            y = base_y - (395.0 - car.distance); 
                            rect_w = 24.0; rect_h = 40.0;
                            angle = 0.0;
                        }
                    } else {
                        x = 800.0 - car.distance;
                        y = base_y;
                        rect_w = 40.0; rect_h = 24.0;
                        angle = 90.0; 
                    }
                }
            }
            
            
            let center_x = x + (rect_w / 2.0);
            let center_y = y + (rect_h / 2.0);

    
            let sprite_rect = Rect::new(
                (center_x - 12.0) as i32, 
                (center_y - 20.0) as i32, 
                24, 
                40
            );

            canvas.copy_ex(
                &car_texture, 
                None, 
                Some(sprite_rect), 
                angle, 
                None, 
                false, 
                false
            ).unwrap();
        }
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
