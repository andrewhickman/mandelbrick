use std::array;

use kurbo::{Size, Rect, Point, Circle, Line};
use piet::Color;
use piet_svg::RenderContext as SvgRenderContext;
use piet_common::Device;

const TILES_X: usize = 48;
const TILES_Y: usize = 32;
const MIN_X: f64 = -2.31538881281125;
const MAX_X: f64 = 1.19729198177323;
const MIN_Y: f64 = -1.13430317325124;

const MAX_ITERATIONS: u16 = 1000;

const BACKGROUND: Color = Color::rgb8(56, 56, 56);
const COLORS: [Color; 9] = [
    Color::rgb8(246, 246, 247),
    Color::rgb8(250, 201, 165),
    Color::rgb8(248, 172, 0),
    Color::rgb8(234, 160, 198),
    Color::rgb8(0, 154, 150),
    Color::rgb8(209, 75, 150),
    Color::rgb8(0, 98, 174),
    Color::rgb8(20, 20, 20),
    Color::rgb8(0, 53, 91),
];
const SCALE: f64 = 16.0;

fn escape_time(x0: f64, y0: f64) -> u16 {
    let mut x = 0.0;
    let mut y = 0.0;
    let mut x2 = 0.0;
    let mut y2 = 0.0;

    for i in 0..MAX_ITERATIONS {
        y = (x + x) * y + y0;
        x = x2 - y2 + x0;
        x2 = x * x;
        y2 = y * y;

        if x2 + y2 > 4.0 {
            return i;
        }
    }

    // MAX_ITERATIONS
    MAX_ITERATIONS + (((x2 + y2) / 4.0) * u16::MAX as f64) as u16
}

fn escape_times() -> Vec<u16> {
    let step_x = (MAX_X - MIN_X) / TILES_X as f64;
    let step_y = step_x;

    let mut times = Vec::with_capacity(TILES_X * TILES_Y);

    let mut x = MIN_X + step_x * 0.5;
    for _ in 0..TILES_X {
        let mut y = MIN_Y + step_y * 0.5;

        for _ in 0..TILES_Y {
            times.push(escape_time(x, y));

            y += step_y;
        }

        x += step_x;
    }

    times
}

fn color_key(times: &[u16]) -> [u16; COLORS.len() - 1] {
    let mut times = times.to_vec();
    times.sort_unstable();

    // times.retain(|&t| t != MAX_ITERATIONS);

    let step = times.len() / COLORS.len();

    array::from_fn(|n| times[(n + 1) * step])
}

fn color(escape_time: u16, key: &[u16]) -> Color {
    let index = match key.binary_search(&escape_time) {
        Ok(index) => index + 1,
        Err(index) => index,
    };

    COLORS[index]
}

fn render(ctx: &mut impl piet::RenderContext) {
    let times = escape_times();
    let color_key = dbg!(color_key(&times));
    let color_key = [
        1,
        2,
        3,
        4,
        5,
        9,
        MAX_ITERATIONS,
        1550,
    ];

    for (i, c)in COLORS.iter().enumerate() {
        println!("{},{}", i + 1, times.iter().filter(|&&t| color(t, &color_key) == *c).count());
    }

    ctx.fill(Rect::from_origin_size(Point::ZERO, Size::new(TILES_X as f64 * SCALE, TILES_Y as f64 * SCALE)), &BACKGROUND);

    let radius = SCALE * 0.5;

    let mut idx = 0;
    let mut view_x = radius;

    for x in 0..TILES_X {
        let mut view_y = TILES_Y as f64 * SCALE - radius;

        for y in 0..TILES_Y {
            let escape_time = times[idx];
            let color = color(escape_time, &color_key);
            ctx.fill(Circle::new(Point::new(view_x, view_y), radius), &color);

            view_y -= SCALE;
            idx += 1;

            if x == 0 && y != 0 && y % 16 == 0 {
                ctx.stroke(Line::new(Point::new(0.0, y as f64 * SCALE), Point::new(TILES_X as f64 * SCALE, y as f64 * SCALE)), &Color::BLUE, 4.0);
            }
        }

        view_x += SCALE;

        if x != 0 && x % 16 == 0 {
            ctx.stroke(Line::new(Point::new(x as f64 * SCALE, 0.0), Point::new(x as f64 * SCALE, TILES_Y as f64 * SCALE)), &Color::BLUE, 4.0);
        }
    }

    ctx.finish().unwrap();
}

fn render_svg() {
    let mut ctx = SvgRenderContext::new(Size::new(TILES_X as f64 * SCALE, TILES_Y as f64 * SCALE));
    render(&mut ctx);
    println!("{}", ctx.display());
}

fn render_img() {
    let size = Size::new(TILES_X as f64 * SCALE, TILES_Y as f64 * SCALE);
    let mut device = Device::new().unwrap();
    let mut target = device.bitmap_target(size.width as usize, size.height as usize, 1.0).unwrap();
    let mut ctx = target.render_context();

    render(&mut ctx);
    drop(ctx);

    target.save_to_file("out.png").unwrap();
}

fn main() {
    render_img();
}