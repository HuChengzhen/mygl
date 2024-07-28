extern crate image as im;
use piston_window::*;
use vecmath::*;
use mygl::common::{Point, RGBA};
use mygl::raster::rasterize_line;


fn main() {
    let opengl = OpenGL::V3_2;
    let (width, height) = (301, 301);
    let mut window: PistonWindow =
        WindowSettings::new("piston: paint", (width, height))
            .exit_on_esc(true)
            .graphics_api(opengl)
            .build()
            .unwrap();

    let mut canvas = im::ImageBuffer::new(width, height);
    let r: f64 = 150.0;
    for i in (0..360).step_by(10) {
        let radian = (i as f64).to_radians();
        let x = r * radian.sin() + 150.0;
        let y = r * radian.cos() + 150.0;
        let center = Point {
            x: 150,
            y: 150,
            color: RGBA {
                r: rand::random(),
                g: rand::random(),
                b: rand::random(),
                a: 255,
            },
        };
        let point1 = Point {
            x: x as i32,
            y: y as i32,
            color: RGBA {
                r: rand::random(),
                g: rand::random(),
                b: rand::random(),
                a: 255,
            },
        };
        let line = rasterize_line(&center, &point1);


        for point in line {
            println!("{:?}", point);
            canvas.put_pixel(point.x as u32, point.y as u32, im::Rgba([point.color.r, point.color.g, point.color.b, point.color.a]))
        }
    }


    let mut draw = false;
    let mut texture_context = TextureContext {
        factory: window.factory.clone(),
        encoder: window.factory.create_command_buffer().into(),
    };
    let mut texture: G2dTexture = Texture::from_image(
        &mut texture_context,
        &canvas,
        &TextureSettings::new(),
    ).unwrap();

    let mut last_pos: Option<[f64; 2]> = None;

    while let Some(e) = window.next() {
        if e.render_args().is_some() {
            texture.update(&mut texture_context, &canvas).unwrap();
            window.draw_2d(&e, |c, g, device| {
                // Update texture before rendering.
                texture_context.encoder.flush(device);

                clear([1.0; 4], g);
                image(&texture, c.transform, g);
            });
        }
        if let Some(button) = e.press_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = true;
            }
        };
        if let Some(button) = e.release_args() {
            if button == Button::Mouse(MouseButton::Left) {
                draw = false;
                last_pos = None
            }
        };
        if draw {
            if let Some(pos) = e.mouse_cursor_args() {
                let (x, y) = (pos[0] as f32, pos[1] as f32);

                if let Some(p) = last_pos {
                    let (last_x, last_y) = (p[0] as f32, p[1] as f32);
                    let distance = vec2_len(vec2_sub(p, pos)) as u32;

                    for i in 0..distance {
                        let diff_x = x - last_x;
                        let diff_y = y - last_y;
                        let delta = i as f32 / distance as f32;
                        let new_x = (last_x + (diff_x * delta)) as u32;
                        let new_y = (last_y + (diff_y * delta)) as u32;
                        if new_x < width && new_y < height {
                            canvas.put_pixel(new_x, new_y, im::Rgba([255, 0, 0, 255]));
                        };
                    };
                };

                last_pos = Some(pos)
            };
        }
    }
}