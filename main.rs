/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/11 10:29:08 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/17 16:15:58 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// assert_eq!(true as i32, 1);
// assert_eq!(false as i32, 0);

use std::io;
use std::process;
use zeroize::Zeroize;
mod final_amount;

fn main() {
    let principal: f64;
    let rate: f64;
    let time: f64;
    let pmt: f64;
    let mut input = String::new();
    println!("Please indicate the principal:");
    io::stdin().read_line(&mut input).unwrap();
    principal = input
        .trim()
        .parse::<f64>()
        .expect("Principal is incorrectly indicated.");
    if principal <= 0.0 {
        println!("Please indicate a value larger than 0. Example: 100. Exiting program.");
        process::exit(1);
    }
    input.zeroize();
    println!("Please indicate the rate (yearly):");
    io::stdin().read_line(&mut input).unwrap();
    rate = input
        .trim()
        .parse::<f64>()
        .expect("Rate is incorrectly indicated.");
    if rate <= 0.0 {
        println!("Please indicate a value larger than 0. Ex: 5. Exiting program.");
        process::exit(1);
    }
    input.zeroize();
    println!("Please indicate the time in years:");
    io::stdin().read_line(&mut input).unwrap();
    time = input
        .trim()
        .parse::<f64>()
        .expect("Time is incorrectly indicated.");
    if time <= 0.0 {
        println!("Please restart program and indicate a positive value, larger than 0, in years. Exiting program.");
    }
    input.zeroize();
    println!("Will you be making monthly deposits? If so, please indicate the sum. If not, please simply write 'no'.");
    io::stdin().read_line(&mut input).unwrap();
    input = input.trim().to_string();
    if input == "no" {
        pmt = 0.0;
    } else {
        pmt = match input.parse::<f64>() {
            Ok(x) => x,
            Err(_i) => {
                println!("Monthly payment incorrectly given. Please retart program and restate your desired monthly payment. Exiting program.");
                process::exit(1);
            }
        };
    }
    println!(
        "Your final amount, for the indicated parameters will be {}",
        final_amount::final_amount(pmt, principal, rate, time)
    );
}
