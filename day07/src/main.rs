use std::iter::Peekable;
use std::cell::Cell;
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::HashMap;

#[derive(PartialEq)]
enum EntryInfo {
    File,
    Directory,
}

struct Entry {
    size: Cell<i64>,
    info: EntryInfo,
    dir_content: Option<HashMap<String, Rc<RefCell<Entry>>>>,
}

impl Entry {
    fn new_dir() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            Self { size: Cell::new(0), info: EntryInfo::Directory, dir_content: Some(HashMap::new()) }
        ))
    }
    fn new_file(size: i64) -> Rc<RefCell<Entry>> {
        Rc::new(RefCell::new(
            Self { size: Cell::new(size), info: EntryInfo::File, dir_content: None }
        ))
    }
}


fn main() {
    let data = include_str!("../input.txt");
    part1(&mut data.lines().peekable());
}

fn part1<I>(lines: &mut Peekable<I>)
where I: Iterator<Item = &'static str>
{
    let root = Entry::new_dir();

    let mut dir_trace: Vec<Rc<RefCell<Entry>>> = Vec::new();
    dir_trace.push(root.clone());

    while let Some(line) = lines.next() {
        match line {
            // Navigation.
            "$ cd .." => {
                if dir_trace.len() != 1 {
                    dir_trace.pop();
                }
            },
            "$ cd /" => {
                while dir_trace.len() != 1 {
                    dir_trace.pop();
                }
            },
            command if &line[0..4] == "$ cd" => {
                let folder_name = command.split_once("$ cd ").unwrap().1;
                let _curr_dir = dir_trace.last().unwrap().clone();
                let curr_dir = _curr_dir.borrow();
                assert!(curr_dir.info == EntryInfo::Directory);
                if let Some(ref map) = curr_dir.dir_content {
                    let dest_dir = map.get(folder_name).unwrap();
                    dir_trace.push(dest_dir.clone());
                }
            },
            // Populating.
            _ if &line[0..4] == "$ ls" => {
                let curr_dir = dir_trace.last().unwrap();
                ls_populate(lines, &mut *curr_dir.borrow_mut());
            },
            // Report unrecognized.
            unrecognized => { println!("unrecognized line ({})! ", unrecognized); },
        }
    }

    // Update the size of directories
    while dir_trace.len() != 1 {
        dir_trace.pop();
    }
    dfs_populate_size(root.clone(), &mut dir_trace);

    // Accumulate the total (repeated) size count of directories that are at most 100000.
    let mut accum = 0;
    for dir in dir_trace.iter() {
        let size = dir.borrow().size.get();
        if size <= 100000 {
            accum += size;
        }
    }
    println!("{}", accum);

    println!("============");
    let current_used_space = dir_trace[0].borrow().size.get();
    let current_space = 70_000_000 - current_used_space;

    dir_trace.sort_by_key(|a| { a.borrow().size.get() });
    for d in dir_trace {
        let size = d.borrow().size.get();
        if size >= 30000000 - current_space {
            println!("part 2 {}", size);
            break;
        }
    }
}

fn ls_populate<I>(lines: &mut Peekable<I>, parent_folder: &mut Entry)
where I: Iterator<Item = &'static str>
{
    assert!(parent_folder.info == EntryInfo::Directory);
    if let Some(ref mut map) = parent_folder.dir_content {
        while let Some(line) = lines.next_if(|&l| {l.chars().nth(0).unwrap() != '$'}) {
            let (type_or_size, name) = line.split_once(' ').unwrap();
            match type_or_size.parse::<i64>() {
                Ok(size) => { map.insert(String::from(name), Entry::new_file(size)); },
                Err(_) => { map.insert(String::from(name), Entry::new_dir()); },
            }
        }
    }
}

fn dfs_populate_size(dir_entry: Rc<RefCell<Entry>>, trace: &mut Vec<Rc<RefCell<Entry>>>) {
    let mut accum_size = 0;
    assert!(dir_entry.borrow().info == EntryInfo::Directory);
    if let Some(ref map) = dir_entry.borrow().dir_content {
        for entry in map.values() {
            match entry.borrow().info {
                EntryInfo::File => {},
                EntryInfo::Directory => {
                    trace.push(entry.clone());
                    dfs_populate_size(entry.clone(), trace);
                },
            }
            accum_size += entry.borrow().size.get();
        }
    }
    dir_entry.borrow().size.set(accum_size);
}
