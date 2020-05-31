extern crate image;

mod field;

fn main() -> Result<(), rand_distr::NormalError> {
    use field::Column;
    use field::Row;
    let mut field = field::Field::new();
    field.render(field::Side::IASide).save("a.png").unwrap();
    field.render(field::Side::ASide).save("b.png").unwrap();

    field.to_opponent_hop1zuo1((Row::A, Column::K)).unwrap();

    field.render(field::Side::IASide).save("a2.png").unwrap();
    field.render(field::Side::ASide).save("b2.png").unwrap();

    field
        .to_empty_square((Row::A, Column::K), (Row::A, Column::L))
        .unwrap();

    field.render(field::Side::IASide).save("a3.png").unwrap();
    field.render(field::Side::ASide).save("b3.png").unwrap();

    field
        .step_on_occupied((Row::A, Column::P), (Row::A, Column::M))
        .unwrap();

    field.render(field::Side::IASide).save("a4.png").unwrap();
    field.render(field::Side::ASide).save("b4.png").unwrap();

    field.relocate_stepping((Row::O, Column::Z)).unwrap();

    field.render(field::Side::IASide).save("a5.png").unwrap();
    field.render(field::Side::ASide).save("b5.png").unwrap();

    field.descend_from_stepping((Row::O, Column::C)).unwrap();

    field.render(field::Side::IASide).save("a6.png").unwrap();
    field.render(field::Side::ASide).save("b6.png").unwrap();

    Ok(())
}
