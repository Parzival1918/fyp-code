use crate::structs::*;
use num_complex::{Complex64, ComplexFloat};
use scilib::{coordinate, quantum};

fn magnitude(x: f64, y: f64, z: f64) -> f64 {
    (x.powi(2) + y.powi(2) + z.powi(2)).sqrt()
}

pub fn find_nns(system: &System, cutoff: f64) -> Vec<NNs> {
    let mut neigh_list: Vec<NNs> = Vec::new();
    let atoms = &system.atoms;
    let box_ = &system.box_;

    for center in atoms {
        let mut new_nns: Vec<Atom> = Vec::new();
        for other in atoms {
            if center.id != other.id {
                let (x, y, z) = center.distance_to_atom(other, box_);
                let mag = magnitude(x, y, z);

                if mag <= cutoff {
                    new_nns.push(other.clone());
                }
            }
        }

        neigh_list.push(NNs::new(center.clone(), new_nns));
    }

    neigh_list
}

fn q_lm(l: i32, m: i32, theta: &Vec<f64>, phi: &Vec<f64>) -> Complex64 {
    let n = theta.len();
    let mut sum: Complex64 = Complex64::new(0.0, 0.0);
    for i in 0..theta.len() {
        // println!("{} {} {} {}", l, m, theta[i], phi[i]);
        sum += quantum::spherical_harmonics(l as usize, m as isize, theta[i], phi[i]);
    }

    sum / Complex64::new(n as f64, 0.0)
}

pub fn q_l(l: i32, nns: &NNs) -> f64 {
    // calculate polar and azimuthal angles of all neighbours
    let mut theta: Vec<f64> = Vec::new();
    let mut phi: Vec<f64> = Vec::new();

    for atom in &nns.neighbours {
        let (x, y, z) = (
            atom.position.x - nns.central.position.x,
            atom.position.y - nns.central.position.y,
            atom.position.z - nns.central.position.z,
        );
        let cartesian = coordinate::cartesian::Cartesian::from(x, y, z);
        let sph_coords = coordinate::spherical::Spherical::from_coord(cartesian);
        theta.push(sph_coords.phi);
        phi.push(sph_coords.theta);
    }

    let mut sumq_lm = 0.0;
    for m in -l..l + 1 {
        sumq_lm += q_lm(l, m, &theta, &phi).abs().powi(2);
    }

    let fac = 4.0 * std::f64::consts::PI / (2.0 * (l as f64) + 1.0);

    (fac * sumq_lm).sqrt()
}

// pub fn rdf(system: &System, cutoff: f64, bins: u32) -> Vec<f64> {
//     // vec to store rdf of each center particle to detect particles
//     let mut rdf: Vec<f64> = vec![0.0; bins as usize];
//     let bin_width = cutoff / bins as f64;
//     let mut npart = system.atoms.len();
//     let mut num_density = 0.0;
//     let sys_vol = system.box_.vol();

//     for (i, atom) in system.atoms[..npart - 1].iter().enumerate() {
//         for other in system.atoms[i + 1..].iter() {
//             let other_center_idx = center_particles.iter().position(|&x| x == other.atom_type);
//             let other_detect_idx = detect_particles.iter().position(|&x| x == other.atom_type);
//             if other_center_idx == None && other_detect_idx == None {
//                 continue;
//             }

//             let (x, y, z) = atom.distance_to_atom(other, &system.box_);
//             let mag = magnitude(x, y, z);
//             if mag <= cutoff {
//                 let bin = (mag / bin_width) as usize;
//                 if atom_center_idx != None && other_detect_idx != None {
//                     rdf[atom_center_idx.unwrap()][other_detect_idx.unwrap()][bin] += 1.0;
//                 }
//                 if atom_detect_idx != None && other_center_idx != None {
//                     rdf[other_center_idx.unwrap()][atom_detect_idx.unwrap()][bin] += 1.0;
//                 }
//             }
//         }
//     }
//     for i in 0..detect_particles.len() {
//         num_density[i] = npart[i] as f64 / sys_vol;
//     }

//     // normalize rdf
//     let gfac = (4.0 / 3.0) * std::f64::consts::PI * bin_width.powi(3);
//     for i in 0..center_particles.len() {
//         for j in 0..detect_particles.len() {
//             for k in 0..bins as usize {
//                 let vb = gfac * (((k + 1) as f64).powi(3) - ((k) as f64).powi(3));
//                 let nid = vb * num_density[j];
//                 rdf[i][j][k] /= npart[j] as f64 * nid;
//             }
//         }
//     }

//     rdf
// }
