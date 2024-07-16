#[derive(Debug)]
pub struct Coord<T> {
    pub z: T,
    pub x: T
}

#[derive(Copy, Clone, Debug)]
pub struct TriCoord<T> {
    pub a: T,
    pub b: T,
    pub c: T
}

const TRI_SIDE:f32 = 1.0;
const TRI_ALTITUDE:f32 = 0.866025404;
const TRI_HALF_ALT:f32 = 0.433012702;
const TRI_APOTHEM:f32 = 0.288675135;

pub const CHUNK_SIDE:f64 = 16.0;
const CHUNK_ALTITUDE:f64 = 13.856406461;
const CHUNK_HALF_ALTITUDE:f64 = 6.92820323;
const CHUNK_APOTHEM:f64 = 4.618802154;

pub fn tricoord_tester() {
    let mut tricord = TriCoord::<i16> { a: 0, b: 0, c:0 };
    println!("tricord: {:?}", tricord);
    println!("(z,x): {:?}\n", trichunk_to_coord(tricord));

}

// basically going reverse: from the triangle find how many steps to get to origin.
fn trichunk_to_coord(tricoord: TriCoord<i16>) -> Coord<f64> {
    // converts a,b,c origin of trichunk to z,x world coordinates
     let sum = tricoord.a + tricoord.b + tricoord.c;
     let mut temp = tricoord;
     let mut zx_coord = Coord::<f64> {z:0.0, x:0.0};

    // go to a == 0
    println!("step 1");
    zx_coord.x += -temp.a as f64 * CHUNK_SIDE;
    println!("  x is now = {:.02}", zx_coord.x);
    temp.c = temp.c + temp.a;
    temp.a = 0;
    println!("  a is now = {}, c is now = {}", temp.a, temp.c);

    // if odd then go to even
    println!("step 2");
    let c_b_sum = temp.b + temp.c; // 1 if odd, 0 if even
    println!("  c-b sum = {}", c_b_sum);

    let s2_z_value = CHUNK_APOTHEM * c_b_sum as f64;
    zx_coord.z += s2_z_value;
    println!("  z change was {}, z is now = {}", s2_z_value, zx_coord.z);

    let s2_x_value = CHUNK_SIDE/2.0 * c_b_sum as f64;
    zx_coord.x += s2_x_value;
    println!("  x change was {}, x is now = {}", s2_x_value, zx_coord.x);

    temp.c -= c_b_sum;
    println!("  c is now {}", temp.c);

    // now b and c's absolute values equal each other
    // go to b == 0, c == 0
    println!("step 3");
    println!("  b is currently = {}", temp.b);
    let s3_z_value = -temp.b as f64 * CHUNK_ALTITUDE;
    zx_coord.z += s3_z_value;
    println!("  z change was {}, z is now = {}", s3_z_value, zx_coord.z);
    let s3_x_value = -temp.b as f64 * CHUNK_SIDE/2.0;
    zx_coord.x += s3_x_value;
    println!("  x change was {}, x is now = {}", s3_x_value, zx_coord.x);
    
    return zx_coord;
}

// fn tricoord_array(side_size: i16) -> Vec<TriCoord<i16>> {
    
//     let triangular_number = 
//     let mut vec:Vec<TriCoord<i16>> = Vec::with_capacity();

//     let mut col_index = 0;
//     loop {
//         // first iterate the column tricoords
//         if col_index == side_size {
//             return 
//         }

//         let mut row_index = 0;
//         loop {
//             // second iterate the row tricoords
//             if row_index == side_size {
//                 break;
//             }

//             row_index += 1;
//         }

//         col_index += 1;
//     }
// }

pub fn triangular_number_naive(n: i16) -> i16 {
    let mut index = 0;
    let mut sum = 0;
    loop {
        index += 1;
        sum += index;

        if index == n {
            return sum; 
        }
    }
}

pub fn triangular_number_O1(n: i16) -> i16 {
    return n * (n + 1) / 2;
}