pub fn fizzbuzz(i:u32) {
    println!("start fizzbuzz {}", i);

    let mut c = 0;
    loop {
        c += 1;
        
        let three = c % 3 == 0;
        let five = c % 5 == 0;
        if three | five {
            if three {
                print!("fizz")
            }
            if five {
                print!("buzz")
            }
        } else {
            print!("{}", c);
        }
        
        println!();

        if c == i {
            break;
        }
    }
}