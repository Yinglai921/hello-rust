mod match_number;
mod match_shape;
mod option_enum;

/*
    enum Command{
        Clear,
        DrawLine(f64, f64),
        DrawShape(Shape)
    }
*/

fn main() {
    match_shape::read_shape();
    match_shape::read_perimeter();
    match_number::read_number();
    option_enum::get_number();
    option_enum::get_number_2();
    option_enum::if_let();
}
