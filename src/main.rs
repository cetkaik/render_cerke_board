extern crate image;

mod field;

fn main() -> Result<(), rand_distr::NormalError> {
    let field = field::Field::new();
    field.render(field::Side::IASide).save("a.png").unwrap();
    field.render(field::Side::ASide).save("b.png").unwrap();

    Ok(())
}
