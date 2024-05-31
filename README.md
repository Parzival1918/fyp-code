# fyp-code

by **Pedro Juan Royo**

This is the GitHub repository containing the code written by me for the final year project *"Molecular dynamics study of growth and dissolution mechanisms in the KNO3 and KCl systems"*.

## python

> All the python scripts I wrote were ran using **python v3.11**. Although they should be able to run with any version above and including **v3.9**.

For the python scripts I would recommend creating virtual environments for each script to avoid conflicts between different packages and versions. To create the virtual environment run:

```shell
# Create a virtual environment in the folder .venv/
# If python complains there is no module named venv, install it using 'pip install virtualenv' and try again
> python3 -m venv .venv

# Activate the virtual environment in bash and zsh
> source .venv/bin/activate
# Activate the virtual environment in fish shell
> source .venv/bin/activate.fish

# Once the environment is active install the dependencies by running
> pip install -r requirements.txt

# To deactivate the virtual environment
> deactivate
```

The python scripts available:

- `genetic-algorithm`: Genetic algorithm script to optimise the nitrate-water interaction.
- `a-b-plot`: Generate surface-bulk atoms plot from a `joined.csv` file generates with the `joincsv` subcommand from the `rust-analysis` rust script.

## rust

To run the rust scripts first install the `rustup` tool:

```shell
> curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

This will install the tools `rustup`, `rustc` and `cargo` (but we will only need to use `cargo`).

The rust scripts available:

- `rust-analysis`: This script contains multiple commands to carry out analysis to the LAMMPS output trajectory files

## bash

Bash scripts that I used in the Stanage HPC to run the simulations and check their progress.

## LAMMPS

Scripts used to create the simulations to run with LAMMPS.

The available scripts are:

- `KCl`
  - `create-slab`: Create a crystal slab.
  - `create-solution`: Create a solution with different numbr of ions.
  - `create-CmuMD-sim`: Create all the files to run a CmuMD simulation. You will first have to create slabs and solutions.
  - `create-interpolation-forcefields-run`: This is an example of the input files required to run a KCl CmuMD simulation that slowly shifts the forcefield from the modified to the original one. It is not setup so that it can easily be modified, I will work on this soon.
- `KNO3`
  - `create-slab`: Create a crystal slab.
  - `create-solution`: Create a solution with different numbr of ions.
  - `create-CmuMD-sim`: Create all the files to run a CmuMD simulation. You will first have to create slabs and solutions.

