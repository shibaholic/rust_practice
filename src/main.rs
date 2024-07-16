use rust_practice::fizzbuzz::fizzbuzz;
use rust_practice::guess_number::guess_number;
use rust_practice::prime::is_prime;
use rust_practice::point_point_dist::point_point_dist;
use rust_practice::tricoord_conv::{triangular_number_O1, triangular_number_naive, tricoord_tester, TriCoord};
fn main() {
    // fizzbuzz(16);
    // guess_number();
    // is_prime();
    // point_point_dist();
    // tricoord_tester();
    println!("{}", triangular_number_naive(5));
    println!("{}", triangular_number_O1(5));
}