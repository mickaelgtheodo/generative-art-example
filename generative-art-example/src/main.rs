use nannou::{prelude::*, rand};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(900, 900)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    let window_rect = app.window_rect();

    for rect in window_rect.subdivisions_iter() {
        if rand::random() {
            draw.line()
                .start(rect.bottom_right())
                .end(rect.top_left())
                .weight(10.0)
                .color(BLACK);
        } else {
            draw.line()
                .start(rect.top_right())
                .end(rect.bottom_left())
                .weight(10.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
