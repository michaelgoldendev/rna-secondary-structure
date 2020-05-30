//! A module for parsing, reading, and writing various secondary structure formats.

use std::error::Error;
use std::fs::OpenOptions;
use std::io;
use std::path::{Path};

use crate::secondary_structure;
use crate::secondary_structure::{get_matching_bracket, LEFT_BRACKETS, PairedSites, SecondaryStructureRecord, StructureParseError};
use std::iter::*;

/// Reads a connect (CT) format string and returns a vector of SecondaryStructureRecords.
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

fn write_ct(buffer: &mut dyn io::Write, ss: &SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    let it = ss.sequence.chars().zip(ss.paired.iter());

    buffer.write(format!(">{}\n", ss.name).as_bytes())?;
    for (i, (c, j)) in it.enumerate() {
        buffer.write(format!("{}\t{}\t{}\t{}\t{}\t{}\n", i + 1, c, i, i + 2, j, i + 1).as_bytes())?;
    }
    Ok(())
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
    write_ct(&mut file, ss)?;

    Ok(())
}

/// Write a collection of SecondaryStructureRecords to a buffer in connect (CT) format.
pub fn write_records_to_ct_buffer<'a, I>(buffer: &mut dyn io::Write, records : I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = &'a &'a SecondaryStructureRecord>
{
    for ss in records
    {
        write_ct(buffer, ss)?;
    }
    Ok(())
}

/// Write a collection of SecondaryStructureRecords to the specified file path in connect (CT)
/// format.
pub fn write_records_to_ct_file<'a, I>(path: &Path, records : I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item = &'a &'a SecondaryStructureRecord>
{
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)?;
    write_records_to_ct_buffer(&mut file, records)?;

    Ok(())
}



/// Get a connect (CT) format string representation of a secondary structure and sequence.
/// The CT format can represent arbitarily pseudoknotted secondary structures.
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
    let mut bytes = Vec::new();
    write_ct(&mut bytes, ss).unwrap();
    String::from_utf8(bytes).unwrap()
}

/// Converts a paired sites list representing an arbitarily pseudoknotted secondary structure into
/// a dot bracket string representation.
///
/// # Examples
///
/// ```rust
/// use crate::rna_secondary_structure::io;
///
/// let paired = vec![5, 7, 6, 9, 1, 3, 2, 10, 4, 8, 0, 0];
///
/// let dbs_observed = io::get_dot_bracket_string(&paired).unwrap();
/// let dbs_expected = "(<<{)>>(})..";
/// assert_eq!(dbs_observed, dbs_expected);
/// ```
pub fn get_dot_bracket_string(paired: &dyn PairedSites) -> Result<String, StructureParseError> {
    let paired = paired.paired();

    let mut stacks: Vec<Vec<i64>> = Vec::new();
    stacks.push(Vec::new());

    let mut dbn = "".to_string();
    for (i, j) in paired.iter().enumerate() {
        let i = i as i64;
        let j = *j;
        if j == 0 {
            dbn += ".";
        } else if i < j {
            let mut success = false;
            for (index, left) in LEFT_BRACKETS.chars().enumerate() {
                if index >= stacks.len() {
                    stacks.push(Vec::new()); // add an new stack for an additional bracket type
                }
                let stack = stacks.get(index).unwrap();
                if stack.len() == 0 || j < *stack.last().unwrap() {
                    stacks.get_mut(index).unwrap().push(j);
                    dbn += &left.to_string();
                    success = true;
                    break;
                }
            }
            if !success {
                return Err(StructureParseError::InsufficientBracketTypes);
            }
        } else {
            let left = dbn.chars().nth((j - 1) as usize).unwrap();
            let index = LEFT_BRACKETS.find(left).unwrap();
            stacks.get_mut(index).unwrap().pop();

            let right = get_matching_bracket(left).unwrap();
            dbn.push_str(&right.to_string());
        }
    }
    Ok(dbn.to_string())
}

/// Convert a secondary structure to it's full dot bracket structure representation and write it to
/// a buffer.
pub fn write_full_dot_bracket_repr(buffer: &mut dyn io::Write, ss: &SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    buffer.write(format!(">{}", &ss.name).as_bytes())?;
    buffer.write("\n".as_bytes())?;
    buffer.write(&ss.sequence.as_bytes())?;
    buffer.write("\n".as_bytes())?;
    buffer.write(get_dot_bracket_string(&ss.paired)?.as_bytes())?;
    buffer.write("\n".as_bytes())?;
    Ok(())
}

/// Convert a secondary structure to it's full dot bracket structure representation and write it to
/// the specified file path.
pub fn write_dbn_file(path: &Path, ss: &secondary_structure::SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)?;
    write_full_dot_bracket_repr(&mut file, ss)?;

    Ok(())
}

/// Write a collection of SecondaryStructureRecords to a buffer in full dot bracket notation format.
pub fn write_records_to_dbn<'a, I>(buffer: &mut dyn io::Write, records : I) -> Result<(), Box<dyn Error>>
where
    I: IntoIterator<Item = &'a &'a SecondaryStructureRecord>
{
    for ss in records
    {
        write_full_dot_bracket_repr(buffer, ss)?;
    }
    Ok(())
}

/// Write a collection of SecondaryStructureRecords to the specified file path in full dot bracket
/// notation format.
pub fn write_records_to_dbn_file<'a, I>(path: &Path, records : I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item = &'a &'a SecondaryStructureRecord>
{
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)?;
    write_records_to_dbn(&mut file, records)?;

    Ok(())
}

