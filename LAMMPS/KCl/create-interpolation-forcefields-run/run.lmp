#!/bin/bash

# Job options
#SBATCH --job-name=system
#SBATCH --nodes=2
#SBATCH --ntasks-per-node=64
#SBATCH --ntasks-per-socket=32
#SBATCH --ntasks-per-core=1
#SBATCH --time=30:00:00
#SBATCH --mail-type=ALL
#SBATCH --mail-user=pjuanroyo1@sheffield.ac.uk

# Load modules
module purge
module load intel/2022b

# Set openmp (set same as cpus-per task)
export OMP_NUM_THREADS=1

# srun
srun --export=ALL --unbuffered --distribution=block:block --hint=nomultithread --exact \
/users/mta20pj/bin/lmp_stanage_03Mar2020_PLUMED -in input.lmp > screen.lmp

