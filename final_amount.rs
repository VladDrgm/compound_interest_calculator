/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   final_amount.rs                                    :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: vdragomi <vdragomi@student.42wolfsburg.de> +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2021/08/10 16:26:43 by vdragomi          #+#    #+#             */
/*   Updated: 2021/08/17 15:59:03 by vdragomi         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

// The formula for compound interest, including principal sum, is:
// A = P (1 + r/n) ** (t * 12)  -> if t (time) is represented in years; else time is not multiplied by 12
// Where:
// A = the future value of the investment/loan, including interest
// P = the principal investment amount (the initial deposit or loan amount)
// r = the annual interest rate (decimal)
// t = the time the money is invested or borrowed for
// n = number of times interest is compounded
// https://www.thecalculatorsite.com/articles/finance/compound-interest-formula.php
// -------> Total = [ Compound interest for principal ] + [ Future value of a series ]
//--------> Total = [ P(1+r/n)^(nt) ] + [ PMT × (((1 + r/n)^(nt) - 1) / (r/n)) ]
//TIME IS EXPRESSED IN YEARS!

// If the additional deposits are made at the END of the period (end of month, year, etc), here are the two formulae you will need:

// Compound interest for principal:

// P(1+r/n) ^ (nt)

// Future value of a series:

// PMT × {[(1 + r/n) ^ (nt) - 1] / (r/n)}

pub fn final_amount(pmt: f64, principal: f64, mut rate: f64, time: f64) -> f64 {
    rate = rate / 100.0;
    let power = f64::powf(1.0 + rate / 12.0, time * 12.0);
    let a = principal * power;
    let future_value_power = f64::powf(1.0 + (rate / 12.0), 12.0 * time);
    let future_value = ((future_value_power - 1.0) / (rate / 12.0)) * pmt;
    let total: f64 = a + future_value;
    total
}
