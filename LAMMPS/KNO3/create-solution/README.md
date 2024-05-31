# create-solution

Build KCl solution from scratch.

Scripts based on code from Dr. Stephen Yeandel.

- Can run steps 1 to 3 (and possibly 4) using an interactive session in Stanage
- Rest of steps maybe run using the squeue command as they can take a few more resources and time.

Steps to create a new solution:

1. In the `1-pack-K/` folder change the `box_x` and `box_y` values of the variables in the file `input.lmp` to the desired dimensions (they must match the dimensions of the crystal slab, so it is a good idea to create a crystal slab before making any solutions). Also change the `Nmols` variable to the number of potassium ions you want. Run the simulation with LAMMPS, it should take less than 5 seconds.
2. Once the simulation has run copy the newly created file `deposit_data.lmp` to the directory `2-pack-NO3` with the name `data.lmp`. Also change the `Nmols` variable to the number of nitrate ions you want (should match the number of potassium ions). Run the simulation with LAMMPS, it should take less than 5 seconds.
3. Once the simulation has run copy the newly creates file `deposit_data.lmp` to the directory `3-pack-H2O` with the name `data.lmp`. Also change the `Nmols` variable to the number of water molecules you want. Run the simulation with LAMMPS, it should take less than a minute.
4. Once the simulation has run copy the newly creates file `deposit_data.lmp` to the directory `4-relax` with the name `data.lmp`. Run the simulation using LAMMPS, depending on the production time chosen you may want to send the job to run to the HPC.
5. Once the simulation has run copy the newly creates file `prod_data.lmp` to the directory `5-combine` with the name `data.lmp`. Set the `sol_offset` so that the replicated solutions added on top of the original one do not have any overlapping atoms. Send this simulation to run to the HPC. The soution will be the file created after the simulation runs with the name `prod_data.lmp`. Before using this solution to make the CmuMD simulations check that the replicated solutions have joined into a single one without any voids inside.
