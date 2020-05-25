extern crate image;

use rand::distributions::{Distribution, Uniform};

struct Noise {
    width: usize,
    height: usize,
    data: Vec<Vec<f64>>
}

impl Noise {
    fn gen_noise(width: usize, height: usize) -> Noise {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let between = Uniform::from(0.0..1.0);
        let mut rng = rand::thread_rng();
        let mut noise: Vec<Vec<f64>> = Vec::new();
        for _ in 0..height {
            let mut vec = Vec::new();
            for _ in 0..width {
                vec.push(between.sample(&mut rng));
            }
            noise.push(vec);
        }

        Noise {
            width, height, data: noise
        }
    }

    fn sample_smooth_noise(&self, x: f64, y: f64) -> f64 {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let fract_x = x.fract();
        let fract_y = y.fract();
        let width = self.width;
        let height = self.height;
    
        //wrap around
        let x1: usize = ((x as usize) + width) % width;
        let y1: usize = ((y as usize) + height) % height;
    
        //neighbor values
        let x2: usize = (x1 + width - 1) % width;
        let y2: usize = (y1 + height - 1) % height;
    
        //smooth the noise with bilinear interpolation
        let mut value = 0.0;
        value += fract_x * fract_y * self.data[y1][x1];
        value += (1. - fract_x) * fract_y * self.data[y1][x2];
        value += fract_x * (1. - fract_y) * self.data[y2][x1];
        value += (1. - fract_x) * (1. - fract_y) * self.data[y2][x2];
    
        return value;
    }

    fn turbulence(&self, x: f64, y: f64, initial_size: f64) -> f64 {
        /* algorithm taken from https://lodev.org/cgtutor/randomnoise.html#Wood */
        let mut value = 0.0f64;
        let mut size = initial_size;
    
        while size >= 1. {
            value += self.sample_smooth_noise(
                x / size,
                y / size,
            ) * size;
            size /= 2.0;
        }
    
        return 128.0 * value / initial_size;
    }
}

fn rawwood(width: u32, height: u32, offsetstdev: f64) -> image::RgbImage {
    use rand::{*};
    let mut imgbuf = image::RgbImage::new(width, height);

    let noise = Noise::gen_noise(width as usize, height as usize);

    /* algorithm taken and modified from https://lodev.org/cgtutor/randomnoise.html#Wood */
    let wavenumber = 0.0411; // dimension: # per px
    let turb = 14.6; //makes twists
    let turb_size = 32.0; //initial size of the turbulence

    let mut rng = rand::thread_rng();
    let distr = rand_distr::Normal::new(0., offsetstdev).unwrap();
    let offsetx = rng.sample(distr);
    let offsety = rng.sample(distr);
    let phase = rng.sample(Uniform::from(0.0..std::f64::consts::PI));

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let x_value_times_scale = x as f64 - width as f64 / 2. + offsetx; // dimension: px
        let y_value_times_scale = y as f64 - height as f64 / 2. + offsety; // dimension: px
        let dist_value_times_scale = x_value_times_scale.hypot(y_value_times_scale) + turb * noise.turbulence(x as f64, y as f64, turb_size) / 256.0;
        let sine_value = 88.0 * ((wavenumber * dist_value_times_scale * std::f64::consts::PI + phase).sin()).abs().powf(0.4);
        *pixel = image::Rgb([120 + sine_value as u8, 70 + sine_value as u8, 70]);
    }

    return imgbuf;
}

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

fn main() -> Result<(), rand_distr::NormalError> {
    let rawboard = rawboard(100.0);
    rawboard.save("fractal.png").unwrap();

    let raw_wood = image::imageops::colorops::brighten(&rawwood(584, 668, 40.0), 20);

    raw_wood.save("rawwood.png").unwrap();

    for x in 0..7 {
        for y in 0..8 {
            image::imageops::crop_imm(&raw_wood, 84 * x, 84 * y, 80, 80).to_image().save(format!("rawwood_{}_{}.png", x, y)).unwrap();
        }
    }

    

    // If I succeed in implementing GIMP's bump_map later, then I will resurrect this code
    /*
    extern crate cloth_bumpmap;
    [dependencies]
    cloth_bumpmap = "0.1.1"
    let (width, height) = rawboard.dimensions();
    let bumpmap = cloth_bumpmap::cloth_bumpmap(width, height)?;

    bumpmap.save("bumpmap.png").unwrap();

    let clothed = emboss::emboss::apply_bump_map(
        rawboard,
        bumpmap,
        std::f64::consts::PI / 4.0 * 3.0,
        std::f64::consts::PI / 4.0,
    )
    .unwrap();
    clothed.save("clothed.png").unwrap();
    */
    Ok(())
}
