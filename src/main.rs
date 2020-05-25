extern crate image;

const BNUAK: &'static [u8] = include_bytes!("bnuak.png_80x80.png");
const BKAUK: &'static [u8] = include_bytes!("bkauk.png_80x80.png");
const BKAUN: &'static [u8] = include_bytes!("bkaun.png_80x80.png");
const BMAUN: &'static [u8] = include_bytes!("bmaun.png_80x80.png");
const BKUA: &'static [u8] = include_bytes!("bkua.png_80x80.png");
const BGUA: &'static [u8] = include_bytes!("bgua.png_80x80.png");
const BTAM: &'static [u8] = include_bytes!("btam.png_80x80.png");
const BTUK: &'static [u8] = include_bytes!("btuk.png_80x80.png");
const BDAU: &'static [u8] = include_bytes!("bdau.png_80x80.png");
const BIO: &'static [u8] = include_bytes!("bio.png_80x80.png");
const BUAI: &'static [u8] = include_bytes!("buai.png_80x80.png");

const RNUAK: &'static [u8] = include_bytes!("rnuak.png_80x80.png");
const RKAUK: &'static [u8] = include_bytes!("rkauk.png_80x80.png");
const RKAUN: &'static [u8] = include_bytes!("rkaun.png_80x80.png");
const RMAUN: &'static [u8] = include_bytes!("rmaun.png_80x80.png");
const RKUA: &'static [u8] = include_bytes!("rkua.png_80x80.png");
const RGUA: &'static [u8] = include_bytes!("rgua.png_80x80.png");
const RTUK: &'static [u8] = include_bytes!("rtuk.png_80x80.png");
const RDAU: &'static [u8] = include_bytes!("rdau.png_80x80.png");
const RIO: &'static [u8] = include_bytes!("rio.png_80x80.png");
const RUAI: &'static [u8] = include_bytes!("ruai.png_80x80.png");

use rand::distributions::{Distribution, Uniform};

enum Color {
    Kok1,  // Red, 赤
    Huok2, // Black, 黒
}

enum Profession {
    Nuak1, // Vessel, 船, felkana
    Kauk2, // Pawn, 兵, elmer
    Gua2,  // Rook, 弓, gustuer
    Kaun1, // Bishop, 車, vadyrd
    Dau2,  // Tiger, 虎, stistyst
    Maun1, // Horse, 馬, dodor
    Kua2,  // Clerk, 筆, kua
    Tuk2,  // Shaman, 巫, terlsk
    Uai1,  // General, 将, varxle
    Io,    // King, 王, ales
}

enum Side {
    ASide,
    IASide,
}

struct PhysicalPiece {
    color: Color,
    profession: Profession,
    image: image::RgbImage,
}

struct PhysicalTam {
    image: image::RgbImage,
}

enum Piece {
    NonTam2(PhysicalPiece, Side),
    Tam2(PhysicalTam),
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AbsoluteRow {
    A,
    E,
    I,
    U,
    O,
    Y,
    AI,
    AU,
    IA,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum AbsoluteColumn {
    K,
    L,
    N,
    T,
    Z,
    X,
    C,
    M,
    P,
}

type AbsoluteCoord = (AbsoluteRow, AbsoluteColumn);

use std::collections::HashMap;

struct Field {
    field: HashMap<AbsoluteCoord, Piece>,
    a_side_hand: Vec<PhysicalPiece>,
    ia_side_hand: Vec<PhysicalPiece>,
}

struct Noise {
    width: usize,
    height: usize,
    data: Vec<Vec<f64>>,
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
            width,
            height,
            data: noise,
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
            value += self.sample_smooth_noise(x / size, y / size) * size;
            size /= 2.0;
        }

        return 128.0 * value / initial_size;
    }
}

fn rawwood(width: u32, height: u32, offsetstdev: f64) -> image::RgbImage {
    use rand::*;
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
        let dist_value_times_scale = x_value_times_scale.hypot(y_value_times_scale)
            + turb * noise.turbulence(x as f64, y as f64, turb_size) / 256.0;
        let sine_value = 88.0
            * ((wavenumber * dist_value_times_scale * std::f64::consts::PI + phase).sin())
                .abs()
                .powf(0.4);
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

fn multiply_channel(a: u8, b: u8) -> u8 {
    ((a as f32) * (b as f32) / 255.0) as u8
}
fn multiply_pixel(a: image::Rgb<u8>, b: image::Rgb<u8>) -> image::Rgb<u8> {
    let image::Rgb(a) = a;
    let image::Rgb(b) = b;
    image::Rgb([
        multiply_channel(a[0], b[0]),
        multiply_channel(a[1], b[1]),
        multiply_channel(a[2], b[2]),
    ])
}

fn multiply_image(a: &image::RgbImage, b: &image::RgbImage) -> Option<image::RgbImage> {
    let (width, height) = a.dimensions();
    if b.dimensions() != (width, height) {
        return None;
    }
    let mut c = image::RgbImage::new(width, height);
    for (x, y, pixel) in c.enumerate_pixels_mut() {
        *pixel = multiply_pixel(*a.get_pixel(x, y), *b.get_pixel(x, y))
    }
    Some(c)
}

fn main() -> Result<(), rand_distr::NormalError> {
    let rawboard = rawboard(100.0);
    rawboard.save("fractal.png").unwrap();
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

    let field = generate_field();

    Ok(())
}

fn generate_field() -> Field {
    let raw_wood = image::imageops::colorops::brighten(&rawwood(584, 668, 40.0), 20);

    raw_wood.save("rawwood.png").unwrap();

    let mut pieces = Vec::new();

    use rand::seq::SliceRandom;
    for x in 0..7 {
        for y in 0..8 {
            let image = image::imageops::crop_imm(&raw_wood, 84 * x, 84 * y, 80, 80).to_image();
            pieces.push(image);
        }
    }
    let mut rng = rand::thread_rng();
    pieces.shuffle(&mut rng);

    let mut i = 0;

    let tam2_image = image::load_from_memory(&BTAM).unwrap().to_rgb();

    let res = multiply_image(&tam2_image, &pieces[i]).unwrap();
    res.save(format!("rawwood_{}.png", i)).unwrap();
    i += 1;

    let physical_tam = PhysicalTam { image: res };

    let mut hashmap = HashMap::new();
    hashmap.insert(
        (AbsoluteRow::O, AbsoluteColumn::Z),
        Piece::Tam2(physical_tam),
    );

    for (character, col, row, profession, color) in vec![
        (&BNUAK, AbsoluteColumn::Z, AbsoluteRow::AI, Profession:: Nuak1, Color::Huok2),
        (&RNUAK, AbsoluteColumn::Z, AbsoluteRow::I, Profession:: Nuak1, Color::Kok1),
        (&BKAUK, AbsoluteColumn::K, AbsoluteRow::I, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::N, AbsoluteRow::I, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::C, AbsoluteRow::I, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::P, AbsoluteRow::I, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::K, AbsoluteRow::AI, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::N, AbsoluteRow::AI, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::C, AbsoluteRow::AI, Profession:: Kauk2, Color::Huok2),
        (&BKAUK, AbsoluteColumn::P, AbsoluteRow::AI, Profession:: Kauk2, Color::Huok2),
        (&RKAUK, AbsoluteColumn::L, AbsoluteRow::I, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::T, AbsoluteRow::I, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::X, AbsoluteRow::I, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::M, AbsoluteRow::I, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::L, AbsoluteRow::AI, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::T, AbsoluteRow::AI, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::X, AbsoluteRow::AI, Profession:: Kauk2, Color::Kok1),
        (&RKAUK, AbsoluteColumn::M, AbsoluteRow::AI, Profession:: Kauk2, Color::Kok1),
        (&BGUA, AbsoluteColumn::L, AbsoluteRow::AU, Profession:: Gua2, Color::Huok2),
        (&BGUA, AbsoluteColumn::M, AbsoluteRow::E, Profession:: Gua2, Color::Huok2),
        (&RGUA, AbsoluteColumn::L, AbsoluteRow::E, Profession:: Gua2, Color::Kok1),
        (&RGUA, AbsoluteColumn::M, AbsoluteRow::AU, Profession:: Gua2, Color::Kok1),
        (&BKAUN, AbsoluteColumn::N, AbsoluteRow::A, Profession:: Kaun1, Color::Huok2),
        (&BKAUN, AbsoluteColumn::C, AbsoluteRow::IA, Profession:: Kaun1, Color::Huok2),
        (&RKAUN, AbsoluteColumn::N, AbsoluteRow::IA, Profession:: Kaun1, Color::Kok1),
        (&RKAUN, AbsoluteColumn::C, AbsoluteRow::A, Profession:: Kaun1, Color::Kok1),
        (&BDAU, AbsoluteColumn::X, AbsoluteRow::E, Profession:: Dau2, Color::Huok2),
        (&BDAU, AbsoluteColumn::T, AbsoluteRow::AU, Profession:: Dau2, Color::Huok2),
        (&RDAU, AbsoluteColumn::T, AbsoluteRow::E, Profession:: Dau2, Color::Kok1),
        (&RDAU, AbsoluteColumn::X, AbsoluteRow::AU, Profession:: Dau2, Color::Kok1),
        (&BMAUN, AbsoluteColumn::L, AbsoluteRow::A, Profession:: Maun1, Color::Huok2),
        (&BMAUN, AbsoluteColumn::M, AbsoluteRow::IA, Profession:: Maun1, Color::Huok2),
        (&RMAUN, AbsoluteColumn::M, AbsoluteRow::A, Profession:: Maun1, Color::Kok1),
        (&RMAUN, AbsoluteColumn::L, AbsoluteRow::IA, Profession:: Maun1, Color::Kok1),
        (&BKUA, AbsoluteColumn::P, AbsoluteRow::IA, Profession:: Kua2, Color::Huok2),
        (&BKUA, AbsoluteColumn::K, AbsoluteRow::A, Profession:: Kua2, Color::Huok2),
        (&RKUA, AbsoluteColumn::P, AbsoluteRow::A, Profession:: Kua2, Color::Kok1),
        (&RKUA, AbsoluteColumn::K, AbsoluteRow::IA, Profession:: Kua2, Color::Kok1),
        (&BTUK, AbsoluteColumn::P, AbsoluteRow::E, Profession:: Tuk2, Color::Huok2),
        (&BTUK, AbsoluteColumn::K, AbsoluteRow::AU, Profession:: Tuk2, Color::Huok2),
        (&RTUK, AbsoluteColumn::K, AbsoluteRow::E, Profession:: Tuk2, Color::Kok1),
        (&RTUK, AbsoluteColumn::P, AbsoluteRow::AU, Profession:: Tuk2, Color::Kok1),
        (&BUAI, AbsoluteColumn::T, AbsoluteRow::A, Profession:: Uai1, Color::Huok2),
        (&BUAI, AbsoluteColumn::X, AbsoluteRow::IA, Profession:: Uai1, Color::Huok2),
        (&RUAI, AbsoluteColumn::X, AbsoluteRow::A, Profession:: Uai1, Color::Kok1),
        (&RUAI, AbsoluteColumn::T, AbsoluteRow::IA, Profession:: Uai1, Color::Kok1),
        (&BIO, AbsoluteColumn::Z, AbsoluteRow::IA, Profession:: Io, Color::Huok2),
        (&RIO, AbsoluteColumn::Z, AbsoluteRow::A, Profession:: Io, Color::Huok2),
    ] {
        let char_image = image::load_from_memory(character).unwrap().to_rgb();

        let res = multiply_image(&char_image, &pieces[i]).unwrap();
        res.save(format!("rawwood_{}.png", i)).unwrap();

        hashmap.insert(
            (row, col),
            Piece::NonTam2(
                PhysicalPiece {
                    color,
                    profession,
                    image: res,
                },
                if 
                row == AbsoluteRow::A ||
                row == AbsoluteRow::E ||
                row == AbsoluteRow::I {
                    Side::ASide
                } else {
                    Side::IASide
                },
            ),
        );

        i += 1;
    }

    let board = Field {
        a_side_hand: Vec::new(),
        ia_side_hand: Vec::new(),
        field: hashmap,
    };

    board
}
