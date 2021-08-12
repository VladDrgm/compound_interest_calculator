/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/07/11 10:29:08 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/12 15:09:54 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use std::io;
mod final_amount;

fn main() {
    let mut principal = String::new();
    println!("Please indicate the principal:");
    io::stdin().read_line(&mut principal).unwrap();
    let principal = principal.trim();
    let principal = principal
        .parse::<f64>()
        .expect("Principal is not correctly indicated.");
    let mut rate = String::new();
    println!("Please indicate the rate (yearly):");
    io::stdin().read_line(&mut rate).unwrap();
    let rate = rate.trim();
    let rate = rate.parse::<f64>().expect(
        "Rate is not a correctly indicated value. Please indicate the correct rate (ex: 5.0)",
    );
    let mut time = String::new();
    println!("Please indicate the time in years:");
    io::stdin().read_line(&mut time).unwrap();
    let time = time.trim();
    let time = time
        .parse::<f64>()
        .expect("Time is not a correctly indicated value. Please indicate the number of years.");
    println!(
        "Your final amount, for the indicated parameters will be {}",
        final_amount::final_amount(principal, rate, time)
    );
}
