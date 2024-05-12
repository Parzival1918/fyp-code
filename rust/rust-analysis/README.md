# rust-analysis

There are two ways to run the script, using `cargo` or building the binary and running the binary file directly:

1. Using `cargo`
  - Run the following command within the directory of the script `cargo run --release -- [COMMANDS]`. Anything written after `--` will be passed directly to the `rust-analysis` script as arguments. A description of all available subcommands and their arguments is provided below.
2. Building the binary
  - The fastest way to build the binary is by using the command `cargo build --release`. This will compile the script into a binary in the folder `target/release/` with the name `rust-analysis`. Run the binary `./rust-analysis [COMMANDS]`. A description of the available subcommands and their arguments is provided below.

Available subcommands:

- `sph`: This subcommand calculates the solid atoms of a KCl simulation using the SPH density formula.
  - Input arguments: `[h] [LIMIT] [SKIP] [FILENAME]`.
    - `[h]`: The maximum radius to use atoms for the density calculation. The value I used is 6.
    - `[LIMIT]`: The minimum density value to count an atom as solid. The value I used is 0.12.
    - `[SKIP]`: Number of trajectory snapshots that will be skipped. If you want to analysise the whole trajectory file use 0.
    - `[FILENAME]`: The path to the file and filename of the LAMMPS trajectory output. The code expects this file to be compressed in the .gz format.
  - Outputs:
    - `largset_cluster.csv`: This file contains 5 columns and each row is a different snapshot of the trajectory file, containing data of the largest cluster in the simulation which will always be the crystal slab in our simulations. The first row value goes from 0 to the number of snapshots analysed. The second row is the id of the cluster. The third row is the number of bulk atoms in the cluster. The fourth row is the number of surface atoms. The fifth row is the ratio of surface over bulk atoms.
    - `test.lmp.gz`: This is a file formatted as a LAMMPS trajectory output with an extra property that adds the cluster id of each atom. Using OVITO this file can be visualised and filter the atoms by cluster id.
- `sph_kno3`: This subcommand calculates the solid atoms of a KNO3 simulation using the SPH density algorithm.
  - Input arguments: `[h] [LIMIT] [SKIP] [FILENAME]`.
    - `[h]`: The maximum radius to use atoms for the density calculation. The value I used is 6.
    - `[LIMIT]`: The minimum density value to count an atom as solid. The value I used is 0.12.
    - `[SKIP]`: Number of trajectory snapshots that will be skipped. If you want to analysise the whole trajectory file use 0.
    - `[FILENAME]`: The path to the file and filename of the LAMMPS trajectory output. The code expects this file to be compressed in the .gz format.
  - Outputs:
    - `largset_cluster.csv`: This file contains 5 columns and each row is a different snapshot of the trajectory file, containing data of the largest cluster in the simulation which will always be the crystal slab in our simulations. The first row value goes from 0 to the number of snapshots analysed. The second row is the id of the cluster. The third row is the number of bulk atoms in the cluster. The fourth row is the number of surface atoms. The fifth row is the ratio of surface over bulk atoms.
    - `test.lmp.gz`: This is a file formatted as a LAMMPS trajectory output with an extra property that adds the cluster id of each atom. Using OVITO this file can be visualised and filter the atoms by cluster id.
- `joincsv`: This subcommand joins multiple `largest_cluster.csv` output files from running the `sph` or `sph_kno3` subcommands into a single file to make area vs bulk atoms plots.
  - Input arguments: None.
    - There are no input arguments but the program will fail to run unless the `largest_cluster.csv` files are renamed to `split_*.csv`, where `*` is the order of the cluster files starting from 1.
  - Outputs:
    - `joined.csv`: This file has 4 columns. The first one is the bulk atoms. The second one is the surface atoms. The third one is the file number from which that row of data comes. The fourth one starts at 0 and increments by one for each row.
- `surface_traj_track`: This subcommand tracks the positions of K and Cl ions within a range on the z-position. Used to make the surface trajectory plots of the final report. IMPORTANT, for this subcommand to work a directory with the name `surface-traj` must be created before running the script.
  - Input arguments: `[LOW] [HIGH] [SKIP] [FILENAME]`.
    - `[LOW]`: The lower bound of the z-position to track.
    - `[HIGH]`: The upper bound of the z-position to track.
    - `[SKIP]`: Number of trajectory snapshots that will be skipped. If you want to analysise the whole trajectory file use 0.
    - `[FILENAME]`: The path to the file and filename of the LAMMPS trajectory output. The code expects this file to be compressed in the .gz format.
  - Outputs:
    - The `surface-traj` directory will be filled with csv files containing the x and y positions of the atoms within the set z range. The files have 3 columns. The first column contains the timestep value of the coordinates. The second column is the x position. The third column is the y position.
