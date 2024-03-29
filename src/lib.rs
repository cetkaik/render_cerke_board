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
    fn test2() {
        use super::{Field, AbsoluteSide};
        let field = Field::new(160, 8, 24.66);
        field.render(AbsoluteSide::IASide).save("c.png").unwrap();
    }

    #[test]
    fn test() {
        use super::{Color, Column, Coord, Field, Profession, Row, AbsoluteSide};
        let mut field = Field::new(80, 4, 12.33);
        field.render(AbsoluteSide::IASide).save("a.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b.png").unwrap();

        field
            .move_to_opponent_hop1zuo1(Coord(Row::A, Column::K))
            .unwrap();

        field.render(AbsoluteSide::IASide).save("a2.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b2.png").unwrap();

        field
            .move_to_empty_square(Coord(Row::A, Column::K), Coord(Row::A, Column::L))
            .unwrap();

        field.render(AbsoluteSide::IASide).save("a3.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b3.png").unwrap();

        field
            .step_on_occupied(Coord(Row::A, Column::P), Coord(Row::A, Column::M))
            .unwrap();

        field.render(AbsoluteSide::IASide).save("a4.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b4.png").unwrap();

        field.relocate_stepping(Coord(Row::O, Column::Z)).unwrap();

        field.render(AbsoluteSide::IASide).save("a5.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b5.png").unwrap();

        field
            .descend_from_stepping(Coord(Row::O, Column::C))
            .unwrap();

        field.render(AbsoluteSide::IASide).save("a6.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b6.png").unwrap();

        field
            .place_from_hop1zuo1(
                Coord(Row::O, Column::M),
                AbsoluteSide::IASide,
                Color::Huok2,
                Profession::Kua2,
            )
            .unwrap();

        field.render(AbsoluteSide::IASide).save("a7.png").unwrap();
        field.render(AbsoluteSide::ASide).save("b7.png").unwrap();
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

use cetkaik_naive_representation::absolute::{Column, Coord, Row};
use cetkaik_fundamental::{Color, Profession, AbsoluteSide};
type LogicalNonTam2Piece = cetkaik_fundamental::ColorAndProf;

impl PhysicalNonTam2Piece {
    pub fn as_logical(&self) -> LogicalNonTam2Piece {
        LogicalNonTam2Piece {
            color: self.color,
            prof: self.profession,
        }
    }
}

type LogicalPieceOnField = cetkaik_naive_representation::absolute::Piece;

impl PhysicalPieceOnField {
    pub fn as_logical(&self) -> LogicalPieceOnField {
        match self {
            PhysicalPieceOnField::NonTam2(p, s) => LogicalPieceOnField::NonTam2Piece {
                side: *s,
                prof: p.as_logical().prof,
                color: p.as_logical().color,
            },
            PhysicalPieceOnField::Tam2(_) => LogicalPieceOnField::Tam2,
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

enum PhysicalPieceOnField {
    NonTam2(PhysicalNonTam2Piece, AbsoluteSide),
    Tam2(PhysicalTam),
}

impl PhysicalPieceOnField {
    fn image(&self) -> image::RgbImage {
        match self {
            PhysicalPieceOnField::NonTam2(pp, _) => pp.image.clone(),
            PhysicalPieceOnField::Tam2(pt) => pt.image.clone(),
        }
    }

    fn physical_side(&self) -> AbsoluteSide {
        match self {
            PhysicalPieceOnField::NonTam2(_, s) => *s,
            PhysicalPieceOnField::Tam2(_) => AbsoluteSide::IASide,
        }
    }

    fn into_nontam2piece(self) -> Option<(PhysicalNonTam2Piece, AbsoluteSide)> {
        match self {
            PhysicalPieceOnField::NonTam2(p, s) => Some((p, s)),
            PhysicalPieceOnField::Tam2(_) => None,
        }
    }

    fn is_tam2(&self) -> bool {
        match self {
            PhysicalPieceOnField::NonTam2(_, _) => false,
            PhysicalPieceOnField::Tam2(_) => true,
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
        *pixel = multiply_pixel(*a.get_pixel(x, y), *b.get_pixel(x, y));
    }
    Some(c)
}

use std::collections::HashMap;

pub struct Field {
    field: HashMap<Coord, PhysicalPieceOnField>,
    a_side_hop1zuo1: Vec<PhysicalNonTam2Piece>,
    ia_side_hop1zuo1: Vec<PhysicalNonTam2Piece>,
    background: image::RgbImage,
    piece_dimension: u32,
    square_dimension: u32,
    floating: Option<(Coord, PhysicalPieceOnField)>,
    focus: HashMap<Coord, bool /* whether floating */>,
    a_side_focus_index: Option<usize>,
    ia_side_focus_index: Option<usize>,
}

pub struct LogicalField {
    pub f: cetkaik_naive_representation::absolute::Field,
    pub floating: Option<(Coord, LogicalPieceOnField)>,
}

impl Field {
    #[must_use]
    pub fn to_logical(&self) -> LogicalField {
        LogicalField {
            f: cetkaik_naive_representation::absolute::Field {
                board: cetkaik_naive_representation::absolute::Board(self
                    .field
                    .iter()
                    .map(|(k, v)| (*k, v.as_logical()))
                    .collect()),
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
            },
            floating: self.floating.as_ref().map(|(c, p)| (*c, p.as_logical())),
        }
    }
}

mod background;

fn load_from_80x80(data: &'static [u8], dimension: u32) -> image::RgbImage {
    let image = image::load_from_memory(data).unwrap().to_rgb8();
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

fn get_horiz_offset_from_coord(coord: Coord, down_side: AbsoluteSide) -> i32 {
    let Coord(_, col) = coord;
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
        AbsoluteSide::IASide => 1,
        AbsoluteSide::ASide => -1,
    })
}

fn get_vert_offset_from_coord(coord: Coord, down_side: AbsoluteSide) -> i32 {
    let Coord(row, _) = coord;
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
        AbsoluteSide::IASide => 1,
        AbsoluteSide::ASide => -1,
    })
}

impl Default for Field {
    fn default() -> Self {
        Self::new(80, 4, 12.33)
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
        down_side: AbsoluteSide,
        side_to_be_compared_against: AbsoluteSide,
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
                + usize::from(self.floating.is_some()),
            49
        );
    }

    pub fn delete_focus(&mut self) {
        self.focus = HashMap::new();
        self.ia_side_focus_index = None;
        self.a_side_focus_index = None;
    }

    fn on_hop1zuo1<T, F>(&self, side: AbsoluteSide, f: F) -> T
    where
        F: FnOnce(&[PhysicalNonTam2Piece]) -> T,
    {
        if side == AbsoluteSide::ASide {
            f(&self.a_side_hop1zuo1)
        } else {
            f(&self.ia_side_hop1zuo1)
        }
    }

    fn on_hop1zuo1_mut<T, F>(&mut self, side: AbsoluteSide, f: F) -> T
    where
        F: FnOnce(&mut Vec<PhysicalNonTam2Piece>) -> T,
    {
        if side == AbsoluteSide::ASide {
            f(&mut self.a_side_hop1zuo1)
        } else {
            f(&mut self.ia_side_hop1zuo1)
        }
    }

    fn hop1zuo1_len(&self, side: AbsoluteSide) -> usize {
        if side == AbsoluteSide::ASide {
            self.a_side_hop1zuo1.len()
        } else {
            self.ia_side_hop1zuo1.len()
        }
    }

    fn set_hop1zuo1_focus_index(&mut self, side: AbsoluteSide, index: Option<usize>) {
        if side == AbsoluteSide::ASide {
            self.a_side_focus_index = index;
        } else {
            self.ia_side_focus_index = index;
        }
    }

    fn get_hop1zuo1_focus_index(&self, side: AbsoluteSide) -> Option<usize> {
        if side == AbsoluteSide::ASide {
            self.a_side_focus_index
        } else {
            self.ia_side_focus_index
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if either:
    /// * `coord` is already occupied
    /// * the `side`'s hop1zuo1 does not contain the piece specified by the `color` and `profession`
    pub fn place_from_hop1zuo1(
        &mut self,
        coord: Coord,
        side: AbsoluteSide,
        color: Color,
        profession: Profession,
    ) -> Result<(), OperationError> {
        self.debug_assert_49_piece();

        if self.field.contains_key(&coord) {
            return Err(OperationError::ParachutingToNonEmptySquare);
        }

        self.delete_focus();

        let ind = self.on_hop1zuo1(side, |v: &[PhysicalNonTam2Piece]| {
            v.iter()
                .position(|p| p.color == color && p.profession == profession)
                .ok_or(OperationError::NoMatchingColorOrProfessionInHop1Zuo1)
        })?;

        self.set_hop1zuo1_focus_index(side, Some(self.hop1zuo1_len(side) - 1));

        let nontam2piece = self.on_hop1zuo1_mut(side, |v| v.swap_remove(ind));

        self.field
            .insert(coord, PhysicalPieceOnField::NonTam2(nontam2piece, side));
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

        self.delete_focus();
        self.focus.insert(coord, false);
        self.set_hop1zuo1_focus_index(!side, Some(self.hop1zuo1_len(!side)));
        self.on_hop1zuo1_mut(!side, move |v| v.push(nontam2piece));

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

    fn render_one_side_hop1zuo1(
        &self,
        whose_hop1zuo1: AbsoluteSide,
        background: &mut image::RgbImage,
        down_side: AbsoluteSide,
    ) {
        let one_if_whose_hop1zuo1_is_down: i32 = if down_side == whose_hop1zuo1 { 1 } else { -1 };

        let mut i: usize = 0;
        self.on_hop1zuo1(whose_hop1zuo1, |v: &[PhysicalNonTam2Piece]| {
            for p in v {
                let vert_offset = (6 + (i / 9)) as i32 * one_if_whose_hop1zuo1_is_down;
                let horiz_offset = ((i % 9) as i32 - 4) * one_if_whose_hop1zuo1_is_down;

                let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                    background,
                    horiz_offset,
                    vert_offset,
                );

                self.place_img_on_subimg_regarding_side(
                    down_side,
                    whose_hop1zuo1,
                    &p.image,
                    &mut sub_image,
                );

                if Some(i) == self.get_hop1zuo1_focus_index(whose_hop1zuo1) {
                    self.put_border_on_sub_image(&mut sub_image, 9);
                }

                i += 1;
            }
        });

        /* when placed from hop1 zuo1, the focus_index should be out of bound */
        {
            let vert_offset = (6 + (i / 9)) as i32 * one_if_whose_hop1zuo1_is_down;
            let horiz_offset = ((i % 9) as i32 - 4) * one_if_whose_hop1zuo1_is_down;

            let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                background,
                horiz_offset,
                vert_offset,
            );

            if Some(i) == self.get_hop1zuo1_focus_index(whose_hop1zuo1) {
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

    fn render_main_field(&self, background: &mut image::RgbImage, down_side: AbsoluteSide) {
        for Coord(row, col) in self.field.keys() {
            let horiz_offset = get_horiz_offset_from_coord(Coord(*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord(Coord(*row, *col), down_side);
            let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                background,
                horiz_offset,
                vert_offset,
            );
            self.place_img_on_subimg_regarding_side(
                down_side,
                self.field[&Coord(*row, *col)].physical_side(),
                &self.field[&Coord(*row, *col)].image(),
                &mut sub_image,
            );
        }
    }

    #[must_use]
    pub fn render(&self, down_side: AbsoluteSide) -> image::RgbImage {
        let mut background = if down_side == AbsoluteSide::IASide {
            self.background.clone()
        } else {
            image::imageops::rotate180(&self.background)
        };
        let (width, height) = background.dimensions();

        let one_if_ia_is_down: i32 = match down_side {
            AbsoluteSide::IASide => 1,
            AbsoluteSide::ASide => -1,
        };

        // render the pieces
        self.render_one_side_hop1zuo1(AbsoluteSide::ASide, &mut background, down_side);
        self.render_one_side_hop1zuo1(AbsoluteSide::IASide, &mut background, down_side);
        self.render_main_field(&mut background, down_side);

        // then render the focuses
        for Coord(row, col) in self.focus.keys() {
            let horiz_offset = get_horiz_offset_from_coord(Coord(*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord(Coord(*row, *col), down_side);
            if !self.focus[&Coord(*row, *col)]
            /* not floating */
            {
                let mut sub_image = self.get_subimage_from_horiz_vert_offset(
                    &mut background,
                    horiz_offset,
                    vert_offset,
                );
                self.put_border_on_sub_image(&mut sub_image, 9);
            } else if let Some((Coord(row2, col2), piece)) = &self.floating {
                // if equal, handle later
                if (row2, col2) != (row, col) {
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

        if let Some((Coord(row, col), piece)) = &self.floating {
            let horiz_offset = get_horiz_offset_from_coord(Coord(*row, *col), down_side);
            let vert_offset = get_vert_offset_from_coord(Coord(*row, *col), down_side);
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

            if self.focus.contains_key(&Coord(*row, *col)) {
                self.put_border_on_sub_image(&mut sub_image, 9);
            }
        }

        background
    }

    #[must_use]
    pub fn new(piece_dimension: u32, padding: u32, length_scale: f64) -> Field {
        use rand::seq::SliceRandom;
        use wood_grain::{wood, BRIGHT_WOOD};

        let raw_wood = wood(
            (piece_dimension + padding) * 6 + piece_dimension,
            (piece_dimension + padding) * 7 + piece_dimension,
            f64::from(piece_dimension) / 2.,
            length_scale,
            &BRIGHT_WOOD,
        )
        .expect("should not panic here, since converting u32 to f64 never results in an infinity");

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

        let tam2_image = load_from_80x80(BTAM, piece_dimension);

        let res = multiply_image(&tam2_image, &pieces[i])
            .expect("The dimension of `tam2_image` differs from that of `pieces[i]`");
        //res.save(format!("rawwood_{}.png", i)).unwrap();
        i += 1;

        let physical_tam = PhysicalTam { image: res };

        let mut hashmap = HashMap::new();
        hashmap.insert(
            Coord(Row::O, Column::Z),
            PhysicalPieceOnField::Tam2(physical_tam),
        );

        for (character, col, row, profession, color) in Field::INITIAL_BOARD.iter() {
            let char_image = load_from_80x80(character, piece_dimension);

            let res = multiply_image(&char_image, &pieces[i])
                .expect("The dimension of `char_image` differs from that of `pieces[i]`");
            // res.save(format!("rawwood_{}.png", i)).unwrap();

            hashmap.insert(
                Coord(*row, *col),
                PhysicalPieceOnField::NonTam2(
                    PhysicalNonTam2Piece {
                        color: *color,
                        profession: *profession,
                        image: res,
                    },
                    if *row == Row::A || *row == Row::E || *row == Row::I {
                        AbsoluteSide::ASide
                    } else {
                        AbsoluteSide::IASide
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
