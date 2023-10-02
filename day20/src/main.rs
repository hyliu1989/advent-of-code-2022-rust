use std::rc::Rc;
use std::cell::RefCell;

struct LinkedList {
    data: i32,
    next: Option<Rc<RefCell<LinkedList>>>,
    prev: Option<Rc<RefCell<LinkedList>>>,
}

fn clone_option<T>(option: &Option<Rc<T>>) -> Option<Rc<T>> {
    match option {
        Some(inner) => Some(Rc::clone(inner)),
        None => None,
    }
}

/* Insert an item to the back of `node`.

Panicks if
- the item to insert has front/prev connection.
- the item to insert has back/next connection and `node` also has back/next connection.
*/
fn insert_back(node: &Rc<RefCell<LinkedList>>, new_next: &Rc<RefCell<LinkedList>>) {
    if let Some(_) = new_next.borrow().prev {
        panic!("The item to be append has another front connection.");
    }
    if let Some(_) = new_next.borrow().next {
        if let Some(_) = node.borrow().next {
            panic!("Both node and item has back/next connection.");
        }
    }

    let item0 = node;
    let item1 = new_next;
    let item2_option = clone_option(&item0.borrow().next);

    // Connect 0 and 1
    item0.borrow_mut().next = Some(Rc::clone(item1));
    item1.borrow_mut().prev = Some(Rc::clone(item0));
    // Try to connect 1 and 2 if needed.
    match item2_option.as_ref() {
        Some(item2) => {
            // Connect 1 and 2
            item1.borrow_mut().next = Some(Rc::clone(item2));
            item2.borrow_mut().prev = Some(Rc::clone(item1));
        },
        None => {
            // Do nothing and use the existing .next of item1.
        },
    }
}

fn remove(node: &Rc<RefCell<LinkedList>>) {
    {
        let item0_option = clone_option(&node.borrow().prev);
        let item1 = node;
        let item2_option = clone_option(&node.borrow().next);
        if let Some(item0) = &item1.borrow().prev {
            RefCell::borrow_mut(item0).next = item2_option;
        }
        if let Some(item2) = &item1.borrow().next {
            RefCell::borrow_mut(item2).prev = item0_option;
        }
        item1.borrow_mut().prev = None;
        item1.borrow_mut().next = None;
    }
}


fn mix(order: &Vec<Rc<RefCell<LinkedList>>>, multiplier: i64) {
    for element in order.iter() {
        let mut num_moves = element.borrow().data.clone() as i64;
        if num_moves == 0 {
            continue;
        }
        num_moves =(num_moves * multiplier) % (order.len() as i64 - 1);
        if num_moves < 0 {
            num_moves += order.len() as i64 - 1;
        }
        assert!(num_moves >= 0);

        let get_moved_1 = |cursor: &Rc<RefCell<LinkedList>>| {
            Rc::clone(cursor.borrow().next.as_ref().unwrap())
        };
        // Let `cursor` represent where to back insert the `element`.
        let mut cursor = Rc::clone(element.borrow().prev.as_ref().unwrap());
        remove(element);
        for _ in 0..num_moves.abs() {
            cursor = get_moved_1(&cursor);
        }
        // Back-insert the element.
        insert_back(&cursor, element);
    }
}

const CASE: u8 = 1;

fn main() {
    let data = include_str!("../input.txt");
    let mut order: Vec<Rc<RefCell<LinkedList>>> = Vec::new();
    let mut element_zero = None;
    for num_str in data.lines() {
        let num: i32 = num_str.parse().unwrap();
        let curr = Rc::new(RefCell::new(
            LinkedList { data: num, next: None, prev: None }
        ));
        if let Some(last_) = order.last() {
            insert_back(last_, &curr);
        }
        if num == 0 {
            element_zero = Some(Rc::clone(&curr));
        }
        order.push(curr);
    }
    if let Some(last_) = order.last() {
        insert_back(last_, &order[0]);
    }
    let order = order;
    let element_zero = element_zero.unwrap();
    
    let multiplier: i64;
    if CASE == 0 {
        multiplier = 1;
        mix(&order, multiplier);
    } else {
        multiplier = 811589153;
        for _ in 0..10 {
            mix(&order, multiplier);
        }
    }

    // Collecting info
    let mut cursor = element_zero;
    let mut ret = 0i64;
    for i in 1..=3000 {
        let next_ = Rc::clone(cursor.borrow().next.as_ref().unwrap());
        cursor = next_;
        if i == 1000 || i == 2000 || i == 3000 {
            ret += cursor.borrow().data as i64 * multiplier;
            println!("=*= {}", cursor.borrow().data);
        }
    }
    println!("{}", ret);
}
