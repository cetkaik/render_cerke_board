const BNUAK: &'static [u8] = include_bytes!("../bnuak.png_80x80.png");
const BKAUK: &'static [u8] = include_bytes!("../bkauk.png_80x80.png");
const BKAUN: &'static [u8] = include_bytes!("../bkaun.png_80x80.png");
const BMAUN: &'static [u8] = include_bytes!("../bmaun.png_80x80.png");
const BKUA: &'static [u8] = include_bytes!("../bkua.png_80x80.png");
const BGUA: &'static [u8] = include_bytes!("../bgua.png_80x80.png");
const BTAM: &'static [u8] = include_bytes!("../btam.png_80x80.png");
const BTUK: &'static [u8] = include_bytes!("../btuk.png_80x80.png");
const BDAU: &'static [u8] = include_bytes!("../bdau.png_80x80.png");
const BIO: &'static [u8] = include_bytes!("../bio.png_80x80.png");
const BUAI: &'static [u8] = include_bytes!("../buai.png_80x80.png");

const RNUAK: &'static [u8] = include_bytes!("../rnuak.png_80x80.png");
const RKAUK: &'static [u8] = include_bytes!("../rkauk.png_80x80.png");
const RKAUN: &'static [u8] = include_bytes!("../rkaun.png_80x80.png");
const RMAUN: &'static [u8] = include_bytes!("../rmaun.png_80x80.png");
const RKUA: &'static [u8] = include_bytes!("../rkua.png_80x80.png");
const RGUA: &'static [u8] = include_bytes!("../rgua.png_80x80.png");
const RTUK: &'static [u8] = include_bytes!("../rtuk.png_80x80.png");
const RDAU: &'static [u8] = include_bytes!("../rdau.png_80x80.png");
const RIO: &'static [u8] = include_bytes!("../rio.png_80x80.png");
const RUAI: &'static [u8] = include_bytes!("../ruai.png_80x80.png");

pub enum Color {
    Kok1,  // Red, 赤
    Huok2, // Black, 黒
}

pub enum Profession {
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

pub enum Side {
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

pub enum Piece {
    NonTam2(PhysicalPiece, Side),
    Tam2(PhysicalTam),
}

impl Piece {
    fn image(&self) -> image::RgbImage {
        match self {
            Piece::NonTam2(pp, _) => pp.image.clone(),
            Piece::Tam2(pt) => pt.image.clone()
        }
    }
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

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum AbsoluteRow {
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
pub enum AbsoluteColumn {
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

pub struct Field {
    field: HashMap<AbsoluteCoord, Piece>,
    a_side_hand: Vec<PhysicalPiece>,
    ia_side_hand: Vec<PhysicalPiece>,
    background: image::RgbImage,
    piece_dimension: u32,
}

mod background;
mod noise;

fn load_from_80x80(data: &'static[u8], dimension: u32) -> image::RgbImage {
    let image = image::load_from_memory(data).unwrap().to_rgb();
    if dimension == 80 {
        image
    } else {
        image::imageops::resize(&image, dimension, dimension, image::imageops::FilterType::CatmullRom)
    }
}

impl Field {
    pub fn render(&self) -> image::RgbImage {
        use crate::image::GenericImage;
        let mut background = self.background.clone();
        let (width, height) = background.dimensions();

        for (x, y, pixel) in self.field[&(AbsoluteRow::O, AbsoluteColumn::Z)].image().enumerate_pixels() {
            background.sub_image(
                width / 2 - self.piece_dimension / 2,
                height / 2 - self.piece_dimension / 2,
                self.piece_dimension,
                self.piece_dimension
            ).put_pixel(x, y, *pixel);
        }
        
       background
    }

    pub fn new() -> Field {
        let piece_dimension = 80;
        let padding = 4;

        let raw_wood = image::imageops::colorops::brighten(
            &noise::rawwood(
                (piece_dimension + padding) * 6 + piece_dimension,
                (piece_dimension + padding) * 7 + piece_dimension,
                piece_dimension as f64 / 2.,
            ),
            20,
        );

        raw_wood.save("rawwood.png").unwrap();

        let mut pieces = Vec::new();

        use rand::seq::SliceRandom;
        for x in 0..7 {
            for y in 0..8 {
                let image = image::imageops::crop_imm(
                    &raw_wood,
                    (piece_dimension + padding) * x,
                    (piece_dimension + padding) * y,
                    piece_dimension,
                    piece_dimension,
                )
                .to_image();
                pieces.push(image);
            }
        }
        let mut rng = rand::thread_rng();
        pieces.shuffle(&mut rng);

        let mut i = 0;

        let tam2_image = load_from_80x80(&BTAM, piece_dimension);

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
            (
                &BNUAK,
                AbsoluteColumn::Z,
                AbsoluteRow::AI,
                Profession::Nuak1,
                Color::Huok2,
            ),
            (
                &RNUAK,
                AbsoluteColumn::Z,
                AbsoluteRow::I,
                Profession::Nuak1,
                Color::Kok1,
            ),
            (
                &BKAUK,
                AbsoluteColumn::K,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::N,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::C,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::P,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::K,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::N,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::C,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &BKAUK,
                AbsoluteColumn::P,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Huok2,
            ),
            (
                &RKAUK,
                AbsoluteColumn::L,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::T,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::X,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::M,
                AbsoluteRow::I,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::L,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::T,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::X,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &RKAUK,
                AbsoluteColumn::M,
                AbsoluteRow::AI,
                Profession::Kauk2,
                Color::Kok1,
            ),
            (
                &BGUA,
                AbsoluteColumn::L,
                AbsoluteRow::AU,
                Profession::Gua2,
                Color::Huok2,
            ),
            (
                &BGUA,
                AbsoluteColumn::M,
                AbsoluteRow::E,
                Profession::Gua2,
                Color::Huok2,
            ),
            (
                &RGUA,
                AbsoluteColumn::L,
                AbsoluteRow::E,
                Profession::Gua2,
                Color::Kok1,
            ),
            (
                &RGUA,
                AbsoluteColumn::M,
                AbsoluteRow::AU,
                Profession::Gua2,
                Color::Kok1,
            ),
            (
                &BKAUN,
                AbsoluteColumn::N,
                AbsoluteRow::A,
                Profession::Kaun1,
                Color::Huok2,
            ),
            (
                &BKAUN,
                AbsoluteColumn::C,
                AbsoluteRow::IA,
                Profession::Kaun1,
                Color::Huok2,
            ),
            (
                &RKAUN,
                AbsoluteColumn::N,
                AbsoluteRow::IA,
                Profession::Kaun1,
                Color::Kok1,
            ),
            (
                &RKAUN,
                AbsoluteColumn::C,
                AbsoluteRow::A,
                Profession::Kaun1,
                Color::Kok1,
            ),
            (
                &BDAU,
                AbsoluteColumn::X,
                AbsoluteRow::E,
                Profession::Dau2,
                Color::Huok2,
            ),
            (
                &BDAU,
                AbsoluteColumn::T,
                AbsoluteRow::AU,
                Profession::Dau2,
                Color::Huok2,
            ),
            (
                &RDAU,
                AbsoluteColumn::T,
                AbsoluteRow::E,
                Profession::Dau2,
                Color::Kok1,
            ),
            (
                &RDAU,
                AbsoluteColumn::X,
                AbsoluteRow::AU,
                Profession::Dau2,
                Color::Kok1,
            ),
            (
                &BMAUN,
                AbsoluteColumn::L,
                AbsoluteRow::A,
                Profession::Maun1,
                Color::Huok2,
            ),
            (
                &BMAUN,
                AbsoluteColumn::M,
                AbsoluteRow::IA,
                Profession::Maun1,
                Color::Huok2,
            ),
            (
                &RMAUN,
                AbsoluteColumn::M,
                AbsoluteRow::A,
                Profession::Maun1,
                Color::Kok1,
            ),
            (
                &RMAUN,
                AbsoluteColumn::L,
                AbsoluteRow::IA,
                Profession::Maun1,
                Color::Kok1,
            ),
            (
                &BKUA,
                AbsoluteColumn::P,
                AbsoluteRow::IA,
                Profession::Kua2,
                Color::Huok2,
            ),
            (
                &BKUA,
                AbsoluteColumn::K,
                AbsoluteRow::A,
                Profession::Kua2,
                Color::Huok2,
            ),
            (
                &RKUA,
                AbsoluteColumn::P,
                AbsoluteRow::A,
                Profession::Kua2,
                Color::Kok1,
            ),
            (
                &RKUA,
                AbsoluteColumn::K,
                AbsoluteRow::IA,
                Profession::Kua2,
                Color::Kok1,
            ),
            (
                &BTUK,
                AbsoluteColumn::P,
                AbsoluteRow::E,
                Profession::Tuk2,
                Color::Huok2,
            ),
            (
                &BTUK,
                AbsoluteColumn::K,
                AbsoluteRow::AU,
                Profession::Tuk2,
                Color::Huok2,
            ),
            (
                &RTUK,
                AbsoluteColumn::K,
                AbsoluteRow::E,
                Profession::Tuk2,
                Color::Kok1,
            ),
            (
                &RTUK,
                AbsoluteColumn::P,
                AbsoluteRow::AU,
                Profession::Tuk2,
                Color::Kok1,
            ),
            (
                &BUAI,
                AbsoluteColumn::T,
                AbsoluteRow::A,
                Profession::Uai1,
                Color::Huok2,
            ),
            (
                &BUAI,
                AbsoluteColumn::X,
                AbsoluteRow::IA,
                Profession::Uai1,
                Color::Huok2,
            ),
            (
                &RUAI,
                AbsoluteColumn::X,
                AbsoluteRow::A,
                Profession::Uai1,
                Color::Kok1,
            ),
            (
                &RUAI,
                AbsoluteColumn::T,
                AbsoluteRow::IA,
                Profession::Uai1,
                Color::Kok1,
            ),
            (
                &BIO,
                AbsoluteColumn::Z,
                AbsoluteRow::IA,
                Profession::Io,
                Color::Huok2,
            ),
            (
                &RIO,
                AbsoluteColumn::Z,
                AbsoluteRow::A,
                Profession::Io,
                Color::Huok2,
            ),
        ] {
            let char_image = load_from_80x80(character, piece_dimension);

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
                    if row == AbsoluteRow::A || row == AbsoluteRow::E || row == AbsoluteRow::I {
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
            background: background::background_img(piece_dimension as f32 * 1.25),
            piece_dimension
        };

        board
    }
}
