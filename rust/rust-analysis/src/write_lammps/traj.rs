use std::{collections::HashMap, fs::OpenOptions, io::Write};

use flate2::Compression;

use crate::structs::*;

pub fn save(filename: &str, snapshots: Vec<TrajSnapshot>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
    let mut file = flate2::write::GzEncoder::new(file, Compression::best());

    for snapshot in snapshots {
        file.write_all(format!("ITEM: TIMESTEP\n{}\n", snapshot.step).as_bytes())
            .unwrap();

        file.write_all(format!("ITEM: NUMBER OF ATOMS\n{}\n", snapshot.system.atoms.len()).as_bytes())
            .unwrap();

        file.write_all(
            format!(
                "ITEM: BOX BOUNDS xy xz yz pp pp pp\n0.0 {} 0.0\n0.0 {} 0.0\n0.0 {} 0.0\n",
                snapshot.system.box_.lx, snapshot.system.box_.ly, snapshot.system.box_.lz
            )
            .as_bytes(),
        )
        .unwrap();

        let mut count = 0u32;
        file.write_all("ITEM: ATOMS id type xs ys zs ix iy iz\n".as_bytes()).unwrap();
        for i in 0..snapshot.system.atoms.len() {
            let atom = &snapshot.system.atoms[i];
            count += 1;
            file.write_all(format!("{} {} {} {} {} 0 0 0\n", atom.id, atom.atom_type, atom.position.x/snapshot.system.box_.lx, atom.position.y/snapshot.system.box_.ly, atom.position.z/snapshot.system.box_.lz).as_bytes()).unwrap();
        }
        println!("Count: {count}");
    }

    file.finish().unwrap();
}

pub fn save_extra_prop<T>(filename: &str, snapshots: Vec<TrajSnapshot>, extra_props: Vec<HashMap<u32, T>>) 
where
    T: ToString
{
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(filename)
        .unwrap();
    let mut file = flate2::write::GzEncoder::new(file, Compression::best());

    let mut idx = 0usize;
    for snapshot in snapshots {
        file.write_all(format!("ITEM: TIMESTEP\n{}\n", snapshot.step).as_bytes())
            .unwrap();

        file.write_all(format!("ITEM: NUMBER OF ATOMS\n{}\n", snapshot.system.atoms.len()).as_bytes())
            .unwrap();

        file.write_all(
            format!(
                "ITEM: BOX BOUNDS xy xz yz pp pp pp\n0.0 {} 0.0\n0.0 {} 0.0\n0.0 {} 0.0\n",
                snapshot.system.box_.lx, snapshot.system.box_.ly, snapshot.system.box_.lz
            )
            .as_bytes(),
        )
        .unwrap();

        let mut count = 0u32;
        file.write_all("ITEM: ATOMS id type xs ys zs ix iy iz extra\n".as_bytes()).unwrap();
        for i in 0..snapshot.system.atoms.len() {
            let atom = &snapshot.system.atoms[i];
            count += 1;
            file.write_all(format!("{} {} {} {} {} 0 0 0 {}\n", atom.id, atom.atom_type, atom.position.x/snapshot.system.box_.lx, 
                atom.position.y/snapshot.system.box_.ly, atom.position.z/snapshot.system.box_.lz,
                extra_props[idx].get(&atom.id).unwrap().to_string()).as_bytes()).unwrap();
        }
        println!("Count: {count}");
        idx += 1;
    }

    file.finish().unwrap();
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Read;
    use super::*;

    #[test]
    fn test_save_traj() {
        let box_ = Box { lx: 5.0, ly: 5.0, lz: 5.0 };

        let atom1 = Atom {
            id: 1,
            molecule_id: None,
            atom_type: 1,
            position: Position { x: 1.0, y: 1.0, z: 1.0 },
        };
        let atom2 = Atom {
            position: Position { x: 2.0, y: 2.0, z: 2.0 },
            ..atom1.clone()
        };

        let system1 = System::new(vec![atom1], box_.clone());
        let system2 = System::new(vec![atom2], box_);

        let snapshot1 = TrajSnapshot::new(system1, 0);
        let snapshot2 = TrajSnapshot::new(system2, 1);

        save("test_save_traj.lmp.gz", vec![snapshot1, snapshot2]);

        let text = "\
ITEM: TIMESTEP
0
ITEM: NUMBER OF ATOMS
1
ITEM: BOX BOUNDS xy xz yz pp pp pp
0.0 5 0.0
0.0 5 0.0
0.0 5 0.0
ITEM: ATOMS id type xs ys zs ix iy iz
1 1 0.2 0.2 0.2 0 0 0
ITEM: TIMESTEP
1
ITEM: NUMBER OF ATOMS
1
ITEM: BOX BOUNDS xy xz yz pp pp pp
0.0 5 0.0
0.0 5 0.0
0.0 5 0.0
ITEM: ATOMS id type xs ys zs ix iy iz
1 1 0.4 0.4 0.4 0 0 0
";

        let file = File::open("test_save_traj.lmp.gz").unwrap();
        let mut file = flate2::read::GzDecoder::new(file);

        let mut file_text = String::new();
        file.read_to_string(&mut file_text).unwrap();

        assert_eq!(text, file_text);

        std::fs::remove_file("test_save_traj.lmp.gz").unwrap();
    }

    #[test]
    fn test_save_traj_change_atoms() {
        let box_ = Box { lx: 5.0, ly: 5.0, lz: 5.0 };

        let atom1 = Atom {
            id: 1,
            molecule_id: None,
            atom_type: 1,
            position: Position { x: 1.0, y: 1.0, z: 1.0 },
        };
        let atom2 = Atom {
            id: 2,
            position: Position { x: 2.0, y: 2.0, z: 2.0 },
            ..atom1.clone()
        };

        let system1 = System::new(vec![atom1.clone()], box_.clone());
        let system2 = System::new(vec![atom1, atom2], box_);

        let snapshot1 = TrajSnapshot::new(system1, 0);
        let snapshot2 = TrajSnapshot::new(system2, 1);

        save("test_save_traj_change_atoms.lmp.gz", vec![snapshot1, snapshot2]);

        let text = "\
ITEM: TIMESTEP
0
ITEM: NUMBER OF ATOMS
1
ITEM: BOX BOUNDS xy xz yz pp pp pp
0.0 5 0.0
0.0 5 0.0
0.0 5 0.0
ITEM: ATOMS id type xs ys zs ix iy iz
1 1 0.2 0.2 0.2 0 0 0
ITEM: TIMESTEP
1
ITEM: NUMBER OF ATOMS
2
ITEM: BOX BOUNDS xy xz yz pp pp pp
0.0 5 0.0
0.0 5 0.0
0.0 5 0.0
ITEM: ATOMS id type xs ys zs ix iy iz
1 1 0.2 0.2 0.2 0 0 0
2 1 0.4 0.4 0.4 0 0 0
";

        let file = File::open("test_save_traj_change_atoms.lmp.gz").unwrap();
        let mut file = flate2::read::GzDecoder::new(file);

        let mut file_text = String::new();
        file.read_to_string(&mut file_text).unwrap();

        assert_eq!(text, file_text);

        std::fs::remove_file("test_save_traj_change_atoms.lmp.gz").unwrap();
    }
}
