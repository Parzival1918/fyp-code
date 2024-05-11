use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::structs::*;

pub fn parse_contents<P>(path: P) -> System
where
    P: AsRef<std::path::Path>,
{
    let file = File::open(path).expect("File not found");
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut atoms: Vec<Atom> = Vec::new();
    let mut box_: Box = Box::new(0.0, 0.0, 0.0);

    // Read lines until find Masses section
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.contains("Masses") {
            break;
        }

        if line.contains("xlo xhi") {
            let mut iter = line.split_whitespace();
            iter.next();
            box_.lx = iter.next().unwrap().parse().unwrap();
        } else if line.contains("ylo yhi") {
            let mut iter = line.split_whitespace();
            iter.next();
            box_.ly = iter.next().unwrap().parse().unwrap();
        } else if line.contains("zlo zhi") {
            let mut iter = line.split_whitespace();
            iter.next();
            box_.lz = iter.next().unwrap().parse().unwrap();
        }
    }

    // Check that the box is read
    if box_.lx == 0.0 {
        panic!("Box not found");
    }

    // Consume buffer until Atoms section
    loop {
        let line = lines.next().unwrap().unwrap();
        if line.contains("Atoms") {
            break;
        }
    }

    // Read atoms
    // Skip first line
    lines.next();
    loop {
        let line = match lines.next() {
            Some(line) => line.unwrap(),
            None => break,
        };

        if line.is_empty() {
            break;
        }

        let mut iter = line.split_whitespace();
        let id = iter.next().unwrap().parse().unwrap();
        let molecule_id = iter.next().unwrap().parse().unwrap();
        let atom_type = iter.next().unwrap().parse().unwrap();
        iter.next(); // Skip atom charge
        let x = iter.next().unwrap().parse().unwrap();
        let y = iter.next().unwrap().parse().unwrap();
        let z = iter.next().unwrap().parse().unwrap();

        atoms.push(Atom::new(
            id,
            Some(molecule_id),
            atom_type,
            Position::new(x, y, z),
        ));
    }

    System::new(atoms, box_)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_contents() {
        let path = std::path::Path::new("test-data/data.lmp");
        let system = parse_contents(path);
        assert_eq!(system.atoms.len(), 25250);
        assert_eq!(system.box_.lx, 4.6145724058839996e+01);
        assert_eq!(system.box_.ly, 4.6783554974649995e+01);
        assert_eq!(system.box_.lz, 2.0000000000000000e+02);
    }
}
