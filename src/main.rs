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

    Ok(())
}
