use crate::util::{self, DeckDiff};

pub fn run_compare_files(left: &str, right: &str) {
    let db1 = util::open(left);
    let db2 = util::open(right);

    let n1 = db1.get_notes().unwrap();
    let n2 = db2.get_notes().unwrap();

    let note_types = db1.get_note_types().unwrap();

    let DeckDiff { added, removed, unchanged, changed } = util::find_deck_diff(&n1, &n2);

    println!("===== Unchanged =====");
    for nid in &unchanged {
        println!("- {nid}");
    }
    println!("===== Removed =====");
    for nid in &removed {
        println!("- {} - {:?}", console::style(nid).red(), n1[nid].fields);
    }
    println!("===== Added =====");
    for nid in &added {
        println!("- {} - {:?}", console::style(nid).green(), n2[nid].fields);
    }
    println!("===== Changed =====");
    for nid in &changed {
        let na = &n1[nid];
        let nb = &n2[nid];

        println!("- {}", console::style(nid).bold());

        if na.ntid == nb.ntid {
            for i in 0..na.fields.len() {
                let field = &note_types[&na.ntid].fields[i];

                // let field = note_types[&na.ntid]
                //     .map(std::borrow::Cow::from)
                //     .unwrap_or_else(|| std::borrow::Cow::from(format!("<{}>", na.ntid)));

                //dbg!(&na.fields, &nb.fields);

                if na.fields[i] != nb.fields[i] {
                    let naf = &na.fields[i];
                    let nbf = &nb.fields[i];


                    println!("  * Field: {field}");
                    println!("    * FROM: {:?}", naf);
                    println!("    * TO:   {:?}", nbf);


                    print!("    * => ");
                    util::print_diff(naf, nbf);
                } else {
                    println!("  * Field: {field} (equal)");
                }
            }
        } else {
            todo!("implement changed note type");
        }

        //println!("- {nid}\nFROM:  {:?}\nTO:    {:?}", n1[nid].fields, n2[nid].fields);
    }

    // TODO: Text comparison (or just use diff? or a library?)
    // See: https://en.wikipedia.org/wiki/Longest_common_subsequence
}
