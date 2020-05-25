extern crate image;

mod field;

fn main() -> Result<(), rand_distr::NormalError> {
    let field = field::Field::new();
    field.render().save("a.png").unwrap();

    Ok(())
}
