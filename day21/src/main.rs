use std::collections::HashMap;
use std::cell::RefCell;


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
    for value in 0..500000i64 {
        if let Some(humn) = monkeys_map.get("humn") {
            *humn.borrow_mut() = Monkey::Evaluated(value);
        } else {
            unreachable!();
        }
        if eval(monkeys_map.get("root").unwrap(), &monkeys_map) == 0 {
            println!("{}", value);
            break;
        }
    }
}
