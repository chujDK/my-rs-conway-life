#![windows_subsystem = "windows"]

use nannou::prelude::*;
use nannou::winit;
use nannou_egui::{self, egui, Egui};

mod cells;
use cells::Cells;

use crate::cells::CellState;

struct LifeGameModel {
    cells: Cells,
    running: bool,

    egui: Egui,
    last_cursor_position: Vec2,
    window_padding: f32,
    pre_generation_time: u64,
}

fn raw_window_event(app: &App, state: &mut LifeGameModel, event: &winit::event::WindowEvent) {
    let win = app.window_rect();
    let nannou_event = nannou::event::WindowEvent::from_winit_window_event(
        event,
        win.w() as f64,
        win.h() as f64,
        app.window(app.window_id()).unwrap().scale_factor() as f64,
    );
    if let Some(nannou_event) = nannou_event {
        match nannou_event {
            nannou::event::WindowEvent::MouseReleased(button)
                if nannou::state::mouse::Button::Right == button =>
            {
                if !state.running {
                    let win_p = win.pad(state.window_padding);
                    let (w, h) = win_p.w_h();
                    let (left, top) = (-w / 2., h / 2.);
                    let w_size = w as usize / state.cells.x();
                    let h_size = h as usize / state.cells.y();
                    let size = std::cmp::min(w_size, h_size) as f32;
                    let x_offset = (w - size * state.cells.x() as f32) / 2.;
                    let y_offset = -(h - size * state.cells.y() as f32) / 2.;
                    let (left, top) = (left + x_offset, top + y_offset);

                    let cursor_x = state.last_cursor_position.x;
                    let cursor_y = state.last_cursor_position.y;
                    let block_x = ((cursor_x - left) / size) as i32;
                    let block_y = ((top - cursor_y) / size) as i32;
                    if block_x >= 0 && block_y >= 0 {
                        let cell_state = match state.cells.get(block_x as usize, block_y as usize) {
                            Ok(state) => match state {
                                CellState::Alive => Some(CellState::Dead),
                                CellState::Dead => Some(CellState::Alive),
                            },
                            _ => None,
                        };
                        if let Some(cell_state) = cell_state {
                            let _ = state
                                .cells
                                .set(block_x as usize, block_y as usize, cell_state);
                        }
                    }
                }
                state.egui.handle_raw_event(event);
            }
            nannou::event::WindowEvent::MouseMoved(position) => {
                state.last_cursor_position = position;
                state.egui.handle_raw_event(event);
            }
            _ => {
                // Let egui handle things like keyboard and mouse input.
                state.egui.handle_raw_event(event);
            }
        }
    } else {
        state.egui.handle_raw_event(event);
    }
}

fn model(app: &App) -> LifeGameModel {
    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();
    let egui = Egui::from_window(&window);

    LifeGameModel {
        cells: Cells::new(60, 40),
        egui: egui,
        last_cursor_position: Vec2::new(0.0, 0.0),
        running: false,
        window_padding: 15.0,
        pre_generation_time: 1000,
    }
}

fn update(_app: &App, state: &mut LifeGameModel, update: Update) {
    let egui = &mut state.egui;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Time per generation (ms):");
        ui.add(egui::Slider::new(
            &mut state.pre_generation_time,
            100..=2000,
        ));
        ui.label("Right click to make one cell alive");
        let click = ui.button("start/pause").clicked();
        if click {
            state.running = !state.running;
        }

        if state.running {
            ui.label("Running..");
        } else {
            ui.label("pausing..");
        }
    });

    if state.running {
        let current = update.since_start.as_millis();
        let last = current - update.since_last.as_millis();
        let pre_generation_time = state.pre_generation_time as u128;
        let current_sec = (current / pre_generation_time) as u64;
        let last_sec = (last / pre_generation_time) as u64;
        if current_sec != last_sec || current % pre_generation_time as u128 == 0 {
            state.cells.reduce();
        }
    }
}

fn view(app: &App, state: &LifeGameModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(rgb8(176, 211, 255));
    // get the size of one block
    let win = app.window_rect();
    let win_p = win.pad(state.window_padding);
    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));
    let (w, h) = win_p.w_h();
    let w_size = w as usize / state.cells.x();
    let h_size = h as usize / state.cells.y();
    let size = std::cmp::min(w_size, h_size) as f32;
    let x_offset = (w - size * state.cells.x() as f32) / 2.;
    let y_offset = -(h - size * state.cells.y() as f32) / 2.;

    // draw lots of blocks
    for i in 0..state.cells.x() {
        for j in 0..state.cells.y() {
            let b = Rect::from_w_h(size, size)
                .top_left_of(win_p)
                .shift(vec2(
                    i as f32 * size + x_offset,
                    -(j as f32 * size) + y_offset,
                ))
                .pad(size * 0.96);
            let color = match state.cells.get(i, j).unwrap() {
                CellState::Alive => GREEN,
                CellState::Dead => GRAY,
            };
            draw.rect().xy(b.xy()).wh(b.wh()).color(color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    state.egui.draw_to_frame(&frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
