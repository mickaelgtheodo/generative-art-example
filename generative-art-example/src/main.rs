use nannou::prelude::*;

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

    draw.line()
        .start(app.window_rect().bottom_right())
        .end(app.window_rect().top_left())
        .weight(10.0)
        .color(BLACK);

    draw.to_frame(app, &frame).unwrap();
}
