use draw::*;

use image::save_buffer;
use image::ColorType::Rgba8;
use indicatif::ProgressBar;

const WIDTH: u32 = 4000;
const HEIGHT: u32 = 2000;

trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for u8 {
    fn clamp(self, min: Self, max: Self) -> Self {
        if self < min {
            min
        } else if self > max {
            max
        } else {
            self
        }
    }
}

fn randf() -> f32 {
    rand::random::<f32>()
}

fn randu() -> u8 {
    rand::random::<u8>()
}

fn rand_sign() -> f32 {
    ((rand::random::<bool>()) as i32 * -1) as f32
}

fn star_color(r: Option<f32>) -> RGB {
    let r = if let Some(r) = r { r } else { randf() };
    let (g, b) = if r < 0.0025 {
        (randu().max(215), randu().max(215))
    } else if r < 0.05 {
        (235, 255)
    } else {
        (randu().max(245), randu().max(245))
    };
    RGB {
        r: randu().max(210),
        g: g,
        b: b,
    }
}

fn stretchy_star() -> Vec<Drawing> {
    let x = randf() * WIDTH as f32;
    let y = randf() * HEIGHT as f32;
    // let length = randf() * 5.0;
    // let randv = || randf() * length / 2.0 * rand_sign();
    // let vx1 = randv();
    // let vy1 = randv();
    // let vx2 = randv();
    // let vy2 = randv();

    // let (x1, x2) = (x + vx1, x + vx2);
    // let (y1, y2) = (y + vy1, y + vy2);
    let color = star_color(None);
    // let curve = |f: f32| {
    //     f + randf().clamp(0.001, 0.005)
    // };
    vec![
        //TODO: Make stars stretchy by drawing little bezier curves.
        //      Then rotate and shear them with 2d transformations.
        // Drawing::new()
        //     .with_shape(
        //         shape::LineBuilder::new(x, x)
        //         .curve_to(x1, y1, curve(x1), curve(y1))
        //         .curve_to(x2, y2, curve(x2), curve(y2))
        //         .build()
        //     )
        //     .with_style(Style::filled(color)),
        Drawing::new()
            .with_shape(Shape::Circle { radius: 1 })
            .with_xy(x, y)
            .with_style(Style::filled(color)),
    ]
}

fn star(pos: Option<(f32, f32)>) -> Vec<Drawing> {
    let (x, y) = if let Some((x, y)) = pos {
        (x, y)
    } else {
        (randf() * WIDTH as f32, randf() * HEIGHT as f32)
    };
    let r = randf();
    let radius = if r < 0.0025 {
        3
    } else if r < 0.05 {
        2
    } else {
        1
    };
    vec![Drawing::new()
        .with_shape(Shape::Circle { radius: radius })
        .with_xy(x, y)
        .with_style(Style::filled(star_color(Some(r))))]
}

fn swirl() -> Vec<Drawing> {
    let (xo, yo) = (randf() * WIDTH as f32, randf() * HEIGHT as f32);
    let (v, w) = (
        (randf() * 2.9).min(0.1) * rand_sign(),
        (randf() * 2.9).min(0.1) * rand_sign(),
    );
    let tn = std::f32::consts::PI * (randf() * 96.0);
    let steps = rand::random::<usize>().clamp(18, 18 * 3);
    let step = tn / (steps as f32);
    let mut t = 0.0;
    let mut stars: Vec<Drawing> = vec![];
    while t < tn {
        let (x, y) = (
            ((v * t) * (w * t).cos()) + xo,
            ((v * t) * (w * t).sin()) + yo,
        );
        //TODO Add rotation and shear here.
        stars.extend(star(Some((x, y))).into_iter());
        t += step;
    }
    stars
}

fn random_object() -> Vec<Drawing> {
    let r = randf();
    if r < 0.0025 {
        swirl()
    } else if r < 0.16 {
        stretchy_star()
    } else {
        star(None)
    }
}

fn main() {
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    canvas.display_list.add(
        Drawing::new()
            .with_shape(Shape::Rectangle {
                width: WIDTH,
                height: HEIGHT,
            })
            .with_position(Point { x: 0.0, y: 0.0 })
            .with_style(Style::filled(RGB { r: 0, g: 0, b: 0 })),
    );
    let n = 110_000;
    let bar = ProgressBar::new(n).with_message(format!("Drawing {} stars...", n));
    for _ in 0..n {
        let drawings = random_object();
        for drawing in drawings {
            canvas.display_list.add(drawing);
        }
        bar.inc(1);
    }
    bar.finish();
    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Saving as .svg...");
    render::save(&canvas, "./stars.svg", SvgRenderer::new()).unwrap();
    spinner.set_message("Converting .svg to .png...");
    spinner.enable_steady_tick(100);
    let svg = nsvg::parse_file(
        std::path::Path::new("./stars.svg"),
        nsvg::Units::Pixel,
        0.01,
    )
    .unwrap();
    let image = svg.rasterize(1.0).unwrap();
    spinner.set_message("Saving buffer...");
    spinner.enable_steady_tick(100);
    save_buffer(
        std::path::Path::new("stars.png"),
        &image.into_raw(),
        WIDTH,
        HEIGHT,
        Rgba8,
    )
    .unwrap();
    spinner.finish_with_message("All done!");
}
