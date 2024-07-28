use crate::common::{Point, RGBA};

pub fn rasterize_line(v0: &Point, v1: &Point) -> Vec<Point> {
    let mut result = Vec::new();

    let mut start: Point = v0.clone();
    let mut end = v1.clone();

    if start.x > end.x {
        (start, end) = (end, start);
    }

    result.push(start.clone());

    let mut flip_y = false;
    if start.y > end.y {
        start.y = -start.y;
        end.y = -end.y;
        flip_y = true;
    }

    let mut delta_x = end.x - start.x;
    let mut delta_y = end.y - start.y;

    let mut swap_xy = false;
    if delta_x < delta_y {
        (start.x, start.y) = (start.y, start.x);
        (end.x, end.y) = (end.y, end.x);
        (delta_x, delta_y) = (delta_y, delta_x);
        swap_xy = true
    }

    let mut current_x = start.x;
    let mut current_y = start.y;

    let mut result_x;
    let mut result_y;


    let mut p = 2 * delta_y - delta_x;

    for _i in 0..delta_x {
        if p >= 0 {
            current_y += 1;
            p -= 2 * delta_x;
        }

        current_x += 1;
        p += 2 * delta_y;

        result_x = current_x;
        result_y = current_y;
        if swap_xy {
            (result_x, result_y) = (result_y, result_x);
        }

        if flip_y {
            result_y = -result_y;
        }


        let current_point: Point = Point {
            x: result_x,
            y: result_y,
            color: get_color(v0, v1, result_x, result_y),
        };


        result.push(current_point);
    }
    result
}

fn get_color(start: &Point, end: &Point, x: i32, y: i32) -> RGBA {
    let mut weight = 1.0;
    if start.x != end.x {
        weight = (x - start.x) as f64 / (end.x - start.x) as f64;
    } else if start.y != end.y {
        weight = (y - start.y) as f64 / (end.y - start.y) as f64;
    }
    let result = RGBA {
        r: (start.color.r as f64 * (1.0 - weight) + weight * end.color.r as f64) as u8,
        g: (start.color.g as f64 * (1.0 - weight) + weight * end.color.g as f64) as u8,
        b: (start.color.b as f64 * (1.0 - weight) + weight * end.color.b as f64) as u8,
        a: (start.color.a as f64 * (1.0 - weight) + weight * end.color.a as f64) as u8,
    };

    result
}