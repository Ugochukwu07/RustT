#![allow(unused)]

// use core::num;
// use std::io;
// use rand::Rng;
// use std::io::{Write, BufReader, BufRead, ErrorKind};
// use std::fs::File;
// use std::io::prelude::*;
// use std::cmp::Ordering;

// struct Color{
//     red: u8,
//     green: u8,
//     blue: u8
// }

// struct Color(u8, u8, u8);

// use core::num;

// fn print_color (c: &Color){
//     println!("Color - R:{}, G:{}, B:{}", c.red, c.green, c.blue);
// }
// struct Rectangle {
//     width: u32,
//     lenght: u32
// }

// impl Rectangle {
//     fn print_description(&self){
//         println!("Rectangle: {} x {}", &self.width, &self.lenght)
//     }

//     fn is_square(&self) -> bool{
//         &self.lenght == &self.width 
//     }
// }

struct Person {
    name: String,
    age: u8
}

impl ToString for Person{
    fn to_string(&self) -> String {
        return format!("My name is {}, and am {}", &self.name, &self.age);
    }
}

fn main(){
    //TRAITS
    let ugo = Person { name: String::from("Ugochukwu"), age: 21 };
    println!("{}", ugo.to_string());

    //Impl Keyword (Implementation)
    // let my_rect = Rectangle {width: 100, lenght: 100};  
    // my_rect.print_description();
    // println!("Rectangle is square: {}", my_rect.is_square());

    //ARRAY
    // let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // let my_num = [2; 300];

    // for i in 0..numbers.len(){
    //     println!("{}", i);
    // }
    // for i in my_num.iter(){
    //     println!("{}", i);
    // }

    //Pass by Reference
    // let blue = Color {red: 0, green: 0, blue: 225};
    // print_color(&blue);
    // print_color(&blue);

    //TUPLE STRUCTS
    // let mut colors = Color(223, 45, 89);
    // colors.2 = 100;
    // println!("Red is {}, {}, {}", colors.1, colors.2, colors.0);

    // //STRUCT
    // let mut bg = Color {red: 255, green: 211, blue: 204};
    // bg.green = 045;
    // println!("red: {}, green: {}, blue: {}", bg.red, bg.green, bg.blue);


    //References
    // let mut x = 10;
    // // let xr = &x;
    // {
    //     let dom = &mut x;
    //     *dom+=1;
    // }

    // println!("x is {}", x);

    //SHADOWING
    // let mut x = 10;
    // {
    //     x = 15;
    //     println!("x is {}", x);

    //     //do stuff with 15
    // }
    
    // let x = "X is now a String";
    // println!("x is {}", x);

    // let x = true;
    // println!("x is {}", x);

    //CODE BLOCKS
    // let x = 10;
    // {
    //     let y = 5;
    //     println!("x: {}, y: {}", x,y);
    // }

    //FUNCTIONS
    // print_numbers_to(20);

    // fn print_numbers_to(num: u32){
    //     for n in 1..num{
    //         if is_even(n) {
    //             println!("{} is even", n);
    //         } else {
    //             println!("{} is old", n);
    //         }
    //     }
    // }

    // fn is_even(num: u32) -> bool {
    //     return num % 2 == 0;
    // }

    //FOR LOOPS
    // let numbers = 40..70;
    // for i in numbers {
    //     println!("The number is {}", i);
    // }

    // let animals = vec!["Dog", "Cat", "Hen"];
    // for (index, a) in animals.iter().enumerate() {
    //     println!("this index {}, is {}", index, a);
    // }


    //FILES
    // let mut file = File::open("info.txt")
    //                         .expect("Can't open file");

    // let mut contents = String::new();
    // file.read_to_string(&mut contents)
    //     .expect("Oops!, Can't read file");

    // println!("File Contents:\n\n{}", contents);

    // //VECTORS
    // let vec1: Vec<i32> = Vec::new();
    // let mut vec2: Vec<i8> = vec![];
    // println!("{:?}", vec1);
    // vec2.push(2);
    // vec2.push(3);
    // println!("{:?}", vec2);
    // for number in vec2.iter(){
    //     println!("{}", number);
    // }

    //ENUMS
    // enum Day{
    //     Monday,
    //     Tuesday,
    //     Wednesday,
    //     Thursday,
    //     Friday,
    //     Saturday,
    //     Sunday
    // }

    // impl Day {
    //     fn is_weekend(&self) -> bool {
    //         match self{
    //             Day::Saturday | Day::Sunday => true,

    //             _ => false
    //         }
    //     }
    // }

    // let today:Day = Day::Tuesday;
    // match today {
    //     Day::Monday => println!("Everyone hates monday"),
    //     Day::Tuesday => println!("Donut day"),
    //     Day::Wednesday => println!("Hump day"),
    //     Day::Thursday => println!("Pay day"),
    //     Day::Friday => println!("Almost Weekend"),
    //     Day::Saturday => println!("Weekend"),
    //     Day::Sunday => println!("Weekend"),
    // }

    // println!("Is today the weekend => {}", today.is_weekend());

    // //CASTING
    // let int_u8: u8 = 5;
    // let int2_u8: u8 = 4;
    // let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);
    // println!("{}", int3_u32);

    //STRING 2 types STRING and &str
    // let st3 = String::from("x gs sjs dk dso dys");
    // let mut v1: Vec<char> = st3.chars().collect();
    // v1.sort();
    // v1.dedup();
    // for char in v1 {
    //     println!("{}", char);
    // }
    // let st4 = "Random String";
    // let mut st5 = st4.to_string();
    // println!("{}", st5);
    // let byte_arr1 = st5.as_bytes();
    // let st6 = &st5[0..5];
    // println!("String lenght : {}", st6.len());
    // st5.clear();
    // let st6 = String::from("Just Some");
    // let st7 = String::from(" words");
    // let st8 = st6 + &st7;
    // for char in st8.bytes(){
        // println!("{}", char);
    // }


    // let mut st1 = String::new();
    // st1.push('A');
    // st1.push_str("words");
    // // println!("{}", st1);
    // for word in st1.split_whitespace(){
    //     println!("{}-", word);
    // }
    // let st2 = st1.replace("A", "Another");
    // println!("{}", st2);

    //TURPLE
    // let my_tuple: (u8, String, f64) = (47, "Ugochukwu".to_string(), 50_000.00);
    // println!("Name : {}", my_tuple.1);
    // let(v1, v2, v3) = my_tuple;
    // println!("Age : {}", v1);

    //ARRAYS <elm > must be in the same datatype & it has fixed sizes
    // let arr_1 = [1, 2, 3, 4, 5, 6];
    // println!("!st Elemnt : {}", arr_1[0]);
    // println!("Array Lentght : {}", arr_1.len());

    //LOOPING THROUGH ARRAY
    // let arr_2 = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let mut loop_index = 0;
    // //METHOD ONE
    // loop {
    //     if arr_2[loop_index] % 2 == 0{
    //         loop_index += 1;
    //         continue;
    //     }
    //     if arr_2[loop_index] == 9 {
    //         break;
    //     }
    //     println!("Val : {}", arr_2[loop_index]);
    //     loop_index += 1;
    // }

    // //METHOD TWO
    // while loop_index < arr_2.len() {
    //     println!("Arr : {}", arr_2[loop_index]);
    //     loop_index += 1;
    // }

    // //METHOD THREE
    // for val in arr_2.iter() {
    //     println!("Val : {}", val);
    // }

    // //CONSTIONALS
    // let age = 8;
    // if(age >= 1) && (age <= 18){
    //     println!("Important Birthday");
    // } else if(age == 21) || (age == 50) {
    //     println!("Important Birthday");
    // } else if age >= 65 {
    //     println!("Important Birthday");
    // }else{
    //     println!("Not Important Birthday");
    // }

    // //TINIARY OPREATORS
    // let mut my_age = 47;
    // let can_vote = if my_age >= 10{
    //     true
    // }else{
    //     false
    // };
    // println!("Can Vote: {}", can_vote);

    // //MATCH
    // let age2 = 900;
    // match age2 {
    //     1..=18 => println!("Important Birthday"),
    //     21 | 50 => println!("Important Birthday"),
    //     65..=i32::MAX => println!("Important Birthday"),
    //     _=> println!("Not an Important Birthday")
    // };


    // let random_num = rand::thread_rng().gen_range(1, 101);
    // println!("Random: {}", random_num);

    // let my_age = 18;
    // let voting_age = 18;

    // match my_age.cmp(&voting_age){
    //     Ordering::Less => println!("Can't Vote"),
    //     Ordering::Greater => println!("Can Vote"),
    //     Ordering::Equal => println!("You gain the right to vote"),
    // };

    //DATA TYPES

    //INTERGERS depend on my computer x64bit-> MINE
    //Unisigned INterers: u8, u16, u32, u64, u128, usize
    //Signed INterers: i8, i16, i32, i64, i128, isize

    // println!("Max u32 : {}", u32::MAX);
    // println!("Max u64 : {}", u64::MAX);
    // println!("Max usize : {}", usize::MAX);
    // println!("Max u128 : {}", u128::MAX);
    // println!("Max f32 : {}", f32::MAX);
    // println!("Max f64 : {}", f64::MAX);
    // println!("Max i8 : {}", i8::MAX);

    // //BOOLEAN: true and flase
    // let _is_true = true; //to ignore unused variable: start with underscore eg: _var_name

    // //Charater
    // let my_grade = 'A';

    // //PRESISTION
    // let num_1: f32 = 1.111111111111111;
    // println!("f32 : {}", num_1 + 0.111111111111111);
    // let num_2: f64 = 1.111111111111111;
    // println!("f64 : {}", num_2 + 0.111111111111111);


    //CONSTANTS
    // const ONE_MIL: u32 = 1_000_000;
    // const PI: f32 = 3.141592;

    // let age = "47"; // string with double qoutes
    // let your_age = 'g'; //charater with single qoutes

    // let mut age: u32 = age.trim().parse()
    //     .expect("Age wans't assigned a number");

    // age += 1;

    // println!("{}, and my age is {} and i want {}", your_age, age, ONE_MIL);


    // let mut num = 40;
    // num = 12;
    // println!("{}", num);
    
    // let mut name = String::new();
    // let greeting = "Nice to meet you";

    // io::stdin()
    //     .read_line(&mut name)
    //     .expect("Didn't Recieve Input");

    // println!("Hello, world! {}, nice to meet you.", name.trim_end());
}