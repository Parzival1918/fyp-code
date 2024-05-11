mod analysis;
mod read_lammps;
mod structs;
mod write_lammps;

use std::collections::HashMap;
use std::fs::{DirEntry, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Error, Read, Write};
use std::path::Path;

use crate::read_lammps::traj;
use crate::structs::{Atom, System, TrajSnapshot};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    if args.len() == 1 {
        println!("Call the program with a subcommand");
        std::process::exit(1);
    } else if args[1] == "ion_conn" {
        ion_conn(&args);
    } else if args[1] == "harmonics" {
        harmonics(&args);
    } else if args[1] == "sph" {
        sph(&args);
    } else if args[1] == "sph_kno3" {
        sph_kno3(&args);
    } else if args[1] == "joincsv" {
        joincsv(&args);
    } else if args[1] == "surface_traj_track" {
        surface_traj_track(&args);
    } else {
        println!("Unknown subcommand");
        std::process::exit(1);
    }

    // let path = std::path::Path::new("test-data/prod_traj.lmp.gz");

    // println!("Opening gz file... ");
    // let file = File::open(path).unwrap();
    // let file = flate2::read::GzDecoder::new(file);
    // let reader = BufReader::new(file);
    // let mut line_it = reader.lines();
    // println!("done");

    // let mut positions: Vec<f64> = Vec::new();
    // println!("Calculating... ");
    // loop {
    //     let snapshot = match traj::next_step_content(&mut line_it) {
    //         Some(s) => s,
    //         None => break,
    //     };

    //     let mut highest_atom_z: f64 = 0.0;

    //     for atom in snapshot.system.atoms {
    //         if atom.position.z > highest_atom_z {
    //             highest_atom_z = atom.position.z;
    //         }
    //     }

    //     positions.push(highest_atom_z);
    // }
    // println!("done");

    // for item in positions.iter().enumerate() {
    //     println!("TIMESTEP: {}, highest atom pos: {}", item.0, item.1);
    // }
}

fn surface_traj_track(args: &[String]) {
    if args.len() != 6 {
        println!("Subcommand takes 4 arguments: [LOW] [HIGH] [SKIP] [FILENAME]");
        std::process::exit(1);
    }

    let low_bound: f64 = args[2].to_owned().parse().unwrap();
    let high_bound: f64 = args[3].to_owned().parse().unwrap();
    let skip: u32 = args[4].to_owned().parse().unwrap();
    let filename: String = args[5].to_owned().parse().unwrap();

    print!("Opening gz file... ");
    io::stdout().flush().unwrap();
    let file = File::open(filename).unwrap();
    let file = flate2::read::GzDecoder::new(file);
    let reader = BufReader::new(file);
    let mut line_it = reader.lines();
    println!("done");

    println!("Analysing trajectory file:");
    let mut traj_idx = 0u32;
    let mut position_track: HashMap<u32, (Vec<(u32, f64, f64)>, String)> = HashMap::new();
    loop {
        let trajectory = match traj::next_step_content(&mut line_it) {
            Some(s) => s,
            None => break,
        };
        traj_idx += 1;

        println!("Filtering step {}", traj_idx);
        let filtered_system = trajectory
            .system
            .filter_z(low_bound, high_bound)
            .filter_type(&[3, 4]);

        for atom in filtered_system.atoms {
            if let Some((vec, _)) = position_track.get_mut(&atom.id) {
                vec.push((traj_idx, atom.position.x, atom.position.y));
            } else {
                let atom_type: String = if atom.atom_type == 3 {
                    String::from("K")
                } else {
                    String::from("Cl")
                };
                position_track.insert(atom.id, (vec![(traj_idx, atom.position.x, atom.position.y)], atom_type));
            }
        }

        for _ in 1..skip {
            traj::next_step_content(&mut line_it);
            traj_idx += 1;
        }
    }

    print!("Saving data... ");
    for (id, (positions, atom_type)) in position_track {
        let filename = format!("surface-traj/{}_{}.csv", id, atom_type);
        let mut file = File::create(filename).unwrap();
        for (i, x, y) in positions {
            file.write_all(format!("{},{},{}\n", i, x, y).as_bytes())
                .unwrap();
        }
    }
    println!("done");
}

fn joincsv(_args: &[String]) {
    match std::fs::remove_file("joined.csv") {
        Ok(_) => println!("Previous 'joined.csv' deleted"),
        Err(_) => println!("No previous 'joined.csv' to delete"),
    };
    let mut out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("joined.csv")
        .unwrap();
    out_file.write_all("".as_bytes()).unwrap();

    let mut filenames = std::fs::read_dir(Path::new("."))
        .unwrap()
        .filter(|file| match file {
            Ok(d) => {
                if d.file_name().to_str().unwrap().ends_with(".csv") {
                    if d.file_name().to_str().unwrap().contains("split") {
                        return true;
                    }
                }
                return false;
            }
            Err(_) => return false,
        })
        .collect::<Vec<Result<DirEntry, Error>>>();

    filenames.sort_by(|a, b| {
        let n1: u32 = a
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .replace("split_", "")
            .replace(".csv", "")
            .parse()
            .unwrap();
        let n2: u32 = b
            .as_ref()
            .unwrap()
            .file_name()
            .to_str()
            .unwrap()
            .replace("split_", "")
            .replace(".csv", "")
            .parse()
            .unwrap();

        n1.partial_cmp(&n2).unwrap()
    });

    let mut count = 0u32;
    let mut tot_count = 0u32;
    for file in filenames {
        let file = file.unwrap();
        println!("Joining {}", file.file_name().to_str().unwrap());
        let file = file.path();
        let mut contents = String::new();
        OpenOptions::new()
            .read(true)
            .open(file)
            .unwrap()
            .read_to_string(&mut contents)
            .unwrap();

        for line in contents.lines() {
            let split: Vec<&str> = line.split(",").collect();
            out_file
                .write_all(
                    format!("{},{},{},{}\n", split[2], split[3], count, tot_count).as_bytes(),
                )
                .unwrap();
            tot_count += 1;
        }
        count += 1;
    }
}

fn sph_kno3(args: &[String]) {
    fn lucy(r: f64, h: f64) -> f64 {
        let rbar = r / h;
        if rbar >= 1.0 {
            return 0.0;
        }

        let prefactor = 105.0 / (16.0 * std::f64::consts::PI * h.powi(3));
        prefactor * (1.0 + 3.0 * rbar) * (1.0 - rbar).powi(3)
    }

    fn monaghan(r: f64, h: f64) -> f64 {
        let rbar = r / h;
        if rbar >= 1.0 {
            return 0.0;
        }

        let prefactor = 16.0 / (std::f64::consts::PI * h.powi(3));
        if rbar < 0.5 {
            return prefactor * (0.5 - 3.0 * rbar.powi(2) + 3.0 * rbar.powi(3));
        } else {
            return prefactor * (1.0 - rbar).powi(3);
        }
    }

    if args.len() != 6 {
        println!("Subcommand takes 4 arguments: [h] [LIMIT] [SKIP] [FILENAME]");
        std::process::exit(1);
    }

    let h: f64 = args[2].to_owned().parse().unwrap();
    let lim: f64 = args[3].to_owned().parse().unwrap();
    let skip_n: u32 = args[4].to_owned().parse().unwrap();
    let filename = &args[5];

    print!("Opening gz file... ");
    io::stdout().flush().unwrap();
    let file = File::open(filename).unwrap();
    let file = flate2::read::GzDecoder::new(file);
    let reader = BufReader::new(file);
    let mut line_it = reader.lines();
    println!("done");

    let mut traj_count = 0u32;
    let mut trajs: Vec<TrajSnapshot> = Vec::new();
    let mut extra_props: Vec<HashMap<u32, u32>> = Vec::new();
    match std::fs::remove_file("largest_cluster.csv") {
        Ok(_) => println!("Previous 'largest_cluster.csv' deleted"),
        Err(_) => println!("No previous 'largest_cluster.csv' to delete"),
    };
    let mut csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("largest_cluster.csv")
        .unwrap();
    csv_file.write_all("".as_bytes()).unwrap();
    loop {
        let trajectory = match traj::next_step_content(&mut line_it) {
            Some(s) => s,
            None => break,
        };

        let nns = analysis::find_nns(
            &trajectory
                .system
                .filter_z(0.0, 90.0)
                .filter_type(&[1, 2, 5]),
            h,
        );

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut atoms: Vec<Atom> = Vec::new();
        for nn in nns {
            if nn.central.atom_type == 1 {
                continue;
            }

            let mut density = 0.0;
            for neigh in nn.neighbours {
                let r = neigh.distance_to_atom(&nn.central, &trajectory.system.box_);
                let r = (r.0.powi(2) + r.1.powi(2) + r.2.powi(2)).sqrt();
                let val = monaghan(r, h);
                if neigh.atom_type == 1 {
                    density -= 8.0 * val;
                } else {
                    density += 20.0 * val;
                }
            }

            if density > max {
                max = density;
            }
            if density < min {
                min = density;
            }

            if density >= lim {
                atoms.push(nn.central);
            }
        }

        println!("MIN: {}, MAX: {}", min, max);

        // Assign cluster ids to each atom
        // An atom will be in a cluster if it is within some cutoff of another atom in that cluster
        let mut cluster_val = 1u32;
        let mut extra_prop: HashMap<u32, u32> = HashMap::new();
        let new_system = System::new(atoms, trajectory.system.box_);
        let nns_new = analysis::find_nns(&new_system, 3.4);
        for nn in nns_new {
            let mut neigh_clust: Vec<u32> = Vec::new();
            for neigh in &nn.neighbours {
                if let Some(clust) = extra_prop.get(&neigh.id) {
                    if !neigh_clust.contains(clust) {
                        neigh_clust.push(*clust);
                    }
                }
            }

            let centre_clust = *extra_prop.get(&nn.central.id).unwrap_or(&0);

            if centre_clust != 0 && neigh_clust.len() == 0 {
                for neigh in nn.neighbours {
                    extra_prop.insert(neigh.id, centre_clust);
                }
            } else if centre_clust == 0 && neigh_clust.len() != 0 {
                extra_prop.insert(nn.central.id, cluster_val);
                let mut keys_to_update: Vec<u32> = Vec::new();
                for (key, value) in extra_prop.iter() {
                    if neigh_clust.contains(value) {
                        keys_to_update.push(*key);
                    }
                }
                for key in keys_to_update {
                    extra_prop.insert(key, cluster_val);
                }
                cluster_val += 1;
            } else if centre_clust != 0 && neigh_clust.len() != 0 {
                extra_prop.insert(nn.central.id, centre_clust);
                let mut keys_to_update: Vec<u32> = Vec::new();
                for (key, value) in extra_prop.iter() {
                    if neigh_clust.contains(value) {
                        keys_to_update.push(*key);
                    }
                }
                for key in keys_to_update {
                    extra_prop.insert(key, centre_clust);
                }
            } else {
                extra_prop.insert(nn.central.id, cluster_val);
                for neigh in nn.neighbours {
                    extra_prop.insert(neigh.id, cluster_val);
                }
                cluster_val += 1;
            }
        }

        // Reassign cluster ids so they start from 1 and increment by 1
        let mut id_changes: Vec<(u32, u32)> = Vec::new(); // (atom_id, new_cluster_id)
        let mut new_ids: HashMap<u32, u32> = HashMap::new(); // old_id, new_id
        let mut ids: u32 = 1u32;
        for (key, value) in extra_prop.iter() {
            if new_ids.contains_key(value) {
                id_changes.push((*key, *new_ids.get(value).unwrap()));
            } else {
                new_ids.insert(*value, ids);
                id_changes.push((*key, ids));
                ids += 1;
            }
        }
        for (atom_id, new_clustr_id) in id_changes {
            extra_prop.insert(atom_id, new_clustr_id);
        }

        // Count the atoms per cluster and surface atoms
        let mut cluster_atoms: HashMap<u32, (u32, u32)> = HashMap::new(); // (cluster_id, (volume, surface))
        let nns = analysis::find_nns(
            &trajectory
                .system
                .filter_z(0.0, 90.0)
                .filter_type(&[1, 2, 5]),
            4.5,
        );
        for nn in nns {
            if nn.central.atom_type == 1 {
                continue; // skip water
            }

            let mut water_count = 0u32;
            for neigh in nn.neighbours {
                if neigh.atom_type == 1 {
                    water_count += 1;
                }
            }

            // println!("{}", nn.central.atom_type);
            let cluster_id = match extra_prop.get(&nn.central.id) {
                Some(c) => c,
                None => continue,
            };
            match cluster_atoms.contains_key(cluster_id) {
                true => {
                    let (vol_count, mut surface_count) = cluster_atoms.get(cluster_id).unwrap();
                    if water_count >= 1 {
                        surface_count += 1;
                    }
                    cluster_atoms.insert(*cluster_id, (vol_count + 1, surface_count));
                }
                false => {
                    let surface_count = if water_count >= 1 { 1 } else { 0 };
                    cluster_atoms.insert(*cluster_id, (1, surface_count));
                }
            }
        }

        let mut max = (0u32, 0u32, 0u32);
        for (cluster_id, (vol_count, surface_count)) in cluster_atoms {
            if vol_count > max.1 {
                max.0 = cluster_id;
                max.1 = vol_count;
                max.2 = surface_count;
            }
        }
        if let Err(e) = csv_file.write_all(
            format!(
                "{},{},{},{},{}\n",
                traj_count,
                max.0,
                max.1,
                max.2,
                max.2 as f64 / max.1 as f64
            )
            .as_bytes(),
        ) {
            println!("Error occurred writing to csv file: {}", e.to_string());
        };

        trajs.push(TrajSnapshot::new(new_system, traj_count));

        extra_props.push(extra_prop);

        for _ in 1..skip_n {
            traj::next_step_content(&mut line_it);
        }

        traj_count += 1;
    }

    write_lammps::traj::save_extra_prop("test.lmp.gz", trajs, extra_props);
}

fn sph(args: &[String]) {
    fn lucy(r: f64, h: f64) -> f64 {
        let rbar = r / h;
        if rbar >= 1.0 {
            return 0.0;
        }

        let prefactor = 105.0 / (16.0 * std::f64::consts::PI * h.powi(3));
        prefactor * (1.0 + 3.0 * rbar) * (1.0 - rbar).powi(3)
    }

    fn monaghan(r: f64, h: f64) -> f64 {
        let rbar = r / h;
        if rbar >= 1.0 {
            return 0.0;
        }

        let prefactor = 16.0 / (std::f64::consts::PI * h.powi(3));
        if rbar < 0.5 {
            return prefactor * (0.5 - 3.0 * rbar.powi(2) + 3.0 * rbar.powi(3));
        } else {
            return prefactor * (1.0 - rbar).powi(3);
        }
    }

    if args.len() != 6 {
        println!("Subcommand takes 4 arguments: [h] [LIMIT] [SKIP] [FILENAME]");
        std::process::exit(1);
    }

    let h: f64 = args[2].to_owned().parse().unwrap();
    let lim: f64 = args[3].to_owned().parse().unwrap();
    let skip_n: u32 = args[4].to_owned().parse().unwrap();
    let filename = &args[5];

    print!("Opening gz file... ");
    io::stdout().flush().unwrap();
    let file = File::open(filename).unwrap();
    let file = flate2::read::GzDecoder::new(file);
    let reader = BufReader::new(file);
    let mut line_it = reader.lines();
    println!("done");

    let mut traj_count = 0u32;
    let mut trajs: Vec<TrajSnapshot> = Vec::new();
    let mut extra_props: Vec<HashMap<u32, u32>> = Vec::new();
    match std::fs::remove_file("largest_cluster.csv") {
        Ok(_) => println!("Previous 'largest_cluster.csv' deleted"),
        Err(_) => println!("No previous 'largest_cluster.csv' to delete"),
    };
    let mut csv_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("largest_cluster.csv")
        .unwrap();
    csv_file.write_all("".as_bytes()).unwrap();
    loop {
        let trajectory = match traj::next_step_content(&mut line_it) {
            Some(s) => s,
            None => break,
        };

        let nns = analysis::find_nns(
            &trajectory
                .system
                .filter_z(0.0, 90.0)
                .filter_type(&[1, 3, 4]),
            h,
        );

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut atoms: Vec<Atom> = Vec::new();
        for nn in nns {
            if nn.central.atom_type == 1 {
                continue;
            }

            let mut density = 0.0;
            for neigh in nn.neighbours {
                let r = neigh.distance_to_atom(&nn.central, &trajectory.system.box_);
                let r = (r.0.powi(2) + r.1.powi(2) + r.2.powi(2)).sqrt();
                let val = monaghan(r, h);
                if neigh.atom_type == 1 {
                    density -= 4.0 * val;
                } else {
                    density += 20.0 * val;
                }
            }

            if density > max {
                max = density;
            }
            if density < min {
                min = density;
            }

            if density >= lim {
                atoms.push(nn.central);
            }
        }

        println!("MIN: {}, MAX: {}", min, max);

        // Assign cluster ids to each atom
        // An atom will be in a cluster if it is within some cutoff of another atom in that cluster
        let mut cluster_val = 1u32;
        let mut extra_prop: HashMap<u32, u32> = HashMap::new();
        let new_system = System::new(atoms, trajectory.system.box_);
        let nns_new = analysis::find_nns(&new_system, 3.4);
        for nn in nns_new {
            let mut neigh_clust: Vec<u32> = Vec::new();
            for neigh in &nn.neighbours {
                if let Some(clust) = extra_prop.get(&neigh.id) {
                    if !neigh_clust.contains(clust) {
                        neigh_clust.push(*clust);
                    }
                }
            }

            let centre_clust = *extra_prop.get(&nn.central.id).unwrap_or(&0);

            if centre_clust != 0 && neigh_clust.len() == 0 {
                for neigh in nn.neighbours {
                    extra_prop.insert(neigh.id, centre_clust);
                }
            } else if centre_clust == 0 && neigh_clust.len() != 0 {
                extra_prop.insert(nn.central.id, cluster_val);
                let mut keys_to_update: Vec<u32> = Vec::new();
                for (key, value) in extra_prop.iter() {
                    if neigh_clust.contains(value) {
                        keys_to_update.push(*key);
                    }
                }
                for key in keys_to_update {
                    extra_prop.insert(key, cluster_val);
                }
                cluster_val += 1;
            } else if centre_clust != 0 && neigh_clust.len() != 0 {
                extra_prop.insert(nn.central.id, centre_clust);
                let mut keys_to_update: Vec<u32> = Vec::new();
                for (key, value) in extra_prop.iter() {
                    if neigh_clust.contains(value) {
                        keys_to_update.push(*key);
                    }
                }
                for key in keys_to_update {
                    extra_prop.insert(key, centre_clust);
                }
            } else {
                extra_prop.insert(nn.central.id, cluster_val);
                for neigh in nn.neighbours {
                    extra_prop.insert(neigh.id, cluster_val);
                }
                cluster_val += 1;
            }
        }

        // Reassign cluster ids so they start from 1 and increment by 1
        let mut id_changes: Vec<(u32, u32)> = Vec::new(); // (atom_id, new_cluster_id)
        let mut new_ids: HashMap<u32, u32> = HashMap::new(); // old_id, new_id
        let mut ids: u32 = 1u32;
        for (key, value) in extra_prop.iter() {
            if new_ids.contains_key(value) {
                id_changes.push((*key, *new_ids.get(value).unwrap()));
            } else {
                new_ids.insert(*value, ids);
                id_changes.push((*key, ids));
                ids += 1;
            }
        }
        for (atom_id, new_clustr_id) in id_changes {
            extra_prop.insert(atom_id, new_clustr_id);
        }

        // Count the atoms per cluster and surface atoms
        let mut cluster_atoms: HashMap<u32, (u32, u32)> = HashMap::new(); // (cluster_id, (volume, surface))
        let nns = analysis::find_nns(
            &trajectory
                .system
                .filter_z(0.0, 90.0)
                .filter_type(&[1, 3, 4]),
            4.5,
        );
        for nn in nns {
            if nn.central.atom_type == 1 {
                continue; // skip water
            }

            let mut water_count = 0u32;
            for neigh in nn.neighbours {
                if neigh.atom_type == 1 {
                    water_count += 1;
                }
            }

            // println!("{}", nn.central.atom_type);
            let cluster_id = match extra_prop.get(&nn.central.id) {
                Some(c) => c,
                None => continue,
            };
            match cluster_atoms.contains_key(cluster_id) {
                true => {
                    let (vol_count, mut surface_count) = cluster_atoms.get(cluster_id).unwrap();
                    if water_count >= 1 {
                        surface_count += 1;
                    }
                    cluster_atoms.insert(*cluster_id, (vol_count + 1, surface_count));
                }
                false => {
                    let surface_count = if water_count >= 1 { 1 } else { 0 };
                    cluster_atoms.insert(*cluster_id, (1, surface_count));
                }
            }
        }

        let mut max = (0u32, 0u32, 0u32);
        for (cluster_id, (vol_count, surface_count)) in cluster_atoms {
            if vol_count > max.1 {
                max.0 = cluster_id;
                max.1 = vol_count;
                max.2 = surface_count;
            }
        }
        if let Err(e) = csv_file.write_all(
            format!(
                "{},{},{},{},{}\n",
                traj_count,
                max.0,
                max.1,
                max.2,
                max.2 as f64 / max.1 as f64
            )
            .as_bytes(),
        ) {
            println!("Error occurred writing to csv file: {}", e.to_string());
        };

        trajs.push(TrajSnapshot::new(new_system, traj_count));

        extra_props.push(extra_prop);

        for _ in 1..skip_n {
            traj::next_step_content(&mut line_it);
        }

        traj_count += 1;
    }

    write_lammps::traj::save_extra_prop("test.lmp.gz", trajs, extra_props);
}

fn harmonics(args: &[String]) {
    if args.len() != 6 {
        println!("Subcommand takes 4 arguments: [l] [LIMIT] [SKIP] [FILENAME]");
        std::process::exit(1);
    }

    let l: u32 = args[2].to_owned().parse().unwrap();
    let lim: f64 = args[3].to_owned().parse().unwrap();
    let skip_n: u32 = args[4].to_owned().parse().unwrap();
    let filename = &args[5];

    print!("Opening gz file... ");
    io::stdout().flush().unwrap();
    let file = File::open(filename).unwrap();
    let file = flate2::read::GzDecoder::new(file);
    let reader = BufReader::new(file);
    let mut line_it = reader.lines();
    println!("done");

    let mut traj_count = 0u32;
    let mut trajs: Vec<TrajSnapshot> = Vec::new();
    loop {
        let trajectory = match traj::next_step_content(&mut line_it) {
            Some(s) => s,
            None => break,
        };

        let nns = analysis::find_nns(
            &trajectory.system.filter_z(0.0, 90.0).filter_type(&[3, 4]),
            5.0,
        );

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut atoms: Vec<Atom> = Vec::new();
        for nn in nns {
            let q_l = analysis::q_l(l as i32, &nn);

            if q_l > max {
                max = q_l;
            }
            if q_l < min {
                min = q_l;
            }

            if q_l <= lim {
                atoms.push(nn.central);
            }
        }

        println!("MIN: {}, MAX: {}", min, max);

        trajs.push(TrajSnapshot::new(
            System::new(atoms, trajectory.system.box_),
            traj_count,
        ));

        for _ in 0..skip_n {
            traj::next_step_content(&mut line_it);
        }

        traj_count += 1;
    }

    write_lammps::traj::save("test.lmp.gz", trajs);
}

fn ion_conn(args: &[String]) {
    if args.len() != 4 {
        println!("Subcommand takes 2 arguments: [SKIP] [FILENAME]");
        std::process::exit(1);
    }

    let skip_n: u32 = args[2].to_owned().parse().unwrap();
    let filename = &args[3];

    print!("Opening gz file... ");
    io::stdout().flush().unwrap();
    let file = File::open(filename).unwrap();
    let file = flate2::read::GzDecoder::new(file);
    let reader = BufReader::new(file);
    let mut line_it = reader.lines();
    println!("done");

    let mut traj_count = 0u32;
    let mut trajs: Vec<TrajSnapshot> = Vec::new();
    loop {
        let trajectory = match traj::next_step_content(&mut line_it) {
            Some(s) => s,
            None => break,
        };

        let nns = analysis::find_nns(&trajectory.system.filter_z(0.0, 90.0), 4.0);

        let mut full = 0u32;
        let mut semi = 0u32;
        let mut atoms: Vec<Atom> = Vec::new();
        for nn in nns {
            if nn.central.atom_type == 3 {
                let mut count = 0u32;
                let mut water = 0u32;
                for other in nn.neighbours {
                    if other.atom_type == 4 {
                        count += 1;
                    } else if other.atom_type == 1 {
                        water += 1;
                    }
                }

                if count == 6 {
                    full += 1;
                    atoms.push(nn.central);
                } else if water < 4 {
                    semi += 1;
                    atoms.push(Atom {
                        atom_type: 2,
                        ..nn.central
                    })
                }
            } else if nn.central.atom_type == 4 {
                let mut count = 0u32;
                let mut water = 0u32;
                for other in nn.neighbours {
                    if other.atom_type == 3 {
                        count += 1;
                    } else if other.atom_type == 1 {
                        water += 1;
                    }
                }

                if count == 6 {
                    full += 1;
                    atoms.push(nn.central);
                } else if water < 4 {
                    semi += 1;
                    atoms.push(Atom {
                        atom_type: 1,
                        ..nn.central
                    })
                }
            }
        }

        println!("Step {}: full {}, semi {}", trajectory.step, full, semi);

        trajs.push(TrajSnapshot::new(
            System::new(atoms, trajectory.system.box_),
            traj_count,
        ));

        for _ in 0..skip_n {
            traj::next_step_content(&mut line_it);
        }

        traj_count += 1000;
    }

    write_lammps::traj::save("test.lmp.gz", trajs);
}
