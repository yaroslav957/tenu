#![no_main]

use tenu::*;

entry! {
    pub fn main(args: Args) -> i32 {
       for arg in args {
           println!("{}", arg.to_string());
       }
       return 0;
    }
}
