# create-slab

Build a crystal slab from scratch.

Scripts based on code from Dr. Stephen Yeandel.

Steps to create new slab:

1. First change the contents of the variable `file` in the `input.lmp` file in the `1-bulk-lattice-equi/` folder to the unit cell you want to use (the unit cells available are in the `unit-cells/` folder). Also change the `T` variable to the desired temperature in Kelvin. Send this job to run to the HPC. Once it finishes the program will print to screen the size of the equilibrated unit cell at the requested temperature.
2. Copy the equilibrated unit cell sizes to the variables in the `input.lmp` file in the `2-grow-slab/` folder and also set the `file` variable to the same value as in the previous step. Change the `repz` variable to set the desired height of the slab. This does not need to be run in the HPC and should take less than 5 seconds to finish.
3. Once the previous simulation is done, copy the new file `prod_data.lmp` to the directory `add-vacuum/` with the name `data.lmp`. Change the `box_z` variable value to the desired simulation box height. Also change the `repx` and `repy` variables to the replication factors in the x and y directions to obtain the desired slab size.
