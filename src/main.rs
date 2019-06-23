use std::io;

fn main() {
    println!("Select one:\n1-Convert farenheit to centigradge\n2-Nth fibonacci number.\n3-Sing christmas carol."); //Input menu
    println!("Input: ");
    let mut select = String::new(); //String instance
    io::stdin().read_line(&mut select).expect("Invalid input."); //Line read
    let select: u32 = select.trim().parse().expect("Invalid input."); //Conversion u32

    if select == 1 {
        println!("Input temperature in farenheit: ");
        let mut temp_in_f = String::new();
        io::stdin()
            .read_line(&mut temp_in_f)
            .expect("Failed to read line.");

        let temp_in_f: f32 = temp_in_f.trim().parse().expect("Invalid input.");

        let temp_in_c = f_to_c(temp_in_f); //f_to_c function callingg

        println!("Temperature: {}°C", temp_in_c);
    } else if select == 2 {
        println!("Input a number: ");
        let mut n = String::new();
        io::stdin().read_line(&mut n).expect("Invalid input.");
        let n: i32 = n.trim().parse().expect("Invalid Input.");

        println!("nth fibonacci number is {}", fibo(n)); //fibonacci function calling
    } else if select == 3 {
        sing_christmas_carol(); // christmas carol function calling
    } else {
        println!("Program exit.");
    }
}
fn f_to_c(temp_in_f: f32) -> f32 {
    let temp_in_c = (temp_in_f - 32.0) * 5.0 / 9.0; // Formula : (°F - 32) * 5/9
    temp_in_c
}

fn fibo(n: i32) -> i32 {
    let n = n - 2; //
    let mut i = 0;
    let mut prev = 0;
    let mut next = 1;
    let mut temp;
    while i != n {
        temp = next; //storing next in temp
        next = prev + next; // calculating next
        prev = temp; // setting previous
        i = i + 1; // increment counter
    }
    next
}
fn sing_christmas_carol() {
    let mut _move = 1; // lyrics controllers
    while _move < 13 {
        println!("Verse {}\n", _move); // Printing verse number
        println!(
            "On the {} day of Christmas my true love gave to me.\n",
            _move
        ); //prints eveytime in loop
        if _move > 11 {
            println!("Twelve drummers drumming\n");
        }
        if _move > 10 {
            println!("Eleven pipers piping\n");
        }
        if _move > 9 {
            println!("Ten lords a-leaping\n");
        }
        if _move > 8 {
            println!("Nine ladies dancing\n");
        }
        if _move > 7 {
            println!("Eight maids a-milking\n");
        }
        if _move > 6 {
            println!("Seven swans a-swimming\n");
        }
        if _move > 5 {
            println!("Six geese a-laying\n");
        }
        if _move > 4 {
            println!("Five golden rings\n");
        }
        if _move > 3 {
            println!("Four calling birds\n");
        }
        if _move > 2 {
            println!("Three french hens\n");
        }
        if _move > 1 {
            println!("Two turtle doves, and\n");
        }
        if _move > 0 {
            println!("A partridge in a pear tree\n");
        }
        println!("\n"); // Printing gap in between verses
        _move = _move + 1; // increment lyric controller
    }
}
