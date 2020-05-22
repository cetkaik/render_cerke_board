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

    // A redundant loop to demonstrate reading image data
    for x in 0..imgx {
        for y in 0..imgy {
            /* (cx, cy) should range from -10.0 to 10.0 */
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

            if -6.376 <= cx && cx <= 6.376 && -9.642 <= cy && cy <= 9.642 {
                *pixel = image::Rgb([193, 193, 193]);
            }
        }
    }

    // Save the image as “fractal.png”, the format is deduced from the path
    imgbuf.save("fractal.png").unwrap();
}