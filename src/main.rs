use nannou::prelude::*;

mod cells;
use cells::Cells;

struct LifeGameModel {
    cells: Cells,
}

fn model(_app: &App) -> LifeGameModel {
    LifeGameModel {
        cells: Cells::new(60, 40),
    }
}

fn update(_app: &App, state: &mut LifeGameModel, _update: Update) {}

fn view(app: &App, state: &LifeGameModel, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLUE);
    // get the size of one block
    let win = app.window_rect();
    let win_p = win.pad(15.0);
    draw.rect()
        .xy(win_p.xy())
        .wh(win_p.wh())
        .color(rgba(0.3, 0.4, 0.7, 0.5));
    let (w, h) = win_p.w_h();
    let w = w as usize / state.cells.x();
    let h = h as usize / state.cells.y();
    let size = std::cmp::min(w, h) as f32;

    // draw lots of blocks
    for i in 0..state.cells.x() {
        for j in 0..state.cells.y() {
            let b = Rect::from_w_h(size, size)
                .top_left_of(win_p)
                .shift(vec2((i as f32 * size), -(j as f32 * size)));
            draw.rect().xy(b.xy()).wh(b.wh()).color(GRAY);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}
