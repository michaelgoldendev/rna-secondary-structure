//! A module for parsing, reading, and writing various secondary structure formats.

use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{BufRead, BufReader};
use std::iter::*;
use std::path::Path;

use crate::secondary_structure;
use crate::secondary_structure::{from_dotbracketstring, get_dot_bracket_string, SecondaryStructureRecord, StructureParseError};

fn parse_ct(reader: impl BufRead) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    let mut ls: Vec<SecondaryStructureRecord> = Vec::new();
    let mut sequence = "".to_string();
    let mut paired = Vec::new();
    let mut name = "".to_string();
    for line in reader.lines() {
        let line = line?;
        let spl = line.trim().split_whitespace().collect::<Vec<&str>>();
        if !spl.is_empty() && spl[0].starts_with('>') {
            if !paired.is_empty() {
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
    if !paired.is_empty() {
        ls.push(SecondaryStructureRecord {
            name,
            sequence,
            paired: paired.clone(),
        });
        paired.clear();
    }
    Ok(ls)
}

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
/// let observed_ss = &io::parse_ct_string(&ct_string.to_string()).unwrap()[0];
/// assert_eq!(observed_ss.name, "example");
/// assert_eq!(observed_ss.sequence, "CGAACAAG");
/// assert_eq!(observed_ss.paired, vec![8, 5, 0, 0, 2, 0, 0, 1]);
/// ```
pub fn parse_ct_string(ct_string: &String) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    parse_ct(ct_string.as_bytes())
}

/// Reads a connect (CT) format file and returns a vector of SecondaryStructureRecords.
pub fn read_ct_file(f: File) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    parse_ct(BufReader::new(f))
}

fn write_ct(buffer: &mut dyn io::Write, ss: &SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    let it = ss.sequence.chars().zip(ss.paired.iter());

    buffer.write_all(format!(">{}\n", ss.name).as_bytes())?;
    for (i, (c, j)) in it.enumerate() {
        buffer.write_all(format!("{}\t{}\t{}\t{}\t{}\t{}\n", i + 1, c, i, i + 2, j, i + 1).as_bytes())?;
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
pub fn write_records_to_ct_buffer<'a, I>(buffer: &mut dyn io::Write, records: I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item=&'a &'a SecondaryStructureRecord>
{
    for ss in records
    {
        write_ct(buffer, ss)?;
    }
    Ok(())
}

/// Write a collection of SecondaryStructureRecords to the specified file path in connect (CT)
/// format.
pub fn write_records_to_ct_file<'a, I>(path: &Path, records: I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item=&'a &'a SecondaryStructureRecord>
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
///
/// # Examples
/// 
/// ```rust
/// use crate::rna_secondary_structure::secondary_structure;
/// use crate::rna_secondary_structure::io;
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

/// Write the name, sequence, and secondary structure conformation (in dot bracket notation) of a
/// SecondaryStructureRecord to a buffer.
pub fn write_dbn(buffer: &mut dyn io::Write, ss: &SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    buffer.write_all(format!(">{}", &ss.name).as_bytes())?;
    buffer.write_all(b"\n")?;
    buffer.write_all(&ss.sequence.as_bytes())?;
    buffer.write_all(b"\n")?;
    buffer.write_all(get_dot_bracket_string(&ss.paired)?.as_bytes())?;
    buffer.write_all(b"\n")?;
    Ok(())
}

/// Write the name, sequence, and secondary structure conformation (in dot bracket notation) of a
/// SecondaryStructureRecord to the specified file path.
pub fn write_dbn_file(path: &Path, ss: &secondary_structure::SecondaryStructureRecord) -> Result<(), Box<dyn Error>> {
    let append = false;

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(append)
        .truncate(!append)
        .open(&path)?;
    write_dbn(&mut file, ss)?;

    Ok(())
}

/// Write a collection of SecondaryStructureRecords to a buffer in full dot bracket notation format.
pub fn write_records_to_dbn<'a, I>(buffer: &mut dyn io::Write, records: I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item=&'a &'a SecondaryStructureRecord>
{
    for ss in records
    {
        write_dbn(buffer, ss)?;
    }
    Ok(())
}

/// Write a collection of SecondaryStructureRecords to the specified file path in full dot bracket
/// notation format.
pub fn write_records_to_dbn_file<'a, I>(path: &Path, records: I) -> Result<(), Box<dyn Error>>
    where
        I: IntoIterator<Item=&'a &'a SecondaryStructureRecord>
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

fn parse_dbn(reader: impl BufRead) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    let mut ls: Vec<SecondaryStructureRecord> = Vec::new();
    let mut sequence = "".to_string();
    let mut name = "".to_string();
    let mut m = 0;
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            if m == 0 || m == 3 {
                m = 0;
            } else if m == 1 {
                return Err(Box::new(StructureParseError::ExpectedLine {
                    msg: "Expected a line containing a sequence. Found a blank line.".to_string()
                }));
            } else if m == 2 {
                return Err(Box::new(StructureParseError::ExpectedLine {
                    msg: "Expected a line containing a dot bracket string. Found a blank line.".to_string()
                }));
            }
        } else if m == 0 || m == 3 {
            name = line.trim_start_matches('>').to_string();
            m = 1;
        } else if m == 1 {
            sequence = line.to_string();
            m = 2;
        } else if m == 2 {
            ls.push(SecondaryStructureRecord {
                name: name.clone(),
                sequence: sequence.clone(),
                paired: from_dotbracketstring(line)?,
            });
            m = 3;
        }
    }

    Ok(ls)
}

/// Reads a dot bracket notation (dbn) format file and returns a vector of SecondaryStructureRecords.
pub fn read_dbn_file(f: File) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    parse_dbn(BufReader::new(f))
}


