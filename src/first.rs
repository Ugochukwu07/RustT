#![allow(unused)]

use core::num;
use std::io;
use rand::Rng;
use std::io::{Write, BufReader, BufRead, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;

fn main(){
    //STRING 2 types STRING and &str
    let st3 = String::from("x gs sjs dk dso dys");
    let mut v1: Vec<char> = st3.chars().collect();
    v1.sort();
    v1.dedup();
    for char in v1 {
        println!("{}", char);
    }
    let st4 = "Random String";
    let mut st5 = st4.to_string();
    println!("{}", st5);
    let byte_arr1 = st5.as_bytes();
    let st6 = &st5[0..5];
    println!("String lenght : {}", st6.len());
    st5.clear();
    let st6 = String::from("Just Some");
    let st7 = String::from(" words");
    let st8 = st6 + &st7;
    for char in st8.bytes(){
        println!("{}", char);
    }


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
    //METHOD ONE
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

    //METHOD TWO
    // while loop_index < arr_2.len() {
    //     println!("Arr : {}", arr_2[loop_index]);
    //     loop_index += 1;
    // }

    //METHOD THREE
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