use std::collections::HashMap;
use std::cell::RefCell;
extern crate num;
use num::integer::{gcd, lcm};

enum Monkey {
    Evaluated(i64),
    Operation {
        first_operand: String,
        operator: String,
        second_operand: String,
    },
    Init,
}

fn get_or_create_monkey<'b, 'a>(
    name: &'b str, 
    monkeys_map: &'a mut HashMap<String, RefCell<Monkey>>,
) -> &'a RefCell<Monkey> {
    if monkeys_map.get(name).is_none() {
        monkeys_map.insert(String::from(name), RefCell::new(Monkey::Init)); 
    }
    monkeys_map.get(name).unwrap()
}

fn eval(monkey: &RefCell<Monkey>, monkeys_map: &HashMap<String, RefCell<Monkey>>) -> i64 {
    match &*monkey.borrow() {
        Monkey::Evaluated(val) => { *val },
        Monkey::Operation { first_operand , operator, second_operand } => {
            let v1 = eval(monkeys_map.get(first_operand).unwrap(), monkeys_map);
            let v2 = eval(monkeys_map.get(second_operand).unwrap(), monkeys_map);
            match operator.as_str() {
                "+" => {v1 + v2},
                "-" => {v1 - v2},
                "*" => {v1 * v2},
                _ => {v1 / v2},
            }
        },
        Monkey::Init => { unreachable!() },
    }
}

/* c0 + c1 * humn */
#[derive(Copy, Clone)]
struct Formula {
    c0_num: i64,
    c0_den: i64,
    c1_num: i64,
    c1_den: i64,
}

impl Formula {
    fn neg(&self) -> Formula {
        Formula { 
            c0_num: -self.c0_num,
            c0_den: self.c0_den,
            c1_num: -self.c1_num,
            c1_den: self.c1_den,
        }
    }
    fn add(&self, other: &Formula) -> Formula {
        let c0_den = lcm(self.c0_den, other.c0_den);
        let c0_num = self.c0_num * (c0_den / self.c0_den)
                          + other.c0_num * (c0_den / other.c0_den);
        let c1_den = lcm(self.c1_den, other.c1_den);
        let c1_num = self.c1_num * (c1_den / self.c1_den)
                          + other.c1_num * (c1_den / other.c1_den);
        Formula { c0_num, c0_den, c1_num, c1_den }
    }
    fn subtract(&self, other: &Formula) -> Formula {
        self.add(&other.neg())
    }
    fn multiply(&self, other: &Formula) -> Formula {
        assert!(self.c1_num == 0 || other.c1_num == 0);
        let mut c0_num = self.c0_num * other.c0_num;
        let mut c0_den = self.c0_den * other.c0_den;
        let (mut c1_num, mut c1_den);
        if self.c1_num == 0 {
            c1_num = self.c0_num * other.c1_num;
            c1_den = self.c0_den * other.c1_den;
        } else {
            c1_num = self.c1_num * other.c0_num;
            c1_den = self.c1_den * other.c0_den;
        }
        let gcd0 = gcd(c0_den, c0_num);
        c0_den /= gcd0;
        c0_num /= gcd0;
        let gcd1 = gcd(c1_den, c1_num);
        c1_den /= gcd1;
        c1_num /= gcd1;
        Formula { c0_num, c0_den, c1_num, c1_den }
    }
    fn reverse(&self) -> Formula {
        assert!(self.c1_num == 0);
        Formula { c0_num:self. c0_den, c0_den: self.c0_num, c1_num: 0, c1_den: 1 }
    }
    fn divide(&self, other: &Formula) -> Formula {
        self.multiply(&other.reverse())
    }
}

fn is_humn(name: &str) -> Option<Formula> {
    match name {
        "humn" => { Some(Formula { c0_num: 0, c0_den: 1, c1_num: 1, c1_den: 1 }) },
        _ => { None },
    }
}

fn get_formula(monkey: &Monkey, monkeys_map: &HashMap<String, RefCell<Monkey>>) -> Formula {
    match monkey {
        Monkey::Evaluated(value) => {
            Formula {c0_num: *value, c0_den: 1, c1_num: 0, c1_den: 1}
        }
        Monkey::Operation { first_operand, operator, second_operand } => {
            let (f1, f2): (Formula, Formula);
            if let Some(special) = is_humn(first_operand.as_str()) {
                f1 = special;
            } else {
                f1 = get_formula(&*monkeys_map.get(first_operand).unwrap().borrow(), monkeys_map);
            }
            if let Some(special) = is_humn(second_operand.as_str()) {
                f2 = special;
            } else {
                f2 = get_formula(&*monkeys_map.get(second_operand).unwrap().borrow(), monkeys_map);
            }

            match operator.as_str() {
                "+" => f1.add(&f2),
                "-" => f1.subtract(&f2),
                "*" => f1.multiply(&f2),
                "/"|_ => f1.divide(&f2),
            }
        }
        _ => { unreachable!(); }
    }
}


fn main() {
    println!("Hello, world!");
    let mut monkeys_map: HashMap<String, RefCell<Monkey>> = HashMap::new();
    for l in include_str!("../input.txt").lines() {
        let (name, formula) = l.split_once(": ").unwrap();
        let monkey = get_or_create_monkey(name, &mut monkeys_map);
        match *monkey.borrow() {
            Monkey::Init => {},
            _ => { panic!("monkey should be just initialized!"); }
        }
        
        if let Ok(value) = formula.parse::<i64>() {
            *monkey.borrow_mut() = Monkey::Evaluated(value);
        } else {
            let mut iter_terms = formula.split(" ");
            let first_operand= iter_terms.next().unwrap().to_string();
            let operator = iter_terms.next().unwrap().to_string();
            let second_operand = iter_terms.next().unwrap().to_string();
            *monkey.borrow_mut() = Monkey::Operation { first_operand, operator, second_operand };
        }
    }
    println!("{}", eval(monkeys_map.get("root").unwrap(), &monkeys_map));

    // part 2
    println!("part 2");
    // Replace the operation of root to be subtraction.
    if let Some(root) = monkeys_map.get("root") {
        let new_root;
        match &*root.borrow() {
            Monkey::Operation { first_operand, operator: _, second_operand } => {
                new_root = Monkey::Operation {
                    first_operand: first_operand.clone(),
                    operator: "-".to_string(),
                    second_operand: second_operand.clone(),
                };
            },
            _ => { unreachable!(); }
        }
        *root.borrow_mut() = new_root;
    } else {
        unreachable!();
    }
    let root_formula = get_formula(&*monkeys_map.get("root").unwrap().borrow(), &monkeys_map);
    println!("{}/{} + {}/{} * h", root_formula.c0_num, root_formula.c0_den, root_formula.c1_num, root_formula.c1_den);
    println!("h = {}", -root_formula.c0_num * root_formula.c1_den / root_formula.c0_den / root_formula.c1_num);
}
