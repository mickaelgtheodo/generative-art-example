use nannou::{prelude::*, rand};

fn main() {
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .size(900, 900)
        .run();
}

struct Model {
    tiles: Vec<Tile>,
}
struct Tile {
    choice: bool,
    rect: Rect,
}

fn model(app: &App) -> Model {
    let window_rect = app.window_rect();
    let mut tiles: Vec<Tile> = Vec::new();
    for rect in window_rect.subdivisions_iter() {
        tiles.push(Tile {
            choice: rand::random(),
            rect,
        });
    }
    Model { tiles }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, model: &Model, frame: Frame) {
    frame.clear(WHITE);

    let draw = app.draw();

    let tiles: &Vec<Tile> = &model.tiles;

    for tile in tiles {
        if tile.choice {
            draw.line()
                .start(tile.rect.bottom_right())
                .end(tile.rect.top_left())
                .weight(10.0)
                .color(BLACK);
        } else {
            draw.line()
                .start(tile.rect.top_right())
                .end(tile.rect.bottom_left())
                .weight(10.0)
                .color(BLACK);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
