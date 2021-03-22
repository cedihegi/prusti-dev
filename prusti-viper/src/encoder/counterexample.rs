use std::collections::HashMap;
use std::fmt;
use rustc_span::MultiSpan;
use prusti_interface::PrustiError;

pub enum Counterexample {
    Success{
        result: Entry,
        args: HashMap<(String, MultiSpan), Entry>,
        entries: HashMap<(String, MultiSpan), Entry>,
        is_pure: bool,
    },
    Failure(String),
}

impl Counterexample{
    pub fn apply_prusti_error(&self, prusti_error: &mut PrustiError) {
        if let Counterexample::Success{result, args, entries, is_pure} = self {
            if !*is_pure {
                for (place, entry) in entries {
                    //place is a tuple (Name of the variable, Option<Scope>)
                    if let Some(entry_arg) = args.get(place) {
                        let note = format!("counterexample for \"{0}\"\ninitial: {0} <- {1}\nfinal: {0} <- {2}", 
                            place.0, 
                            entry_arg,  
                            entry,
                        );
                        prusti_error.add_note(note, Some(place.1.clone()));
                    } else {
                        let note = format!("counterexample for \"{0}\"\n{0} <- {1}", place.0, entry);
                        prusti_error.add_note(note, Some(place.1.clone()));
                    }
                }
                let result_note = format!{"result <- {}", result};
                prusti_error.add_note(result_note, None);
            } else {
                for (place, entry) in args {
                    let note = format!("counterexample for \"{0}\"\n{0} <- {1}", 
                            place.0,   
                            entry,
                        );
                    prusti_error.add_note(note, Some(place.1.clone()));
                }
                // Todo: find span of return type to give this note a span
                let result_note = format!("result <- {}", result);
                prusti_error.add_note(result_note, None);
            }
        } 
    }
}

impl fmt::Display for Counterexample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Counterexample::Success{result, args, entries, is_pure} => {
                write!(f, "Counterexample:\n");
                write!(f, "initial args:\n");
                for (place, entry) in args {
                    let s = format!("{} <- {}\n", place.0, entry);
                    write!(f, "{}", indent(s));
                }
                write!(f, "\nlocal values when failing:\n");
                for (place, entry) in entries {
                    let s = format!("{} <- {}\n", place.0, entry);
                    write!(f, "{}", indent(s));
                }
                write!(f, "\nresult <- {}", result)
            },
            _ => write!(f, "Counterexample generation failed")
        }
    }
}

#[derive(Debug)]
pub enum Entry{
    IntEntry{value: i64},
    BoolEntry{value: bool},
    CharEntry{value: char},
    RefEntry{el: Box<Entry>},
    Struct{
        name: String,
        field_names: Vec<String>,
        field_entries: Vec<Entry>
    },
    Enum{
        super_name: String,
        name: String,
        named_fields: bool,
        field_names: Vec<String>,
        field_entries: Vec<Entry>
        //note: if fields are not named, their order is important!
        //that is why no HashMap is used
    },
    Tuple{
        fields: Vec<Entry>
    },
    Unit,
    UnknownEntry
}

impl fmt::Display for Entry{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Entry::IntEntry { value } => write!(f, "{}", value),
            Entry::BoolEntry { value } => write!(f, "{}", value),
            Entry::CharEntry { value } => write!(f, "'{}' ({:x})", value, *value as i32),
            Entry::RefEntry { el } => write!(f, "ref({})", indent(el.to_string())),
            Entry::Enum { super_name, name, named_fields, field_names, field_entries } => {
                write!(f, "{}::{}", super_name, name);
                let length = field_entries.len();
                let mut fields_string = "".to_owned();
                if length > 0{
                    if *named_fields {
                        fields_string.push_str("{");
                        for i in 0..length{
                            let s = format!("\n{} <- {}", field_names[i], field_entries[i]);
                            fields_string.push_str(&s);
                        }
                        write!(f, "{}", indent(fields_string));
                        write!(f, "}}")
                    } else {
                        write!(f, "(");
                        let len = length - 1;
                        for (i, entry) in (*field_entries).iter().enumerate(){
                            if i < len {
                                write!(f, "{}, ", entry);
                            } else {
                                write!(f, "{}", entry);
                            }
                        }
                        write!(f, ")")
                    }
                } else {
                    write!(f, "")
                }
            }
            Entry::Struct { name, field_names, field_entries} => {
                write!(f, "{} {{", name);
                let len = field_names.len();
                let mut fields_str = "".to_owned();
                for i in 0..len{
                    let s = format!("\n{} <- {}", field_names[i], field_entries[i]);
                    fields_str.push_str(&s);
                }
                write!(f, "{}}}\n", indent(fields_str))
            },
            Entry::Tuple { fields } => {
                write!(f, "(");
                let len = (*fields).len() - 1;
                for (i, entry) in (*fields).iter().enumerate(){
                    if i < len {
                        write!(f, "{}, ", entry);
                    } else {
                        write!(f, "{}", entry);
                    }
                }
                write!(f, ")")
            },
            Entry::Unit => write!(f, "()"),
            _ => write!(f, "?")
        }
    }
}

//for printing multiline-entries, indents all but the first line
fn indent(s: String) -> String {
    let mut res = "".to_owned();
    let length = s.lines().count();
    let mut lines = s.lines();
    if length > 1 {
        res.push_str(lines.next().unwrap());
        res.push_str("\n");
        while let Some(l) = lines.next() {
            res.push_str("  ");
            res.push_str(l);
            res.push_str("\n");
        }
        res
    } else {
        s
    }
}

