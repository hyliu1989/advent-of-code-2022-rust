use std::iter::Peekable;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;


enum EntryInfo {
    File,
    Directory(RefCell<HashMap<String, Entry>>),
}

struct Entry {
    size: Cell<i64>,
    info: EntryInfo,
}

impl Entry {
    fn new_dir() -> Self {
        Self { size: Cell::new(0), info: EntryInfo::Directory(RefCell::new(HashMap::new())) }
    }
    fn new_file(size: i64) -> Self {
        Self { size: Cell::new(size), info: EntryInfo::File }
    }
    fn get_map<'a>(&'a self) -> Option<&'a RefCell<HashMap<String, Entry>>> {
        match self.info {
            EntryInfo::File => { None },
            EntryInfo::Directory(ref map) => {
                Some(map)
            }
        }
    }
}


// This is an example of impl used here that partially specify the generic type I of Peekable<I>.
fn build_filesys_tree(data: &'static str) -> Entry {
    let mut lines = data.lines().peekable();
    // `lines` was previously a function parameter with type &mut Peekable<impl Iterator<Item = &'static str>>.
    // Equivalent function signatures
    // - fn part1<I>(lines: &mut Peekable<I>) where I: Iterator<Item = &'static str>
    // - fn part1(lines: &mut Peekable<impl Iterator<Item = &'static str>>)

    let mut dir_trace: Vec<(&str, Entry)> = vec![("/", Entry::new_dir())];
    let modify_and_pop = |trace: &mut Vec<(&str, Entry)>| {
        if trace.len() > 1 {
            let (name, popped_entry) = trace.pop().unwrap();
            let parent_of_popped = &trace.last().unwrap().1.info;
            match parent_of_popped {
                EntryInfo::File => { panic!("trace should contain no File!"); },
                EntryInfo::Directory(map) => {
                    map.borrow_mut().insert(name.into(), popped_entry);
                },
            }
        }
    };

    while let Some(line) = lines.next() {
        match line {
            // Navigation.
            "$ cd .." => { modify_and_pop(&mut dir_trace); },
            "$ cd /" => {
                while dir_trace.len() != 1 {
                    modify_and_pop(&mut dir_trace);
                }
            },
            command if &line[0..4] == "$ cd" => {
                let folder_name = command.split_once("$ cd ").unwrap().1;
                let curr_dir = &dir_trace.last().unwrap().1;
                let dest_dir = curr_dir.get_map().unwrap().borrow_mut().remove(folder_name).unwrap();
                dir_trace.push((folder_name, dest_dir));
            },
            // Populating.
            _ if &line[0..4] == "$ ls" => {
                let parent_folder = &mut dir_trace.last_mut().unwrap().1;
                ls_populate(&mut lines, parent_folder);
            },
            // Report unrecognized.
            unrecognized => { println!("unrecognized line ({})! ", unrecognized); },
        }
    }
    while dir_trace.len() != 1 {
        modify_and_pop(&mut dir_trace);
    }
    dir_trace.pop().unwrap().1
}

fn ls_populate<I>(lines: &mut Peekable<I>, parent_folder: &Entry)
where I: Iterator<Item = &'static str>
{
    let mut parent_folder_map = parent_folder.get_map().unwrap().borrow_mut();
    while let Some(line) = lines.next_if(|&l| {l.chars().nth(0).unwrap() != '$'}) {
        let (type_or_size, name) = line.split_once(' ').unwrap();
        match type_or_size.parse::<i64>() {
            Ok(size) => { parent_folder_map.insert(String::from(name), Entry::new_file(size)); },
            Err(_) => { parent_folder_map.insert(String::from(name), Entry::new_dir()); },
        }
    }
}

fn main() {
    let data = include_str!("../input.txt");
    part1(data);
}

fn part1(data: &'static str) {
    let mut root = build_filesys_tree(data);

    // // Update the size of directories
    let dir_sizes = dfs_populate_size(&mut root);

    // Accumulate the total (repeated) size count of directories that are at most 100000.
    let accum = dir_sizes.into_iter()
        .filter_map(|size| {
            if size <= 100000 { Some(size) } else { None }
        })
        .sum::<i64>();
    println!("{}", accum);

    // 
    // let current_used_space = dir_trace[0].borrow().size.get();
    // let current_space = 70_000_000 - current_used_space;

    // dir_trace.sort_by_key(|a| { a.borrow().size.get() });
    // for d in dir_trace {
    //     let size = d.borrow().size.get();
    //     if size >= 30000000 - current_space {
    //         println!("part 2 {}", size);
    //         break;
    //     }
    // }
}


/* Returns a Vec of directory sizes. */
fn dfs_populate_size(dir_entry: &Entry) -> Vec<i64> {
    let mut accum_size = 0;
    let mut dir_sizes: Vec<i64> = vec![];
    
    let map = dir_entry.get_map().unwrap().borrow_mut();
    for entry in map.values() {
        match entry.info {
            EntryInfo::File => {},
            EntryInfo::Directory(_) => {
                let mut subdir_sizes = dfs_populate_size(entry);
                dir_sizes.append(&mut subdir_sizes);
            },
        }
        accum_size += entry.size.get();
    }
    
    dir_entry.size.set(accum_size);
    dir_sizes.push(accum_size);
    dir_sizes
}
