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
            Piece::Tam2(pt) => pt.image.clone(),
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
pub enum Row {
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
pub enum Column {
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

type Coord = (Row, Column);

use std::collections::HashMap;

pub struct Field {
    field: HashMap<Coord, Piece>,
    a_side_hand: Vec<PhysicalPiece>,
    ia_side_hand: Vec<PhysicalPiece>,
    background: image::RgbImage,
    piece_dimension: u32,
}

mod background;
mod noise;

fn load_from_80x80(data: &'static [u8], dimension: u32) -> image::RgbImage {
    let image = image::load_from_memory(data).unwrap().to_rgb();
    if dimension == 80 {
        image
    } else {
        image::imageops::resize(
            &image,
            dimension,
            dimension,
            image::imageops::FilterType::CatmullRom,
        )
    }
}

impl Field {
    pub fn render(&self) -> image::RgbImage {
        use crate::image::GenericImage;
        let mut background = self.background.clone();
        let (width, height) = background.dimensions();

        for (x, y, pixel) in self.field[&(Row::O, Column::Z)].image().enumerate_pixels() {
            background
                .sub_image(
                    width / 2 - self.piece_dimension / 2,
                    height / 2 - self.piece_dimension / 2,
                    self.piece_dimension,
                    self.piece_dimension,
                )
                .put_pixel(x, y, *pixel);
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
        hashmap.insert((Row::O, Column::Z), Piece::Tam2(physical_tam));

        for (character, col, row, profession, color) in vec![
            (&BNUAK, Column::Z, Row::AI, Profession::Nuak1, Color::Huok2),
            (&RNUAK, Column::Z, Row::I, Profession::Nuak1, Color::Kok1),
            (&BKAUK, Column::K, Row::I, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::N, Row::I, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::C, Row::I, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::P, Row::I, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::K, Row::AI, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::N, Row::AI, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::C, Row::AI, Profession::Kauk2, Color::Huok2),
            (&BKAUK, Column::P, Row::AI, Profession::Kauk2, Color::Huok2),
            (&RKAUK, Column::L, Row::I, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::T, Row::I, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::X, Row::I, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::M, Row::I, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::L, Row::AI, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::T, Row::AI, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::X, Row::AI, Profession::Kauk2, Color::Kok1),
            (&RKAUK, Column::M, Row::AI, Profession::Kauk2, Color::Kok1),
            (&BGUA, Column::L, Row::AU, Profession::Gua2, Color::Huok2),
            (&BGUA, Column::M, Row::E, Profession::Gua2, Color::Huok2),
            (&RGUA, Column::L, Row::E, Profession::Gua2, Color::Kok1),
            (&RGUA, Column::M, Row::AU, Profession::Gua2, Color::Kok1),
            (&BKAUN, Column::N, Row::A, Profession::Kaun1, Color::Huok2),
            (&BKAUN, Column::C, Row::IA, Profession::Kaun1, Color::Huok2),
            (&RKAUN, Column::N, Row::IA, Profession::Kaun1, Color::Kok1),
            (&RKAUN, Column::C, Row::A, Profession::Kaun1, Color::Kok1),
            (&BDAU, Column::X, Row::E, Profession::Dau2, Color::Huok2),
            (&BDAU, Column::T, Row::AU, Profession::Dau2, Color::Huok2),
            (&RDAU, Column::T, Row::E, Profession::Dau2, Color::Kok1),
            (&RDAU, Column::X, Row::AU, Profession::Dau2, Color::Kok1),
            (&BMAUN, Column::L, Row::A, Profession::Maun1, Color::Huok2),
            (&BMAUN, Column::M, Row::IA, Profession::Maun1, Color::Huok2),
            (&RMAUN, Column::M, Row::A, Profession::Maun1, Color::Kok1),
            (&RMAUN, Column::L, Row::IA, Profession::Maun1, Color::Kok1),
            (&BKUA, Column::P, Row::IA, Profession::Kua2, Color::Huok2),
            (&BKUA, Column::K, Row::A, Profession::Kua2, Color::Huok2),
            (&RKUA, Column::P, Row::A, Profession::Kua2, Color::Kok1),
            (&RKUA, Column::K, Row::IA, Profession::Kua2, Color::Kok1),
            (&BTUK, Column::P, Row::E, Profession::Tuk2, Color::Huok2),
            (&BTUK, Column::K, Row::AU, Profession::Tuk2, Color::Huok2),
            (&RTUK, Column::K, Row::E, Profession::Tuk2, Color::Kok1),
            (&RTUK, Column::P, Row::AU, Profession::Tuk2, Color::Kok1),
            (&BUAI, Column::T, Row::A, Profession::Uai1, Color::Huok2),
            (&BUAI, Column::X, Row::IA, Profession::Uai1, Color::Huok2),
            (&RUAI, Column::X, Row::A, Profession::Uai1, Color::Kok1),
            (&RUAI, Column::T, Row::IA, Profession::Uai1, Color::Kok1),
            (&BIO, Column::Z, Row::IA, Profession::Io, Color::Huok2),
            (&RIO, Column::Z, Row::A, Profession::Io, Color::Huok2),
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
                    if row == Row::A || row == Row::E || row == Row::I {
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
            piece_dimension,
        };

        board
    }
}
