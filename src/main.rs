//! An example of generating julia fractals.
extern crate image;
extern crate num_complex;

fn main() {
    let imgx = 900;
    let imgy = 900;

    let scalex = 20.0 / imgx as f32;
    let scaley = 20.0 / imgy as f32;

    // Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    /*for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }*/

    let tak1_color = image::Rgb([193, 193, 193]);
    let tam2hue_color = image::Rgb([204, 136, 82]);
    let tam2zo1_color = image::Rgb([32, 72, 38]);
    let tam2nua2_color = image::Rgb([98, 133, 177]);
    let line_width = 0.04;
    let line_color = image::Rgb([10, 10, 10]);

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            /* (cx, cy) should range from -10.0 to 10.0 */
            /* the size of each square is 1.0 */
            let cx = x as f32 * scalex - 10.0;
            let cy = y as f32 * scaley - 10.0;

            let c = num_complex::Complex::new(-0.4, 0.6);
            let mut z = num_complex::Complex::new(cx, cy);

            let mut i = 0;
            while i < 255 && z.norm() <= 2.0 {
                z = z * z + c;
                i += 1;
            }

            let pixel = imgbuf.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            *pixel = image::Rgb([data[0], i as u8, data[2]]);

            /* first draw the board */
            if -6.376 <= cx && cx <= 6.376 && -9.642 <= cy && cy <= 9.642 {
                *pixel = tak1_color;
            }

            /* then draw the squares */
            if -1.5 <= cx && cx <= 1.5 && -1.5 <= cy && cy <= 1.5 {
                *pixel = tam2hue_color;
            }
            if 1.5 <= cx.abs() && cx.abs() <= 2.5 && 1.5 <= cy.abs() && cy.abs() <= 2.5 {
                *pixel = tam2hue_color;
            }
            if (-2.5 <= cx && cx <= 2.5 && -0.5 <= cy && cy <= 0.5)
                || (-2.5 <= cy && cy <= 2.5 && -0.5 <= cx && cx <= 0.5)
            {
                *pixel = if -0.5 <= cx && cx <= 0.5 && -0.5 <= cy && cy <= 0.5 {
                    tam2zo1_color
                } else {
                    tam2nua2_color
                }
            }

            /* Now draw the lines */
            for loc in vec![-4.5, -3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5, 4.5] {
                if (loc - cx).abs() <= line_width / 2.0 && cy.abs() <= 4.5 + line_width / 2.0 {
                    *pixel = line_color;
                }
                if (loc - cy).abs() <= line_width / 2.0 && cx.abs() <= 4.5 + line_width / 2.0 {
                    *pixel = line_color;
                }
            }
            if (cx + cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
                *pixel = line_color;
            }
            if (cx - cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
                *pixel = line_color;
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}
