//! A module for parsing, reading, and writing various secondary structure formats.

use std::error::Error;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use crate::secondary_structure;
use crate::secondary_structure::{get_matching_bracket, SecondaryStructureRecord};

// TODO: get_ct_string expects user to supply a paired sites array and sequence, should be a SecondaryStructureRecord?

/// Get a connect (CT) format string representation of a secondary structure and sequence.
/// 
/// # Examples
/// 
/// ```rust
/// use crate::rna_secondary_structure::secondary_structure;
/// use crate::rna_secondary_structure::io;
/// 
/// let mut ss : secondary_structure::SecondaryStructureRecord = "((..)..)".parse().unwrap();
/// ss.set_sequence("CGAACAAG".to_string());
/// ss.name = "example".to_string();
/// let ct_string_observed = io::get_ct_string(&ss);
/// 
/// let ct_string_expected =
/// ">example
/// 1	C	0	2	8	1
/// 2	G	1	3	5	2
/// 3	A	2	4	0	3
/// 4	A	3	5	0	4
/// 5	C	4	6	2	5
/// 6	A	5	7	0	6
/// 7	A	6	8	0	7
/// 8	G	7	9	1	8
/// ";
///
/// assert_eq!(ct_string_observed, ct_string_expected);
/// ```
pub fn get_ct_string(ss: &SecondaryStructureRecord) -> String {
    let it = ss.sequence.chars().zip(ss.paired.iter());

    let mut data = format!(">{}\n", ss.name);
    for (i, (c, j)) in it.enumerate() {
        data.push_str(&format!("{}\t{}\t{}\t{}\t{}\t{}\n", i + 1, c, i, i + 2, j, i + 1));
    }
    data
}

/// Reads a connect (CT) format string and returns a vector of SecondaryStructureRecord
///
/// # Examples
///
/// ```rust
/// use crate::rna_secondary_structure::io;
///
/// let ct_string =
/// ">example
/// 1	C	0	2	8	1
/// 2	G	1	3	5	2
/// 3	A	2	4	0	3
/// 4	A	3	5	0	4
/// 5	C	4	6	2	5
/// 6	A	5	7	0	6
/// 7	A	6	8	0	7
/// 8	G	7	9	1	8
/// ";
///
/// let ls = io::parse_ct_string(&ct_string.to_string());
/// let observed_ss = &ls[0];
/// let seq = "CGAACAAG";
/// let paired = vec![8, 5, 0, 0, 2, 0, 0, 1];
/// assert_eq!(observed_ss.name, "example");
/// assert_eq!(observed_ss.sequence, seq);
/// assert_eq!(observed_ss.paired, paired);
/// ```
pub fn parse_ct_string(ct_string: &String) -> Vec<SecondaryStructureRecord> {
    let mut ls: Vec<SecondaryStructureRecord> = Vec::new();
    let mut sequence = "".to_string();
    let mut paired = Vec::new();
    let mut name = "".to_string();
    for line in ct_string.lines() {
        let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
        if spl.len() > 0 && spl[0].starts_with(">") {
            if paired.len() > 0 {
                ls.push(SecondaryStructureRecord {
                    name: name.clone(),
                    sequence: sequence.to_string(),
                    paired: paired.clone(),
                });
                sequence = "".to_string();
                paired.clear();
            }
            name = line[1..].to_string();
        } else if spl.len() >= 6 && spl[0].parse::<i64>().is_ok() && spl[5].parse::<i64>().is_ok() {
            sequence.push_str(spl[1]);
            paired.push(spl[4].parse::<i64>().unwrap());
        }
    }
    if paired.len() > 0 {
        ls.push(SecondaryStructureRecord {
            name: name.clone(),
            sequence: sequence.to_string(),
            paired: paired.clone(),
        });
        paired.clear();
    }
    ls
}

/// Unsafe, does not work with pseudoknotted structures
pub fn get_dbn_string(paired: Vec<i64>) -> String {
    let mut dbn = "".to_string();
    for (i, j) in paired.iter().enumerate() {
        let i = i as i64;
        let j = *j;
        if j == 0 {
            dbn += ".";
        } else if i < j {
            dbn += "(";
        } else {
            println!("{} {}", &dbn, j - 1);
            let left = get_matching_bracket(dbn.chars().nth((j - 1) as usize).unwrap()).unwrap().to_string();
            dbn.push_str(&left);
        }
    }
    dbn.to_string()
}

/// Writes a single SecondaryStructureRecord to the specified path in connect (CT) format.
pub fn write_ct_file(path: &Path, ss: &secondary_structure::SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)?;

    file.write_all(get_ct_string(ss).as_bytes())?;
    Ok(())
}