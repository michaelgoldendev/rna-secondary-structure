//! A module for parsing Rfam alignment files that are available on the Rfam ftp server.
//! Example file: [ftp://ftp.ebi.ac.uk/pub/databases/Rfam/14.2/Rfam.seed.gz](ftp://ftp.ebi.ac.uk/pub/databases/Rfam/14.2/Rfam.seed.gz).

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use flate2::read::GzDecoder;

use crate::secondary_structure::SecondaryStructureRecord;

const START_RECORD_TAG: &str = "# STOCKHOLM";
const RFAM_ACCESSION_TAG: &str = "#=GF AC";
const RFAM_STRUCTURE_TAG: &str = "#=GC SS_cons";
const RFAM_CONSENSUS_TAG: &str = "#=GC RF";
const END_RECORD_TAG: &str = "//";

/// Reads a buffer representing an Rfam stockholm file and returns a vector of
/// SecondaryStructureRecords. Note that only the accession (AC), consensus secondary structure
/// (SS_cons)  and consensus sequence (RF) strings are parsed.
pub fn parse_rfam_stockholm(reader: impl BufRead) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    let mut ls: Vec<SecondaryStructureRecord> = Vec::new();
    let mut accession: Option<String> = None;
    let mut dotbracketstring: Option<String> = None;
    let mut consensus_sequence: Option<String> = None;
    for line in reader.lines() {
        let line = line.unwrap_or_default();
        if line.starts_with(START_RECORD_TAG) {} else if line.starts_with(RFAM_ACCESSION_TAG) {
            let (_tag, value) = line.split_at(RFAM_ACCESSION_TAG.len());
            accession = Some(value.trim().to_string());
        } else if line.starts_with(RFAM_CONSENSUS_TAG) {
            let (_tag, value) = line.split_at(RFAM_CONSENSUS_TAG.len());
            consensus_sequence = Some(value.trim().to_string());
        } else if line.starts_with(RFAM_STRUCTURE_TAG) {
            let (_tag, value) = line.split_at(RFAM_STRUCTURE_TAG.len());
            dotbracketstring = Some(value.trim().to_string());
        } else if line.starts_with(END_RECORD_TAG) {
            if accession.is_some() && dotbracketstring.is_some() && consensus_sequence.is_some() {
                let mut ss: SecondaryStructureRecord = dotbracketstring.unwrap().parse()?;
                ss.name = accession.unwrap();
                ss.set_sequence(consensus_sequence.unwrap());
                ls.push(ss);
            }
            accession = None;
            dotbracketstring = None;
            consensus_sequence = None;
        }
    }

    Ok(ls)
}

/// Reads a gzipped Rfam stockholm file and returns a vector of SecondaryStructureRecords.
/// Note that only the accession (AC), consensus secondary structure (SS_cons)  and consensus
/// sequence (RF) strings are parsed.
pub fn parse_rfam_stockholm_gz_file(gz_file: File) -> Result<Vec<SecondaryStructureRecord>, Box<dyn Error>> {
    let reader = BufReader::new(GzDecoder::new(gz_file));
    parse_rfam_stockholm(reader)
}