use std::iter::Peekable;
use std::cell::Cell;
use std::cell::RefCell;
use std::collections::HashMap;


enum EntryInfo {
    File,
    Directory(HashMap<String, Entry>),
}

struct Entry {
    size: Cell<i64>,
    info: EntryInfo,
}

impl Entry {
    fn new_dir() -> Self {
        Self { size: Cell::new(0), info: EntryInfo::Directory(HashMap::new()) }
    }
    fn new_file(size: i64) -> Self {
        Self { size: Cell::new(size), info: EntryInfo::File }
    }
    fn get_map_mut(&mut self) -> Option<&mut HashMap<String, Entry>> {
        match &mut self.info {
            EntryInfo::File => { None },
            EntryInfo::Directory(ref mut map) => {
                Some(map)
            }
        }
    }
}


// This is an example of impl used here that partially specify the generic type I of Peekable<I>.

fn build_filesys_tree(data: &str) -> Entry {
    let mut lines = data.lines().peekable();
    // `lines` was previously a function parameter with type &mut Peekable<impl Iterator<Item = &'static str>>.
    // Equivalent function signatures
    // - fn part1<I>(lines: &mut Peekable<I>) where I: Iterator<Item = &'static str>
    // - fn part1(lines: &mut Peekable<impl Iterator<Item = &'static str>>)

    let mut root = Entry::new_dir();
    let mut dir_trace: Vec<&mut HashMap<String, Entry>> = vec![root.get_map_mut().unwrap()];
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
                let curr_dir_map = dir_trace.pop().unwrap();
                let dest_dir_map = curr_dir_map.get_mut(folder_name).unwrap().get_map_mut().unwrap();
                dir_trace.push(curr_dir_map);
                dir_trace.push(dest_dir_map);
            },
            // Populating.
            _ if &line[0..4] == "$ ls" => {
                let curr_dir_map = *dir_trace.last_mut().unwrap();
                ls_populate(&mut lines, *curr_dir_map);
            },
            // Report unrecognized.
            unrecognized => { println!("unrecognized line ({})! ", unrecognized); },
        }
    }
    root
}

fn ls_populate<I>(lines: &mut Peekable<I>, mut parent_folder_map: HashMap<String, Entry>)
where I: Iterator<Item = &'static str>
{
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

fn part1(data: &str) {
    let mut root = build_filesys_tree(data);

    // Update the size of directories
    let mut dir_entries = dfs_populate_size(&mut root);

    // Accumulate the total (repeated) size count of directories that are at most 100000.
    let accum = dir_entries.into_iter()
        .filter_map(|en| {
            let size = en.size.get();
            if size <= 100000 { Some(size) } else { None }
        })
        .sum::<i64>();
    println!("{}", accum);

    // println!("============");
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


/* Returns a Vec of references to directory Entry's. */
fn dfs_populate_size<'a>(dir_entry: &'a mut Entry) -> Vec<&'a Entry> {
    let mut accum_size = 0;
    let mut dir_entries: Vec<&'a Entry> = vec![];
    {
        let map = dir_entry.get_map_mut().unwrap();
        for entry in map.values_mut() {
            match entry.info {
                EntryInfo::File => {},
                EntryInfo::Directory(_) => {
                    let mut subdir_entries = dfs_populate_size(entry);
                    dir_entries.append(&mut subdir_entries);
                },
            }
            accum_size += entry.size.get();
        }
    }
    
    // dir_entry.size.set(accum_size);
    dir_entries.push(dir_entry);
    dir_entries
}
