use std::io;
use rand::Rng;

fn main() {
    let cp_num = rand::thread_rng().gen_range(1..=100);
    println!("The program has generated a number between 1 and 100. Try to guess it!"); 
    loop {
        println!("###################################################################");
        println!("Please input your guess!");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("failed to read the line!");
        let guess = match guess.trim().parse::<i32>(){
            Ok(n) => n,
            Err(_) => {println!("not a number"); return;}
        };
        if guess != cp_num {
            if guess < cp_num {
                println!("your {} number is small than program random number", guess);
            }
            else {
                println!("your {} number is bigger than program random number", guess);
            }
        }
        else {
            println!("congratulate!, Your number is as same as program {}", cp_num);
            break;
        }
    }
    
}
