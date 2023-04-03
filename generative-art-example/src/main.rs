use nannou::{prelude::*, rand};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(900, 900)
        .run();
}

struct Model {
    diagonal_choices: Vec<bool>,
}

fn model(_app: &App) -> Model {
    let diagonal_choices = (0..4).map(|_| rand::random()).collect::<Vec<bool>>();
    Model { diagonal_choices }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    let window_rect = app.window_rect();
    let mut iter_diagonal_choices = model.diagonal_choices.iter();

    for rect in window_rect.subdivisions_iter() {
        let diagonal_choice = *iter_diagonal_choices.next().unwrap();
        if diagonal_choice {
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
