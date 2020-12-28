

// 피자 파티
// 입력값을 숫자로만 받을 수 있도록 수정 >> 숫자가 아니면 프로그램이 진행되지 않음
// 사람수 , 피자 개수 , 피자조각 개수

use std::io;
use std::io::Write;
use std::any::type_name;
#[macro_use]
extern crate log;
extern crate env_logger;

use env_logger::Env;
use env_logger::init;
fn main() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let mut people = String::new();
    let mut pizza = String::new();
    let mut pieces = String::new();
    loop{
        print!("How many people?");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut people).expect("cant get people.");
        if people.trim().is_empty() {
            match people.cmp() {

            }
            continue;
        }else {
            println!("How many pizzas do you have?");
        }

    }



    // println!("How many pizzas are in a pizza?");
    // io::stdout().flush().unwrap();
    // println!("{} people with 2 pizzas");
    // println!("Each person gets {} pieces of pizzas");
    // println!("There are {} leftover pieces");


}

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}