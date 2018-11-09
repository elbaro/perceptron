use nannou::prelude::*;

struct Point {
    x: f32,
    y: f32,
}

impl std::ops::Mul for &Point {
    type Output = f32;
    fn mul(self, rhs: Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y
    }
}

struct Sample {
    point: Point,
    label: i8,
}

struct Line {
    w: Point,
    b: f32,
}

impl Line {
    fn new() -> Line {
        let w = Point {
            x: rand::random::<f32>() * 2.0 - 1.0,
            y: rand::random::<f32>() * 2.0 - 1.0,
        };
        let b: f32 = (rand::random::<f32>() - 0.5) / 2.0;
        Line { w, b }
    }

    fn eval(&self, p: &Point) -> f32 {
        p * &self.w + self.b
    }
}

struct Model {
    a: Vec<Sample>,
    l: Line,
}

fn main() {
    nannou::run(model, event, view);
}

fn model(_app: &App) -> Model {
    let mut model = Model {
        a: Vec::new(),
        l: Line::new(),
    };

    for _ in 0..50 {
        let x: f32 = rand::random::<f32>() * 2.0 - 1.0;
        let y: f32 = rand::random::<f32>() * 2.0 - 1.0;
        let p = Point { x, y };
        let v: f32 = model.l.eval(&p);
        let label: i8 = if v > 0.0 { 1 } else { -1 };
        model.a.push(Sample {
            point: Point { x, y },
            label,
        })
    }

    model.l = Line::new();
    model
}

fn event(app: &App, mut model: Model, event: Event) -> Model {
    if let Event::Update(_update) = event {
        let alpha: f32 = 1e-5;
        let learning_rate: f32 = app.duration.since_prev_update.ms() as f32 * alpha;
        for sample in &mut model.a {
            let v = model.l.eval(&sample.point);
            if (v > 0.0) != (sample.label == 1) {
                // w <- w + learning_rate * sgn() *
                model.l.w.x += learning_rate * sample.label as f32 * sample.point.x;
                model.l.w.y += learning_rate * sample.label as f32 * sample.point.y;
                model.l.b += learning_rate * sample.label as f32 * 1.0;
            }
        }
    }
    model
}

fn view(app: &App, model: &Model, frame: Frame) -> Frame {
    // Begin drawing
    let win = app.window_rect();
    let draw = app.draw();

    // Clear the background to blue.
    draw.background().color(BLACK);

    // Create an `ngon` of points.
    let w = win.w() / 2.0; // -w ~ +w
    let h = win.h() / 2.0;

    for sample in &model.a {
        let x = sample.point.x * w;
        let y = sample.point.y * h;
        let color = if sample.label == 1 { WHITE } else { LIGHT_RED };
        draw.ellipse().x_y(x, y).w_h(8.0, 8.0).color(color);
    }

    draw.line()
        .start(pt2(-w, (-model.l.b + model.l.w.x) / model.l.w.y * h))
        .end(pt2(w, (-model.l.b - model.l.w.x) / model.l.w.y * h))
        .color(BLUE);

    draw.to_frame(app, &frame).unwrap();

    // Return the drawn frame.
    frame
}
