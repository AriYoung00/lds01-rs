mod lidar_scanner;

extern crate sdl2;

use core::ops::Mul;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 831, 831)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();


    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut scanner = lidar_scanner::LidarScanner::new("/dev/ttyUSB0", 230_400);
    let mut pts = Vec::new();

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        let r_max = 400_i32;
        let mid = 416;
        let i_data = scanner.poll();

        for (i, pt) in i_data.iter().enumerate() {
            let pt = *pt;

            if pt > 0 {
                let cos_i = (i as f32).to_radians().cos();
                let sin_i = (i as f32).to_radians().sin();
                let rad = (pt as f32) / 4000.0 * (r_max as f32);

                let x = (cos_i * rad) as i32 + mid;
                let y = (sin_i * rad) as i32 + mid;

                pts.push(sdl2::rect::Rect::from_center(
                    sdl2::rect::Point::new(x, y),
                    2, 2
                ));
            }
        }
        if pts.len() > 361 {
            pts.drain(0..(pts.len() - 361));
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.draw_rects(&pts[..]);
        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.draw_rect(sdl2::rect::Rect::new(mid, mid, 10, 10));

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {keycode: Some(Keycode::A), ..} => {
                    println!("asdf");
                    pts.clear();
                }
                _ => {}
            }
        }

        canvas.present();
    }
    scanner.drop()
}