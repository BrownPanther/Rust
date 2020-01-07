/******************************************************************************

                            Online Rust Compiler.
                Code, Compile, Run and Debug Rust program online.
Write your code in this editor and press "Run" button to execute it.

*******************************************************************************/


use std::io;
use std::{i32};

pub fn is_leap_year(year: i32) -> bool {

    let mut leap_year: bool = true;
        if year % 4 == 0 
        {
            if year % 100 == 0 
            {
                if year % 400 == 0 
                {
                    leap_year = true;
                    println!("{} is a leap year!", year);
                }
                else 
                {
                    leap_year = false;
                    println!("{} is not a leap year!", year);
                }
            }
            println!("{} is a leap year!", year);
        } 
        else 
        {
            leap_year = false;
            println!("{} is not a leap year!", year);
        }

    leap_year
}

pub fn main()
{
      println!("Enter Year ");
      let mut input1 = String::new();
      io::stdin().read_line(&mut input1).expect("Failed to read
      line");
      
      let aint: i32 = input1.trim().parse().ok().expect("Program only processes numbers, Enter number");
      is_leap_year(aint);
      
}
