pub struct NNs {
    pub central: Atom,
    pub neighbours: Vec<Atom>,
}

impl NNs {
    pub fn new(central: Atom, neighbours: Vec<Atom>) -> Self {
        NNs {
            central,
            neighbours,
        }
    }
}

#[derive(Clone)]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Position {
    pub fn new(x: f64, y: f64, z: f64) -> Position {
        Position { x, y, z }
    }
}

pub struct Property<T>
where
    T: ToString,
{
    name: String,
    value: T,
}

impl<T> Property<T> where T: ToString {}

#[derive(Clone)]
pub struct Atom {
    pub id: u32,
    pub molecule_id: Option<u32>,
    pub atom_type: u32,
    pub position: Position,
    //pub extra_properties: Vec<Property<T>>,
}

impl Atom {
    pub fn new(id: u32, molecule_id: Option<u32>, atom_type: u32, position: Position) -> Atom {
        Atom {
            id,
            molecule_id,
            atom_type,
            position,
        }
    }

    /// Calculate distance between self and other taking into account box periodicity
    pub fn distance_to_atom(&self, other: &Atom, box_: &Box) -> (f64, f64, f64) {
        let half_x = box_.lx / 2.0;
        let half_y = box_.ly / 2.0;
        let half_z = box_.lz / 2.0;

        let mut dx = (self.position.x - other.position.x).abs();
        if dx > half_x {
            dx = box_.lx - dx;
        }

        let mut dy = (self.position.y - other.position.y).abs();
        if dy > half_y {
            dy = box_.ly - dy;
        }

        let mut dz = (self.position.z - other.position.z).abs();
        if dz > half_z {
            dz = box_.lz - dz;
        }

        (dx.abs(), dy.abs(), dz.abs())
    }
}

#[derive(Clone, Copy)]
pub struct Box {
    pub lx: f64,
    pub ly: f64,
    pub lz: f64,
}

impl Box {
    pub fn new(lx: f64, ly: f64, lz: f64) -> Box {
        Box { lx, ly, lz }
    }

    pub fn vol(&self) -> f64 {
        self.lx * self.ly * self.lz
    }
}

pub struct System {
    pub atoms: Vec<Atom>,
    pub box_: Box,
}

impl System {
    pub fn new(atoms: Vec<Atom>, box_: Box) -> System {
        System { atoms, box_ }
    }

    pub fn filter_z(&self, zlo: f64, zhi: f64) -> System {
        let mut new_atoms: Vec<Atom> = Vec::new();
        for atom in self.atoms.iter() {
            if atom.position.z >= zlo && atom.position.z <= zhi {
                new_atoms.push(atom.clone());
            }
        }
        System::new(new_atoms, self.box_.clone())
    }

    pub fn filter_type(&self, atom_type: &[u32]) -> System {
        let mut new_atoms: Vec<Atom> = Vec::new();
        for atom in self.atoms.iter() {
            if atom_type.contains(&atom.atom_type) {
                new_atoms.push(atom.clone());
            }
        }
        System::new(new_atoms, self.box_.clone())
    }
}

pub struct TrajSnapshot {
    pub system: System,
    pub step: u32,
}

impl TrajSnapshot {
    pub fn new(system: System, step: u32) -> TrajSnapshot {
        TrajSnapshot { system, step }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_no_pbcs() {
        let box_ = Box::new(100.0, 100.0, 100.0);
        let atom1 = Atom::new(1, Some(1), 1, Position::new(0.0, 0.0, 0.0));
        let atom2 = Atom::new(2, Some(2), 1, Position::new(10.0, 10.0, 10.0));

        assert_eq!(atom1.distance_to_atom(&atom2, &box_), (10.0, 10.0, 10.0))
    }

    #[test]
    fn distance_pbcs() {
        let box_ = Box::new(10.0, 10.0, 10.0);
        let atom1 = Atom::new(1, Some(1), 1, Position::new(0.0, 0.0, 0.0));
        let atom2 = Atom::new(2, Some(2), 1, Position::new(9.0, 9.0, 9.0));

        assert_eq!(atom1.distance_to_atom(&atom2, &box_), (1.0, 1.0, 1.0))
    }

    #[test]
    fn distance_mix_pbcs() {
        let box_ = Box::new(10.0, 10.0, 10.0);
        let atom1 = Atom::new(1, Some(1), 1, Position::new(0.0, 0.0, 0.0));
        let atom2 = Atom::new(2, Some(2), 1, Position::new(7.0, 2.0, 9.0));

        assert_eq!(atom1.distance_to_atom(&atom2, &box_), (3.0, 2.0, 1.0))
    }
}
