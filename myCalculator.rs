use std::io::{self, Read};

fn main() {
    /* Kelvin's calculator */
    //print intro and ask for for 2 number values and an operator
    println!("welcome to my calculator, you will have to input 2 numbers and an operator from '+=*/'.");

    //ask for first value as string, convert it to int and case it to float
    println!("please input your first value: ");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let num1: i32 = input1.trim_end().parse().expect("im expecting an int");
    let float1 = num1 as f64;

    //ask for second value as string, convert it to int and case it to float
    println!("please input your second value: ");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let num2: i32 = input2.trim_end().parse().expect("im expecting an int");
    let float2 = num2 as f64;

    //ask for the operator as a string and match it with the cases, declare the function outside main and call it
    //when cases are match
    println!("Choose an operator '+=*/': ");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let operator = input3.trim();

    //Match the operator
    match operator {
        "+" => {
            add(float1, float2);
        }
        "-" => {
            subtract(float1, float2);
        }
        "*" => {
            multiply(float1, float2);
        }
        "/" => {
            divide(float1, float2);
        }
        &_ => {
            println!("Invalid operator {}", operator)
        }
    }

//Math Functions
fn add(x: f64, y: f64){
    println!("The result of {} + {} = {}", x, y, x+y);
}

//fn subtract
fn subtract(x: f64, y: f64){
    println!("The result of {} - {} = {}", x, y, x-y);
}

fn multiply(x: f64,y: f64){
    println!("The result of {} * {} = {}", x, y, x*y);
}

fn divide(x: f64, y: f64){
    println!("The result of {} / {} = {}", x, y, x/y);
}
