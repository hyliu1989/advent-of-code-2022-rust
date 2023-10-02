use std::rc::Rc;
use std::cell::RefCell;

struct LinkedList {
    data: i32,
    next: Option<Rc<RefCell<LinkedList>>>,
    prev: Option<Rc<RefCell<LinkedList>>>,
}

fn clone_option<T>(option: &Option<Rc<T>>) -> Option<Rc<T>> {
    match option {
        Some(inner) => Some(inner.clone()),
        None => None,
    }
}

/* Insert an item to the back of `node`.

Panicks if
- the item to insert has front/prev connection.
- the item to insert has back/next connection and `node` also has back/next connection.
*/
fn insert_back(node: Rc<RefCell<LinkedList>>, new_next: Rc<RefCell<LinkedList>>) {
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
    let item2_option;
    item2_option = clone_option(&item0.borrow().next);

    // Connect 0 and 1
    item0.borrow_mut().next = Some(item1.clone());
    item1.borrow_mut().prev = Some(item0.clone());
    // Try to connect 1 and 2 if needed.
    match item2_option {
        Some(item2) => {
            // Connect 1 and 2
            item1.borrow_mut().next = Some(item2.clone());
            item2.borrow_mut().prev = Some(item1.clone());
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
            insert_back(last_.clone(), curr.clone());
        }
        if num == 0 {
            element_zero = Some(curr.clone());
        }
        order.push(curr);
    }
    if let Some(last_) = order.last() {
        insert_back(last_.clone(), order[0].clone());
    }
    let order = order;
    let element_zero = element_zero.unwrap();
    
    // Start moving
    for element in order {
        let moves = element.borrow().data.clone();
        if moves == 0 {
            continue;
        }
        let get_moved_1 = if moves > 0 {
            |cursor: &Rc<RefCell<LinkedList>>| {
                cursor.borrow().next.as_ref().unwrap().clone()
            }
        } else {
            |cursor: &Rc<RefCell<LinkedList>>| {
                cursor.borrow().prev.as_ref().unwrap().clone()
            }
        };
        let mut cursor = element.borrow().prev.as_ref().unwrap().clone();
        remove(&element);
        for _ in 0..moves.abs() {
            cursor = get_moved_1(&cursor.clone());
        }
        // Destination is cursor.next
        if cursor.borrow().next.as_ref().unwrap().borrow().data != element.borrow().data {
            insert_back(cursor, element);
        }
    }
    
    // Debug
    {
        let mut cursor = element_zero.clone();
        println!("++{}", cursor.borrow().data);
        for _ in 1..6 {
            let to_assign = cursor.borrow().next.as_ref().unwrap().clone();
            cursor = to_assign;
            println!("++{}", cursor.borrow().data);
        }
    }

    // Collecting info
    let mut cursor = element_zero;
    let mut ret = 0;
    for i in 1..=3000 {
        let next_ = cursor.borrow().next.as_ref().unwrap().clone();
        cursor = next_;
        if i == 1000 || i == 2000 || i == 3000 {
            ret += cursor.borrow().data;
            println!("=*= {}", cursor.borrow().data);
        }
    }
    println!("{}", ret);
}
