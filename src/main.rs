use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
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
        .window("game tutorial", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("could not make a canvas");

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut field = vec![vec![false; WIDTH]; HEIGHT];

    let preset_number = 0;
    preset(preset_number, 0, 0, &mut field);

    let mut running = false;
    let mut delay = 50;

    let mut event_pump = sdl_context.event_pump()?;
    let mut left_mouse_button_down = false;
    let mut right_mouse_button_down = false;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    running = !running;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    left_mouse_button_down = true;
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    right_mouse_button_down = true;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    left_mouse_button_down = false;
                }
                Event::MouseButtonUp {
                    mouse_btn: MouseButton::Right,
                    ..
                } => {
                    right_mouse_button_down = false;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Comma),
                    ..
                } => {
                    delay = u32::max(1, delay - 10);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Period),
                    ..
                } => {
                    delay = u32::min(delay + 10, 500);
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Backspace),
                    ..
                } => {
                    field = vec![vec![false; WIDTH]; HEIGHT];
                    preset(preset_number, 0, 0, &mut field);
                }
                _ => {}
            }
        }
        let state = event_pump.mouse_state();
        let x = i32::min(i32::max(0, state.x()), WIDTH as i32 - 1) as usize / (CELLSIZE);
        let y = i32::min(i32::max(0, state.y()), HEIGHT as i32 - 1) as usize / (CELLSIZE);
        if left_mouse_button_down && !right_mouse_button_down {
            field[y][x] = true;
        } else if !left_mouse_button_down && right_mouse_button_down {
            field[y][x] = false;
        }

        if running {
            field = game_step(&mut field);
        }
        visualize_field(&mut canvas, &mut field);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / delay));
    }

    Ok(())
}

fn visualize_field(canvas: &mut WindowCanvas, field: &mut Vec<Vec<bool>>) {
    for y in 0..HEIGHT / CELLSIZE {
        for x in 0..WIDTH / CELLSIZE {
            if field[y][x] {
                canvas.set_draw_color(Color::RGB(0, 0xff, 0));
            } else {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
            }
            canvas.fill_rect(Rect::new(
                (x * CELLSIZE) as i32,
                (y * CELLSIZE) as i32,
                (CELLSIZE - 1) as u32,
                (CELLSIZE - 1) as u32,
            ));
        }
    }
    canvas.present();
}

fn game_step(field: &mut Vec<Vec<bool>>) -> Vec<Vec<bool>> {
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

    return new_field;

    // canvas.fill_rect(rect)
    // canvas.clear();
    // canvas.present();
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

fn set(x: usize, y: usize, value: bool, field: &mut Vec<Vec<bool>>) -> bool {
    if 0 < x && x < field[0].len() && 0 < y && y < field.len() {
        field[y][x] = value;
        return true;
    }
    return false;
}

fn preset(n: u32, x: usize, y: usize, field: &mut Vec<Vec<bool>>) {
    match n {
        0 => {
            // Glider gun

            // Left Block
            set(x + 1, y + 5, true, field);
            set(x + 2, y + 5, true, field);
            set(x + 1, y + 6, true, field);
            set(x + 2, y + 6, true, field);

            // Right Block
            set(x + 35, y + 3, true, field);
            set(x + 36, y + 3, true, field);
            set(x + 35, y + 4, true, field);
            set(x + 36, y + 4, true, field);

            // Left Part
            set(x + 11, y + 5, true, field);
            set(x + 11, y + 6, true, field);
            set(x + 11, y + 7, true, field);
            set(x + 12, y + 4, true, field);
            set(x + 12, y + 8, true, field);
            set(x + 13, y + 3, true, field);
            set(x + 13, y + 9, true, field);
            set(x + 14, y + 3, true, field);
            set(x + 14, y + 9, true, field);
            set(x + 15, y + 6, true, field);
            set(x + 16, y + 4, true, field);
            set(x + 16, y + 8, true, field);
            set(x + 17, y + 5, true, field);
            set(x + 17, y + 6, true, field);
            set(x + 17, y + 7, true, field);
            set(x + 18, y + 6, true, field);

            //Right Part
            set(x + 21, y + 3, true, field);
            set(x + 21, y + 4, true, field);
            set(x + 21, y + 5, true, field);
            set(x + 22, y + 3, true, field);
            set(x + 22, y + 4, true, field);
            set(x + 22, y + 5, true, field);
            set(x + 23, y + 2, true, field);
            set(x + 23, y + 6, true, field);
            set(x + 25, y + 2, true, field);
            set(x + 25, y + 6, true, field);
            set(x + 25, y + 1, true, field);
            set(x + 25, y + 7, true, field);
        }
        _ => {}
    }
}
