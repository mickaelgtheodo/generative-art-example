use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(NAVY);
    draw.ellipse().color(MEDIUMAQUAMARINE);
    draw.to_frame(app, &frame).unwrap();
}
