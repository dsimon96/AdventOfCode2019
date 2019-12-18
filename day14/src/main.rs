use std::collections::HashMap;
use std::io::{stdin, BufRead};

use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "rule.pest"]
struct RuleParser;

#[derive(Debug)]
struct Component {
    num: usize,
    sym: String,
}

fn main() {
    let mut rulemap: HashMap<String, (usize, Vec<Component>)> = HashMap::new();
    for line in stdin().lock().lines() {
        let s = line.unwrap();
        let mut rule = RuleParser::parse(Rule::rule, &s).unwrap();

        let mut inputs: Vec<Component> = Vec::new();
        let input = rule.next().unwrap();
        for comp in input.into_inner() {
            let mut input_comp = Component { num: 0, sym: String::from("") };
            for tok in comp.into_inner() {
                match tok.as_rule() {
                    Rule::num => input_comp.num = tok.as_str().parse::<usize>().unwrap(),
                    Rule::sym => input_comp.sym = String::from(tok.as_str()),
                    _ => unreachable!()
                };
            }
            inputs.push(input_comp);
        }

        let output = rule.next().unwrap();
        let output_inner = output.into_inner().next().unwrap();
        let mut output_comp = Component { num: 0, sym: String::from("") };
        for tok in output_inner.into_inner() {
            match tok.as_rule() {
                Rule::num => output_comp.num = tok.as_str().parse::<usize>().unwrap(),
                Rule::sym => output_comp.sym = String::from(tok.as_str()),
                _ => unreachable!()
            }
        }

        rulemap.insert(output_comp.sym, (output_comp.num, inputs));
    }


    let mut base = 0;
    let mut step = 1;

    loop {
        let mut ore_needed = 0;

        let mut need: HashMap<String, usize> = HashMap::new();
        let mut have: HashMap<String, usize> = HashMap::new();

        need.insert(String::from("FUEL"), base + step);

        while need.len() > 0 {
            let sym_needed = need.keys().next().unwrap().clone();
            let mut num_needed = need.remove(&sym_needed).unwrap();
            let (num_outputs, rule_inputs) = rulemap.get(&sym_needed).unwrap();

            let num_have = have.entry(sym_needed).or_insert(0);
            if *num_have >= num_needed {
                *num_have -= num_needed;
                continue;
            } else {
                num_needed -= *num_have;
                *num_have = 0;
            }

            let mut copies = num_needed / num_outputs;
            if num_needed % num_outputs > 0 {
                copies += 1;
                *num_have += num_outputs * copies - num_needed;
            }

            for in_comp in rule_inputs {
                let sym_in = in_comp.sym.clone();
                let num_in = copies * in_comp.num;

                if sym_in == "ORE" {
                    ore_needed += num_in;
                } else {
                    let counter = need.entry(sym_in).or_insert(0);
                    *counter += num_in;
                }
            }
        }

        if ore_needed <= 1000000000000 {
            println!("{}: YES (doubling step size)", base + step);
            step *= 2;
        } else if step == 1 {
            // base is the max val
            println!("{}: NO (found max)", base + step);
            break;
        } else {
            println!("{}: NO (resetting step size)", base + step);
            base += step / 2;
            step = 1;
        }
    }

    println!("Max: {}", base);
}
