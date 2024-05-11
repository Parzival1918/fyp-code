use std::fs::File;
use std::io::{BufReader, Lines};

use crate::structs::*;

pub fn next_step_content(
    line_it: &mut Lines<BufReader<flate2::read::GzDecoder<File>>>,
) -> Option<TrajSnapshot> {
    if let None = line_it.next() {
        // end of file
        return None;
    }
    let timestep: u32 = line_it.next().unwrap().unwrap().parse().unwrap();
    line_it.next();
    let num_atoms: u32 = line_it.next().unwrap().unwrap().parse().unwrap();
    line_it.next();
    let box_x: f64 = line_it
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let box_y: f64 = line_it
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let box_z: f64 = line_it
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse()
        .unwrap();
    let box_ = Box::new(box_x, box_y, box_z);
    line_it.next();
    let mut atoms: Vec<Atom> = Vec::new();
    for _ in 0..num_atoms {
        let line = line_it.next();
        let line = match line {
            Some(l) => match l {
                Ok(s) => s,
                Err(_) => return None,
            },
            None => return None,
        };
        let mut iter = line.split_whitespace();
        let id: u32 = iter.next().unwrap().parse().unwrap();
        let atom_type: u32 = iter.next().unwrap().parse().unwrap();
        let xs: f64 = iter.next().unwrap().parse().unwrap();
        let ys: f64 = iter.next().unwrap().parse().unwrap();
        let zs: f64 = iter.next().unwrap().parse().unwrap();
        atoms.push(Atom::new(
            id,
            None,
            atom_type,
            Position::new(xs * box_x, ys * box_y, zs * box_z),
        ));
    }

    Some(TrajSnapshot::new(System::new(atoms, box_), timestep))
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::BufRead;

    #[test]
    fn test_next_step_content() {
        let path = std::path::Path::new("test-data/prod_traj.lmp.gz");
        let file = File::open(path).unwrap();
        let file = flate2::read::GzDecoder::new(file);

        let reader = BufReader::new(file);
        let mut line_it = reader.lines();
        let snapshot = next_step_content(&mut line_it);
        // let snapshot = next_step_content(&mut line_it);
        // assert_eq!(snapshot.unwrap().step, 0);
        assert_eq!(snapshot.unwrap().system.atoms.len(), 9900);
        // assert_eq!(snapshot.unwrap().system.box_.lx, 5.0216000000000001e+01);
        // assert_eq!(snapshot.unwrap().system.atoms[0].atom_type, 1);
    }
}
