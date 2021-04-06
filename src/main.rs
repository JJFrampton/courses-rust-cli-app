use std::io;
use std::process;

fn main() {
    println!("Please enter your weight (kg) : ");
    // create empty mutable string
    let mut input = String::new();
    // read user input, pass value into the variable created above
    io::stdin().read_line(&mut input).unwrap(); // unwrap will terminate program on error and yield the value on success
    let mut weight = parse_input(input);
    // weight = calculate_weight(weight); // transfer ownership / pass by value && type will be infered by the compiler
    calculate_weight(&mut weight);        // borrowing / pass by reference
    weight = weight * 1000.0; // weight needs to be mutable for this to work
    println!("Your weight is {}", weight);
}

// function signature syntax:
// fn function_name(var_name: arg_type) -> return_type {

// fn calculate_weight(weight: f32) -> f32 { // transfer ownership / pass by value
//     // the last statement without a semi colon will be returned automatically
//     (weight / 9.81) * 3.711
// }
fn calculate_weight(weight: &mut f32) { // borrowing / pass by reference
    *weight = (*weight / 9.81) * 3.711;
}

fn parse_input(weight: String) -> f32 {
    // let weight_string = weight.trim(); // remove white space
    // let weight_int: f32 = weight_string.parse().unwrap(); // parse to f32 and unwrap
    // return weight_int;

    // oneliner example : using turbofish to specify the type to parse into
    // weight.trim().parse::<f32>().unwrap()

    // matching result of parsing example :
    let weight_float = weight.trim().parse::<f32>();
    match weight_float {
        Ok(number) => return number,
        Err(error) => {
            println!("\nYou must enter an integer or float\n\
            Rust Error : \"{}\"\n\
            Terminating the program\n", error);
            process::exit(1);
        },
    };
}
