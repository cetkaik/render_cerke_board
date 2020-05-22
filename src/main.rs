extern crate image;

fn rawboard(square_size_in_pixel: f32) -> image::RgbImage {
    /* Numbers based on physical measurements */
    let tak1_color = image::Rgb([193, 193, 193]);
    let tam2hue_color = image::Rgb([204, 136, 82]);
    let tam2zo1_color = image::Rgb([32, 72, 38]);
    let tam2nua2_color = image::Rgb([98, 133, 177]);
    let line_width = 0.04;
    let line_color = image::Rgb([10, 10, 10]);
    let cwidth = 6.376 * 2.;
    let cheight = 9.642 * 2.;

    let imgx = (square_size_in_pixel * cwidth) as u32;
    let imgy = (square_size_in_pixel * cheight) as u32;

    /* first draw the board */
    let mut imgbuf = image::ImageBuffer::from_pixel(imgx, imgy, tak1_color);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        /* the size of each square is 1.0 */
        /* the center of the image is the origin */
        let cx = x as f32 / square_size_in_pixel - cwidth / 2.;
        let cy = y as f32 / square_size_in_pixel - cheight / 2.;

        /* the board is already drawn */
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

        /* horizontal and vertical */
        for loc in vec![-4.5, -3.5, -2.5, -1.5, -0.5, 0.5, 1.5, 2.5, 3.5, 4.5] {
            if (loc - cx).abs() <= line_width / 2.0 && cy.abs() <= 4.5 + line_width / 2.0 {
                *pixel = line_color;
            }
            if (loc - cy).abs() <= line_width / 2.0 && cx.abs() <= 4.5 + line_width / 2.0 {
                *pixel = line_color;
            }
        }

        /* tam2nua2 corners */
        if (2.5 - line_width * 2. - cx.abs()).abs() <= line_width / 2.0
            && cy.abs() <= 0.5 - line_width * 1.5
            && 0.25 <= cy.abs()
        {
            *pixel = line_color;
        }
        if (2.5 - line_width * 2. - cy.abs()).abs() <= line_width / 2.0
            && cx.abs() <= 0.5 - line_width * 1.5
            && 0.25 <= cx.abs()
        {
            *pixel = line_color;
        }
        if (0.5 - line_width * 2. - cy.abs()).abs() <= line_width / 2.0
            && cx.abs() <= 2.5 - line_width * 1.5
            && 2.25 <= cx.abs()
        {
            *pixel = line_color;
        }
        if (0.5 - line_width * 2. - cx.abs()).abs() <= line_width / 2.0
            && cy.abs() <= 2.5 - line_width * 1.5
            && 2.25 <= cy.abs()
        {
            *pixel = line_color;
        }

        /* tam2hue diagonal */
        if (cx + cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
            *pixel = line_color;
        }
        if (cx - cy).abs() <= line_width * std::f32::consts::FRAC_1_SQRT_2 && cx.abs() <= 2.5 {
            *pixel = line_color;
        }
    }

    return imgbuf;
}

fn main() {
    let rawboard = rawboard(100.0);
    let (width, height) = rawboard.dimensions();

    let mut imgbuf = image::RgbImage::from_pixel(width, height, image::Rgb([255, 255, 255]));

    // Save the image as “fractal.png”, the format is deduced from the path
    rawboard.save("fractal.png").unwrap();
    imgbuf.save("new_layer.png").unwrap();
}
