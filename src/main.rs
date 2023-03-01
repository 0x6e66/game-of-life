use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, self};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use std::time::Duration;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const CELLSIZE: usize = 10;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("game tutorial", 800, 600)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    // let mut field = [[true; WIDTH]; HEIGHT];
    let mut field = vec![vec![false; WIDTH]; HEIGHT];
    // field[1][1] = true;
    // field[2][1] = true;
    // field[2][2] = true;
    // field[2][3] = true;

    let mut started = false;

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), .. } => {
                    started = !started;
                },
                Event::MouseButtonDown { mouse_btn: MouseButton::Left, .. } => {
                    mouse::Cursor::from_surface(surface, hot_x, hot_y)
                },
                _ => {}
            }
        }
        if started {
            game_step(&mut canvas, &mut field);
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}

fn game_step(canvas: &mut WindowCanvas, field: &mut Vec<Vec<bool>>) {
    let mut new_field: Vec<Vec<bool>> = vec![vec![false; WIDTH]; HEIGHT];
    for y in 0..HEIGHT / CELLSIZE {
        for x in 0..WIDTH / CELLSIZE {
            let num_of_alive_cells = get_num_of_alive_cells(field, x, y);
            // println!("{}", num_of_alive_cells);
            // Any live cell with fewer than two live neighbours dies, as if by underpopulation.
            if field[y][x] && num_of_alive_cells < 2 {
                new_field[y][x] = false;
            }
            // Any live cell with two or three live neighbours lives on to the next generation.
            else if field[y][x] && (num_of_alive_cells == 2 || num_of_alive_cells == 3) {
                new_field[y][x] = true;
            }
            // Any live cell with more than three live neighbours dies, as if by overpopulation.
            else if field[y][x] && num_of_alive_cells > 3 {
                new_field[y][x] = false;
            }
            // Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
            else if !field[y][x] && num_of_alive_cells == 3 {
                new_field[y][x] = true;
            }
        }
    }

    
    for y in 0..HEIGHT / CELLSIZE {
        for x in 0..WIDTH / CELLSIZE {
            field[y][x] = new_field[y][x];
            
            if field[y][x]{
                canvas.set_draw_color(Color::RGB(0, 0xff, 0));
            }else{
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.fill_rect(Rect::new((x*CELLSIZE + x) as i32, (y*CELLSIZE + y) as i32, CELLSIZE as u32, CELLSIZE as u32));
        }
    }

    // canvas.fill_rect(rect)
    // canvas.clear();
    canvas.present();
}

fn get_num_of_alive_cells(field: &mut Vec<Vec<bool>>, x: usize, y: usize) -> u32 {
    let mut num_of_alive_cells: u32 = 0;
    if x > 0 {
        num_of_alive_cells += field[y][x - 1] as u32;
    }
    if x > 0 && y > 0 {
        num_of_alive_cells += field[y - 1][x - 1] as u32;
    }
    if y > 0 {
        num_of_alive_cells += field[y - 1][x] as u32;
    }
    if x < field[0].len() - 1 && y > 0 {
        num_of_alive_cells += field[y - 1][x + 1] as u32;
    }
    if x < field[0].len() - 1 {
        num_of_alive_cells += field[y][x + 1] as u32;
    }
    if x < field[0].len() - 1 && y < field.len() - 1 {
        num_of_alive_cells += field[y + 1][x + 1] as u32;
    }
    if y < field.len() - 1 {
        num_of_alive_cells += field[y + 1][x] as u32;
    }
    if x > 0 && y < field.len() - 1 {
        num_of_alive_cells += field[y + 1][x - 1] as u32;
    }
    return num_of_alive_cells;
}
