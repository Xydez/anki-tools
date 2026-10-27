use std::collections::{HashMap, HashSet};

use crate::database::{Database, Note};

pub struct DeckDiff {
    pub added: HashSet<i64>,
    pub removed: HashSet<i64>,
    pub changed: HashSet<i64>,
    pub unchanged: HashSet<i64>,
}

pub fn open(path: &str) -> Database {
    let file = std::fs::File::open(path).unwrap();
    let data = anki_pax::extract_archive(file).unwrap();

    Database::open_serialized(data)
        .unwrap()
}

pub fn find_deck_diff(left: &HashMap<i64, Note>, right: &HashMap<i64, Note>) -> DeckDiff {
    let left_keys = left.keys().copied().collect::<HashSet<_>>();
    let right_keys = right.keys().copied().collect::<HashSet<_>>();

    let removed = left_keys.difference(&right_keys).copied().collect::<HashSet<i64>>();
    let added = right_keys.difference(&left_keys).copied().collect::<HashSet<i64>>();
    let coexisting = left_keys.intersection(&right_keys).copied().collect::<HashSet<i64>>();

    let unchanged = coexisting
        .iter()
        .copied()
        .filter(|nid| left[nid].fields == right[nid].fields)
        .collect::<HashSet<i64>>();

    let changed = coexisting.difference(&unchanged).copied().collect::<HashSet<i64>>();

    DeckDiff { added, removed, changed, unchanged }
}

pub fn print_diff(a: &str, b: &str) {
    use console::style;

    let chardiff = diff::chars(a, b);

    //let term = console::Term::stdout();

    for d in &chardiff {
        match d {
            diff::Result::Left(x) => {
                print!("{}", style(x).on_red());
            }
            diff::Result::Both(x, y) => {
                assert_eq!(x, y);
                print!("{}", x);
            }
            diff::Result::Right(x) => {
                print!("{}", style(x).black().on_green().dim());
            }
        }
    }

    println!();
}
