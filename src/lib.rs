#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_wrap,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]

extern crate image;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::{Color, Column, Field, Profession, Row, Side};
        let mut field = Field::new();
        field.render(Side::IASide).save("a.png").unwrap();
        field.render(Side::ASide).save("b.png").unwrap();

        field
            .move_to_opponent_hop1zuo1((Row::A, Column::K))
            .unwrap();

        field.render(Side::IASide).save("a2.png").unwrap();
        field.render(Side::ASide).save("b2.png").unwrap();

        field
            .move_to_empty_square((Row::A, Column::K), (Row::A, Column::L))
            .unwrap();

        field.render(Side::IASide).save("a3.png").unwrap();
        field.render(Side::ASide).save("b3.png").unwrap();

        field
            .step_on_occupied((Row::A, Column::P), (Row::A, Column::M))
            .unwrap();

        field.render(Side::IASide).save("a4.png").unwrap();
        field.render(Side::ASide).save("b4.png").unwrap();

        field.relocate_stepping((Row::O, Column::Z)).unwrap();

        field.render(Side::IASide).save("a5.png").unwrap();
        field.render(Side::ASide).save("b5.png").unwrap();

        field.descend_from_stepping((Row::O, Column::C)).unwrap();

        field.render(Side::IASide).save("a6.png").unwrap();
        field.render(Side::ASide).save("b6.png").unwrap();

        field
            .from_hop1zuo1(
                (Row::O, Column::M),
                Side::IASide,
                Color::Huok2,
                Profession::Kua2,
            )
            .unwrap();

        field.render(Side::IASide).save("a7.png").unwrap();
        field.render(Side::ASide).save("b7.png").unwrap();
    }
}

const BNUAK: &[u8] = include_bytes!("bnuak.png_80x80.png");
const BKAUK: &[u8] = include_bytes!("bkauk.png_80x80.png");
const BKAUN: &[u8] = include_bytes!("bkaun.png_80x80.png");
const BMAUN: &[u8] = include_bytes!("bmaun.png_80x80.png");
const BKUA: &[u8] = include_bytes!("bkua.png_80x80.png");
const BGUA: &[u8] = include_bytes!("bgua.png_80x80.png");
const BTAM: &[u8] = include_bytes!("btam.png_80x80.png");
const BTUK: &[u8] = include_bytes!("btuk.png_80x80.png");
const BDAU: &[u8] = include_bytes!("bdau.png_80x80.png");
const BIO: &[u8] = include_bytes!("bio.png_80x80.png");
const BUAI: &[u8] = include_bytes!("buai.png_80x80.png");

const RNUAK: &[u8] = include_bytes!("rnuak.png_80x80.png");
const RKAUK: &[u8] = include_bytes!("rkauk.png_80x80.png");
const RKAUN: &[u8] = include_bytes!("rkaun.png_80x80.png");
const RMAUN: &[u8] = include_bytes!("rmaun.png_80x80.png");
const RKUA: &[u8] = include_bytes!("rkua.png_80x80.png");
const RGUA: &[u8] = include_bytes!("rgua.png_80x80.png");
const RTUK: &[u8] = include_bytes!("rtuk.png_80x80.png");
const RDAU: &[u8] = include_bytes!("rdau.png_80x80.png");
const RIO: &[u8] = include_bytes!("rio.png_80x80.png");
const RUAI: &[u8] = include_bytes!("ruai.png_80x80.png");

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Color {
    /// Red, 赤
    Kok1,

    /// Black, 黒
    Huok2,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Profession {
    /// Vessel, 船, felkana
    Nuak1,

    /// Pawn, 兵, elmer
    Kauk2,

    /// Rook, 弓, gustuer
    Gua2,

    /// Bishop, 車, vadyrd
    Kaun1,

    /// Tiger, 虎, stistyst
    Dau2,

    /// Horse, 馬, dodor
    Maun1,

    /// Clerk, 筆, kua
    Kua2,

    /// Shaman, 巫, terlsk
    Tuk2,

    /// General, 将, varxle
    Uai1,

    /// King, 王, ales
    Io,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Side {
    ASide,
    IASide,
}

#[derive(Debug, Copy, Clone)]
pub struct LogicalNonTam2Piece {
    pub color: Color,
    pub profession: Profession,
}

impl PhysicalNonTam2Piece {
    pub fn as_logical(&self) -> LogicalNonTam2Piece {
        LogicalNonTam2Piece {
            color: self.color,
            profession: self.profession,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LogicalTam {}

impl PhysicalTam {
    pub fn as_logical(&self) -> LogicalTam {
        LogicalTam {}
    }
}

#[derive(Debug, Copy, Clone)]
pub enum LogicalPieceOnField {
    NonTam2(LogicalNonTam2Piece, Side),
    Tam2(LogicalTam),
}

impl PieceOnField {
    pub fn as_logical(&self) -> LogicalPieceOnField {
        match self {
            PieceOnField::NonTam2(p, s) => LogicalPieceOnField::NonTam2(p.as_logical(), *s),
            PieceOnField::Tam2(p) => LogicalPieceOnField::Tam2(p.as_logical()),
        }
    }
}

struct PhysicalNonTam2Piece {
    color: Color,
    profession: Profession,
    image: image::RgbImage,
}

struct PhysicalTam {
    image: image::RgbImage,
}

enum PieceOnField {
    NonTam2(PhysicalNonTam2Piece, Side),
    Tam2(PhysicalTam),
}

impl PieceOnField {
    fn image(&self) -> image::RgbImage {
        match self {
            PieceOnField::NonTam2(pp, _) => pp.image.clone(),
            PieceOnField::Tam2(pt) => pt.image.clone(),
        }
    }

    fn physical_side(&self) -> Side {
        match self {
            PieceOnField::NonTam2(_, s) => *s,
            PieceOnField::Tam2(_) => Side::IASide,
        }
    }

    fn into_nontam2piece(self) -> Option<(PhysicalNonTam2Piece, Side)> {
        match self {
            PieceOnField::NonTam2(p, s) => Some((p, s)),
            PieceOnField::Tam2(_) => None,
        }
    }

    fn is_tam2(&self) -> bool {
        match self {
            PieceOnField::NonTam2(_, _) => false,
            PieceOnField::Tam2(_) => true,
        }
    }
}

fn multiply_channel(a: u8, b: u8) -> u8 {
    (f32::from(a) * f32::from(b) / 255.0) as u8
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    field: HashMap<Coord, PieceOnField>,
    a_side_hop1zuo1: Vec<PhysicalNonTam2Piece>,
    ia_side_hop1zuo1: Vec<PhysicalNonTam2Piece>,
    background: image::RgbImage,
    piece_dimension: u32,
    square_dimension: u32,
    floating: Option<(Coord, PieceOnField)>,
    focus: HashMap<Coord, bool /* whether floating */>,
    a_side_focus_index: Option<usize>,
    ia_side_focus_index: Option<usize>,
}

#[derive(Clone)]
pub struct LogicalField {
    pub field: HashMap<Coord, LogicalPieceOnField>,
    pub a_side_hop1zuo1: Vec<LogicalNonTam2Piece>,
    pub ia_side_hop1zuo1: Vec<LogicalNonTam2Piece>,
    pub floating: Option<(Coord, LogicalPieceOnField)>,
}

impl Field {
    #[must_use]
    pub fn as_logical(&self) -> LogicalField {
        LogicalField {
            field: self
                .field
                .iter()
                .map(|(k, v)| (*k, v.as_logical()))
                .collect(),
            a_side_hop1zuo1: self
                .a_side_hop1zuo1
                .iter()
                .map(PhysicalNonTam2Piece::as_logical)
                .collect(),
            ia_side_hop1zuo1: self
                .ia_side_hop1zuo1
                .iter()
                .map(PhysicalNonTam2Piece::as_logical)
                .collect(),
            floating: match &self.floating {
                None => None,
                Some((c, p)) => Some((*c, p.as_logical())),
            },
        }
    }
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

#[derive(Debug)]
pub enum OperationError {
    MovingFromEmptySquare,
    MovingToNonEmptySquare,
    SteppingOnEmptySquare,
    Tam2ToHop1Zuo1,
    TwoPiecesOnFlight,
    NoPieceOnFlight,
    NoMatchingColorOrProfessionInHop1Zuo1,
    ParachutingToNonEmptySquare,
}

fn get_horiz_offset_from_coord(coord: Coord, down_side: Side) -> i32 {
    let (_, col) = coord;
    (match col {
        Column::K => -4,
        Column::L => -3,
        Column::N => -2,
        Column::T => -1,
        Column::Z => 0,
        Column::X => 1,
        Column::C => 2,
        Column::M => 3,
        Column::P => 4,
    }) * (match down_side {
        Side::IASide => 1,
        Side::ASide => -1,
    })
}

fn get_vert_offset_from_coord(coord: Coord, down_side: Side) -> i32 {
    let (row, _) = coord;
    (match row {
        Row::A => -4,
        Row::E => -3,
        Row::I => -2,
        Row::U => -1,
        Row::O => 0,
        Row::Y => 1,
        Row::AI => 2,
        Row::AU => 3,
        Row::IA => 4,
    }) * (match down_side {
        Side::IASide => 1,
        Side::ASide => -1,
    })
}

impl Default for Field {
    fn default() -> Self {
        Self::new()
    }
}

use crate::image::GenericImage;

impl Field {
    const INITIAL_BOARD: [(&'static &'static [u8], Column, Row, Profession, Color); 48] = [
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
    ];

    fn place_img_on_subimg_regarding_side(
        &self,
        down_side: Side,
        side_to_be_compared_against: Side,
        image: &image::RgbImage,
        sub_image: &mut image::SubImage<&mut image::RgbImage>,
    ) {
        for (x, y, pixel) in image.enumerate_pixels() {
            sub_image.put_pixel(
                if down_side == side_to_be_compared_against {
                    x
                } else {
                    self.piece_dimension - x
                },
                if down_side == side_to_be_compared_against {
                    y
                } else {
                    self.piece_dimension - y
                },
                *pixel,
            );
        }
    }

    fn put_border_on_sub_image(
        &self,
        sub_image: &mut image::SubImage<&mut image::RgbImage>,
        weight: u32,
    ) {
        for x in 0..self.piece_dimension {
            for y in 0..self.piece_dimension {
                if x < weight /* FIXME: not scale invariant */
                || y < weight
                || x >= self.piece_dimension - weight
                || y >= self.piece_dimension - weight
                {
                    sub_image.put_pixel(x, y, image::Rgb([0xff, 0x1d, 0x62]));
                }
            }
        }
    }

    fn debug_assert_49_piece(&self) {
        debug_assert_eq!(
            self.field.len()
                + self.a_side_hop1zuo1.len()
                + self.ia_side_hop1zuo1.len()
                + if self.floating.is_some() { 1 } else { 0 },
            49
        );
    }

    pub fn delete_focus(&mut self) {
        self.focus = HashMap::new();
        self.ia_side_focus_index = None;
        self.a_side_focus_index = None;
    }

    /// # Errors
    ///
    /// Will return `Err` if either:
    /// * `coord` is already occupied
    /// * the `side`'s hop1zuo1 does not contain the piece specified by the `color` and `profession`
    pub fn from_hop1zuo1(
        &mut self,
        coord: Coord,
        side: Side,
        color: Color,
        profession: Profession,
    ) -> Result<(), OperationError> {
        self.debug_assert_49_piece();

        if self.field.contains_key(&coord) {
            return Err(OperationError::ParachutingToNonEmptySquare);
        }

        self.delete_focus();

        let nontam2piece = if side == Side::ASide {
            let ind = self
                .a_side_hop1zuo1
                .iter()
                .position(|p| p.color == color && p.profession == profession)
                .ok_or(OperationError::NoMatchingColorOrProfessionInHop1Zuo1)?;

            self.a_side_focus_index = Some(self.a_side_hop1zuo1.len() - 1);

            self.a_side_hop1zuo1.swap_remove(ind)
        } else {
            let ind = self
                .ia_side_hop1zuo1
                .iter()
                .position(|p| p.color == color && p.profession == profession)
                .ok_or(OperationError::NoMatchingColorOrProfessionInHop1Zuo1)?;

            self.ia_side_focus_index = Some(self.ia_side_hop1zuo1.len() - 1);

            self.ia_side_hop1zuo1.swap_remove(ind)
        };

        self.field
            .insert(coord, PieceOnField::NonTam2(nontam2piece, side));
        self.focus.insert(coord, false);

        self.debug_assert_49_piece();
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if either:
    /// * `coord` is empty
    /// * `coord` is occupied by a Tam2
    pub fn move_to_opponent_hop1zuo1(&mut self, coord: Coord) -> Result<(), OperationError> {
        self.debug_assert_49_piece();

        if !self.field.contains_key(&coord) {
            return Err(OperationError::MovingFromEmptySquare);
        }

        if self.field[&coord].is_tam2() {
            return Err(OperationError::Tam2ToHop1Zuo1);
        }

        let (nontam2piece, side) = self
            .field
            .remove(&coord)
            .ok_or(OperationError::MovingFromEmptySquare)?
            .into_nontam2piece()
            .ok_or(OperationError::Tam2ToHop1Zuo1)?;

        if side == Side::ASide {
            self.delete_focus();
            self.focus.insert(coord, false);
            self.ia_side_focus_index = Some(self.ia_side_hop1zuo1.len());
            self.ia_side_hop1zuo1.push(nontam2piece);
        } else {
            self.delete_focus();
            self.focus.insert(coord, false);
            self.a_side_focus_index = Some(self.a_side_hop1zuo1.len());
            self.a_side_hop1zuo1.push(nontam2piece);
        }

        self.debug_assert_49_piece();
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if either:
    /// * `from` is empty
    /// * `to` is already occupied
    pub fn move_to_empty_square(&mut self, to: Coord, from: Coord) -> Result<(), OperationError> {
        self.debug_assert_49_piece();

        if !self.field.contains_key(&from) {
            return Err(OperationError::MovingFromEmptySquare);
        }

        if self.field.contains_key(&to) {
            return Err(OperationError::MovingToNonEmptySquare);
        }

        let piece = self
            .field
            .remove(&from)
            .ok_or(OperationError::MovingFromEmptySquare)?;

        self.field.insert(to, piece);

        self.delete_focus();
        self.focus.insert(from, false);
        self.focus.insert(to, false);

        self.debug_assert_49_piece();
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if no piece is floating
    pub fn relocate_stepping(&mut self, to: Coord) -> Result<(), OperationError> {
        self.debug_assert_49_piece();
        let (from, piece) = self
            .floating
            .take()
            .ok_or(OperationError::NoPieceOnFlight)?;

        self.floating = Some((to, piece));

        self.delete_focus();
        self.focus.insert(from, true);
        self.focus.insert(to, true);

        self.debug_assert_49_piece();
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if `to` is already occupied.
    pub fn descend_from_stepping(&mut self, to: Coord) -> Result<(), OperationError> {
        self.debug_assert_49_piece();
        let (from, piece) = self
            .floating
            .take()
            .ok_or(OperationError::NoPieceOnFlight)?;

        if self.field.contains_key(&to) {
            return Err(OperationError::MovingToNonEmptySquare);
        }

        self.field.insert(to, piece);

        self.delete_focus();
        self.focus.insert(from, true);
        self.focus.insert(to, false);

        self.debug_assert_49_piece();
        Ok(())
    }

    /// # Errors
    ///
    /// Will return `Err` if either :
    /// * the `from` is an empty square
    /// * the `to` is an empty square
    /// * `self.floating` is already occupied
    pub fn step_on_occupied(&mut self, to: Coord, from: Coord) -> Result<(), OperationError> {
        self.debug_assert_49_piece();

        if !self.field.contains_key(&from) {
            return Err(OperationError::MovingFromEmptySquare);
        }

        if !self.field.contains_key(&to) {
            return Err(OperationError::SteppingOnEmptySquare);
        }

        if self.floating.is_some() {
            return Err(OperationError::TwoPiecesOnFlight);
        }

        let piece = self
            .field
            .remove(&from)
            .ok_or(OperationError::MovingFromEmptySquare)?;

        self.floating = Some((to, piece));

        self.delete_focus();
        self.focus.insert(from, false);
        self.focus.insert(to, true);

        self.debug_assert_49_piece();
        Ok(())
    }

    fn render_a_side_hop1zuo1(&self, background: &mut image::RgbImage, down_side: Side) {
        let one_if_ia_is_down: i32 = match down_side {
            Side::IASide => 1,
            Side::ASide => -1,
        };

        let (width, height) = background.dimensions();
        let mut i: usize = 0;
        for p in &self.a_side_hop1zuo1 {
            let vert_offset = (6 + (i / 9)) as i32 * -one_if_ia_is_down;
            let horiz_offset = ((i % 9) as i32 - 4) * -one_if_ia_is_down;

            let mut sub_image = background.sub_image(
                ((width / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * horiz_offset) as u32,
                ((height / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * vert_offset) as u32,
                self.piece_dimension,
                self.piece_dimension,
            );

            self.place_img_on_subimg_regarding_side(
                down_side,
                Side::ASide,
                &p.image,
                &mut sub_image,
            );

            if Some(i) == self.a_side_focus_index {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }

            i += 1;
        }

        /* when placed from hop1 zuo1, the focus_index should be out of bound */
        {
            let vert_offset = (6 + (i / 9)) as i32 * -one_if_ia_is_down;
            let horiz_offset = ((i % 9) as i32 - 4) * -one_if_ia_is_down;

            let mut sub_image = background.sub_image(
                ((width / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * horiz_offset) as u32,
                ((height / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * vert_offset) as u32,
                self.piece_dimension,
                self.piece_dimension,
            );

            if Some(i) == self.a_side_focus_index {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }
        }
    }

    fn render_ia_side_hop1zuo1(&self, mut background: &mut image::RgbImage, down_side: Side) {
        let one_if_ia_is_down: i32 = match down_side {
            Side::IASide => 1,
            Side::ASide => -1,
        };

        let (width, height) = background.dimensions();

        let mut i: usize = 0;
        for p in &self.ia_side_hop1zuo1 {
            let vert_offset = (6 + (i / 9)) as i32 * one_if_ia_is_down;
            let horiz_offset = ((i % 9) as i32 - 4) * one_if_ia_is_down;

            let mut sub_image = background.sub_image(
                ((width / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * horiz_offset) as u32,
                ((height / 2 - self.piece_dimension / 2) as i32
                    + self.square_dimension as i32 * vert_offset) as u32,
                self.piece_dimension,
                self.piece_dimension,
            );

            self.place_img_on_subimg_regarding_side(
                down_side,
                Side::IASide,
                &p.image,
                &mut sub_image,
            );

            if Some(i) == self.ia_side_focus_index {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }

            i += 1;
        }

        /* when placed from hop1 zuo1, the focus_index should be out of bound */
        {
            let vert_offset = (6 + (i / 9)) as i32 * one_if_ia_is_down;
            let horiz_offset = ((i % 9) as i32 - 4) * one_if_ia_is_down;

            let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                &mut background,
                horiz_offset,
                vert_offset,
            );

            if Some(i) == self.ia_side_focus_index {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }
        }
    }

    fn get_subimage_from_horiz_vert_offset<'a>(
        &self,
        background: &'a mut image::RgbImage,
        horiz_offset: i32,
        vert_offset: i32,
    ) -> image::SubImage<&'a mut image::RgbImage> {
        let (width, height) = background.dimensions();
        background.sub_image(
            ((width / 2 - self.piece_dimension / 2) as i32
                + self.square_dimension as i32 * horiz_offset) as u32,
            ((height / 2 - self.piece_dimension / 2) as i32
                + self.square_dimension as i32 * vert_offset) as u32,
            self.piece_dimension,
            self.piece_dimension,
        )
    }

    fn render_main_field(&self, mut background: &mut image::RgbImage, down_side: Side) {
        for (row, col) in self.field.keys() {
            let horiz_offset = get_horiz_offset_from_coord((*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord((*row, *col), down_side);
            let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                &mut background,
                horiz_offset,
                vert_offset,
            );
            self.place_img_on_subimg_regarding_side(
                down_side,
                self.field[&(*row, *col)].physical_side(),
                &self.field[&(*row, *col)].image(),
                &mut sub_image,
            );
        }
    }

    #[must_use]
    pub fn render(&self, down_side: Side) -> image::RgbImage {
        let mut background = if down_side == Side::IASide {
            self.background.clone()
        } else {
            image::imageops::rotate180(&self.background)
        };
        let (width, height) = background.dimensions();

        let one_if_ia_is_down: i32 = match down_side {
            Side::IASide => 1,
            Side::ASide => -1,
        };

        // render the pieces
        self.render_a_side_hop1zuo1(&mut background, down_side);
        self.render_ia_side_hop1zuo1(&mut background, down_side);
        self.render_main_field(&mut background, down_side);

        // then render the focuses
        for (row, col) in self.focus.keys() {
            let horiz_offset = get_horiz_offset_from_coord((*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord((*row, *col), down_side);
            if !self.focus[&(*row, *col)]
            /* not floating */
            {
                let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                    &mut background,
                    horiz_offset,
                    vert_offset,
                );
                self.put_border_on_sub_image(&mut sub_image, 9);
            } else if let Some(((row2, col2), piece)) = &self.floating {
                // if equal, handle later
                if (row2, col2) != (&*row, &*col) {
                    let mut sub_image = background.sub_image(
                        ((width / 2 - self.piece_dimension / 2) as i32
                            - (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                                * if piece.physical_side() == down_side {
                                    1
                                } else {
                                    -1
                                }
                            + self.square_dimension as i32 * horiz_offset)
                            as u32,
                        ((height / 2 - self.piece_dimension / 2) as i32
                            + (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                                * if piece.physical_side() == down_side {
                                    1
                                } else {
                                    -1
                                }
                            + self.square_dimension as i32 * vert_offset)
                            as u32,
                        self.piece_dimension,
                        self.piece_dimension,
                    );
                    self.put_border_on_sub_image(&mut sub_image, 6);
                }
            } else {
                let mut sub_image = background.sub_image(
                    ((width / 2 - self.piece_dimension / 2) as i32
                        - (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                            * one_if_ia_is_down  /* strictly speaking not accurate, but fine */
                        + self.square_dimension as i32 * horiz_offset) as u32,
                    ((height / 2 - self.piece_dimension / 2) as i32
                        + (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                            * one_if_ia_is_down  /* strictly speaking not accurate, but fine */
                        + self.square_dimension as i32 * vert_offset) as u32,
                    self.piece_dimension,
                    self.piece_dimension,
                );
                self.put_border_on_sub_image(&mut sub_image, 6);
            }
        }

        if let Some(((row, col), piece)) = &self.floating {
            let horiz_offset = get_horiz_offset_from_coord((*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord((*row, *col), down_side);
            let mut sub_image = background.sub_image(
                ((width / 2 - self.piece_dimension / 2) as i32
                    - (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                        * if piece.physical_side() == down_side {
                            1
                        } else {
                            -1
                        }
                    + self.square_dimension as i32 * horiz_offset) as u32,
                ((height / 2 - self.piece_dimension / 2) as i32
                    + (self.square_dimension as i32 - self.piece_dimension as i32) / 2
                        * if piece.physical_side() == down_side {
                            1
                        } else {
                            -1
                        }
                    + self.square_dimension as i32 * vert_offset) as u32,
                self.piece_dimension,
                self.piece_dimension,
            );

            self.place_img_on_subimg_regarding_side(
                down_side,
                piece.physical_side(),
                &piece.image(),
                &mut sub_image,
            );

            if self.focus.contains_key(&(*row, *col)) {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }
        }

        background
    }

    #[must_use]
    pub fn new() -> Field {
        use rand::seq::SliceRandom;

        let piece_dimension = 80;
        let padding = 4;

        let raw_wood = image::imageops::colorops::brighten(
            &noise::rawwood(
                (piece_dimension + padding) * 6 + piece_dimension,
                (piece_dimension + padding) * 7 + piece_dimension,
                f64::from(piece_dimension) / 2.,
            ),
            20,
        );

        //raw_wood.save("rawwood.png").unwrap();

        let mut pieces = Vec::new();

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
        //res.save(format!("rawwood_{}.png", i)).unwrap();
        i += 1;

        let physical_tam = PhysicalTam { image: res };

        let mut hashmap = HashMap::new();
        hashmap.insert((Row::O, Column::Z), PieceOnField::Tam2(physical_tam));

        for (character, col, row, profession, color) in Field::INITIAL_BOARD.iter() {
            let char_image = load_from_80x80(character, piece_dimension);

            let res = multiply_image(&char_image, &pieces[i]).unwrap();
            // res.save(format!("rawwood_{}.png", i)).unwrap();

            hashmap.insert(
                (*row, *col),
                PieceOnField::NonTam2(
                    PhysicalNonTam2Piece {
                        color: *color,
                        profession: *profession,
                        image: res,
                    },
                    if *row == Row::A || *row == Row::E || *row == Row::I {
                        Side::ASide
                    } else {
                        Side::IASide
                    },
                ),
            );

            i += 1;
        }

        Field {
            a_side_hop1zuo1: Vec::new(),
            ia_side_hop1zuo1: Vec::new(),
            field: hashmap,
            background: background::gen_bg(piece_dimension as f32 * 1.25),
            piece_dimension,
            square_dimension: (piece_dimension as f32 * 1.25) as u32,
            floating: None,
            focus: HashMap::new(),
            a_side_focus_index: None,
            ia_side_focus_index: None,
        }
    }
}
