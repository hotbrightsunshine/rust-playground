use core::cmp::Ordering::Equal;
use std::{collections::{HashMap, HashSet}, vec};

fn change(total : &mut f32, pieces : &mut [f32])
    -> Vec<f32>{
    let mut change : Vec<f32> = vec![]; 
    
    let mut index : usize = 0;

    // ordering array
    pieces.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    pieces.reverse();
    println!("{:?}", pieces);

    while *total != 0.0 {
        if &mut pieces[index] > total {
            index = if index+1==pieces.len(){break} else {index+1}
        } else { 
            *total = *total - pieces[index]; 
            change.push(pieces[index]);
        }
    }

    return change;
}

fn wrap<T: Copy, Eq>(arr : Vec<T>) -> Vec<(usize, T)> {
    unimplemented!()
}

fn count<T: Eq>(arr : Vec<T>, val : T) -> i32{
    let mut i = 0;
    for x in arr {
        if x == val {
            i = i + 1;
        }
    }
    return i;
}

fn main() {
    let mut pieces : Vec<f32> = vec![500.0, 200.0, 100.0, 50.0, 20.0, 10.0, 5.0, 1.0, 0.5, 0.2, 0.1, 0.05, 0.02, 0.01];
    
    let change: Vec<(f32)> = change(&mut 250.0, &mut pieces);

    println!("{:?}", change);
}
