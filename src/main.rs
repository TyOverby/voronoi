extern crate lux;
extern crate rand;

use lux::prelude::*;
use lux::graphics::ColorVertex;

use rand::random;

const RANGE_X: usize = 512 * 2;
const RANGE_Y: usize = 512 * 2;
const NUM_PTS: usize = 40;

fn points() -> Vec<ColorVertex> {
    let mut out = Vec::with_capacity(NUM_PTS);
    for _ in 0..NUM_PTS {
        out.push(ColorVertex{
            pos: [random::<f32>() * RANGE_X as f32, random::<f32>() * RANGE_Y as f32],
            color: [random::<f32>(), random::<f32>(), random::<f32>(), 1.0]
        });
    }
    out
}

fn euclidian(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    (dx * dx + dy * dy).sqrt()
}

fn manhattan(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx.abs() + dy.abs()
}

fn maximal(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx.abs().max(dy.abs())
}

fn minimal(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let dx = x1 - x2;
    let dy = y1 - y2;
    dx.abs().min(dy.abs())
}

fn approx(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (1007.0 / 1024.0) * maximal(x1, y1, x2, y2) +
    (441.0 / 1024.0) * minimal(x1, y1, x2, y2)
}

fn min_dist(best: &mut Option<f32>, next: f32) -> bool {
    if best.is_none() {
        *best = Some(next);
        return true;
    }

    if let Some(best) = best.as_mut() {
        if next < *best {
            *best = next;
            return true;
        }
    }

    return false;
}

fn max_dist(best: &mut Option<f32>, next: f32) -> bool {
    if best.is_none() {
        *best = Some(next);
        return true;
    }

    if let Some(best) = best.as_mut() {
        if next > *best {
            *best = next;
            return true;
        }
    }

    return false;
}

fn find(x: f32, y: f32, pts: &[ColorVertex], dist: &fn(f32, f32, f32, f32) -> f32, chooser: &fn(&mut Option<f32>, f32) -> bool) -> [f32; 4] {
    let mut best_dist = None;
    let mut best_color = [0.0, 0.0, 0.0, 1.0];
    for &ColorVertex{pos, color} in pts {
        let d = dist(x, y, pos[0], pos[1]);
        if chooser(&mut best_dist, d) {
            best_color = color;
        }
    }
    best_color
}

fn main() {
    let mut window = Window::new_with_defaults().unwrap();

    let mut buffer = Vec::with_capacity(RANGE_X * RANGE_Y);
    let points = points();

    let mut distance_function = euclidian as fn(f32, f32, f32, f32) -> f32;
    let mut chooser = min_dist as fn(&mut Option<f32>, f32) -> bool;
    let mut dirty = true;

    while window.is_open() {
        let mut frame = window.frame();


        if window.is_key_pressed('1') {
            distance_function = euclidian;
            dirty = true;
        } else if window.is_key_pressed('2') {
            distance_function = manhattan;
            dirty = true;
        } else if window.is_key_pressed('3') {
            distance_function = minimal;
            dirty = true;
        } else if window.is_key_pressed('4') {
            distance_function = maximal;
            dirty = true;
        } else if window.is_key_pressed('5') {
            distance_function = approx;
            dirty = true;
        }

        if window.is_key_pressed('q') {
            chooser = min_dist;
            dirty = true;
        } else if window.is_key_pressed('r') {
            chooser = max_dist;
            dirty = true;
        }

        if dirty {
            for x in 0 .. RANGE_X {
                for y in 0 .. RANGE_Y {
                    let x = x as f32;
                    let y = y as f32;
                    buffer.push(ColorVertex {
                        pos: [x, y],
                        color: find(x, y, &points, &distance_function, &chooser)
                    });
                }
            }


            frame.draw(Pixels{
                pixels: &buffer,
                .. Default::default()
            }).unwrap();

            frame.draw(Pixels{
                pixels: &points,
                .. Default::default()
            }).unwrap();
            dirty = false;
        }
    }
}
