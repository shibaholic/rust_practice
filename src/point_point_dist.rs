use std::io;
use std::io::prelude::*;
use anyhow;
struct Point {
    x:i32,
    y:i32,
}

pub fn point_point_dist() {
    loop {
        let point_1: Point;
        let point_2: Point;

        print!("Write the x,y coordinates for the first point: ");
        match stdin_point() {
            Ok(point) => point_1 = point,
            Err(error) => { println!("Error: {:?}", error); continue }
        }
        print!("Write the x,y coordinates for the second point: ");
        match stdin_point() {
            Ok(point) => point_2 = point,
            Err(error) => { println!("Error: {:?}", error); continue }
        }

        println!("The points you entered were: ({},{}) and ({},{})", point_1.x, point_1.y, point_2.x, point_2.y);

        let side_dif_calc = |one, two| -> f64 { one as f64 - two as f64 };
        let x_dif = side_dif_calc(point_2.x, point_1.x);
        let y_dif = side_dif_calc(point_2.y, point_1.y);
        let distance = f64::sqrt(x_dif.powf(2.) + y_dif.powf(2.)); 

        println!("The distance between the points is {:.2}.", distance);
    }
}

fn stdin_point() -> Result<Point, anyhow::Error> {
    io::stdout().flush().expect("flush");
    let mut input = String::new();
    io::stdin().lock().read_line(&mut input).expect("read"); 
    let trimmed = input.trim_end();
    return parse_point(trimmed);
} 

fn parse_point(input:&str) -> anyhow::Result<Point> {
    
    let mut y_coordinate = 0;
    let comma_index = input.find(',').ok_or_else(|| anyhow::Error::msg("invalid format (x,y)"))?;

    let x_slice = &input[0..comma_index];
    let x_coordinate = x_slice.parse::<i32>()?; 
    let y_slice = &input[comma_index + 1..input.len()];
    let y_coordinate = y_slice.parse::<i32>()?;
    return Ok(Point { x: x_coordinate, y: y_coordinate });
}
