# genetic-algorithm

Genetic algorithm to find the best fit of the nitrate-water interaction to the RDF of the paper ![https://doi.org/10.1021/acs.jpcb.7b06809](https://doi.org/10.1021/acs.jpcb.7b06809).

**IMPORTANT**, for this script to work you must first install and add to your PATH environment variable ![TRAVIS](http://www.travis-analyzer.de/).

The folders `paper-rdf-data/` and `travis-input-files/` contain data that the program uses so they should not be moved or modified. The folder `travis-rdf` is used by the script to create some intermediate files so this folder should be present and not modified by the user.

To run the program:

1. Run the nitrate in a box of water simulations and save the the uncompressed trajectory outputs in the `traj-outputs/` folder with the following naming scheme: `[N-Ow_A]_[N-Ow_rho]_[On-Ow_multiplier]_[On-Hw_multiplier].lmp`.
2. Once all the trajectory files are there, run the program `python ga_rdf.py [MUTATION_PERCENT]`. The `[MUTATION_PERCENT]` parameter changes the amount of variation of the new simulations. I used a value of 10.
3. Once the program runs it will output the top 3 performing simulations and also the new children simulations that must be run next. Create new simulations with the parameters indicated by this program. The output is formatted like the files in the `traj-outputs/` folder must be named.
