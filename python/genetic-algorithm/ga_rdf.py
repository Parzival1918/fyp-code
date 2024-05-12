import os
import sys
import numpy as np

# Genetic algorithm for RDF
# The algorithm inputs are (total 4):
# - A, rho from N-Ow buck potential
# - mult from On-Ow buck potential
# - mult from On-Hw buck potential

# Must find best fit to RDF from paper https://doi.org/10.1021/acs.jpcb.7b06809

# Read from the command line the percentage of mutations
# that will be applied to the new trajectories
# The percentage is a float between 0 and 100
# Check that there is only one argument
if len(sys.argv) != 2:
    print("Error: Must provide one argument")
    sys.exit()
# Check that the argument is a float
try:
    mut_perc = float(sys.argv[1])
    mut = mut_perc / 100
except ValueError:
    print("Error: Argument must be a float")
    sys.exit()

# Get names of all the files in the folder traj-outputs/
traj_files = os.listdir('traj-outputs/')
# remove hidden files
traj_files = [x for x in traj_files if not x.startswith('.')]

# from the filenames extract the values of A, rho, mult_OnOw, mult_OnHw
# filename is in format A_rho_multOnOw_multOnHw.lmp
run_params = {}
for traj_file in traj_files:
    traj_file_no_extension = traj_file[:-4] # remove the .lmp extension
    traj_file_no_extension_split = traj_file_no_extension.split('_')
    run_params[traj_file] = {}
    run_params[traj_file]['A'] = float(traj_file_no_extension_split[0])
    run_params[traj_file]['rho'] = float(traj_file_no_extension_split[1])
    run_params[traj_file]['mult_OnOw'] = float(traj_file_no_extension_split[2])
    run_params[traj_file]['mult_OnHw'] = float(traj_file_no_extension_split[3])

# Run travis to get the RDFs of the trajectories in traj-outputs/ folder
os.chdir('travis-rdf/')
os.system('rm -r *') # remove all the files in travis-rdf/ folder from previous runs
for traj_file in traj_files:
    os.mkdir(traj_file)
    os.chdir(traj_file)
    os.system('travis -p ../../traj-outputs/' + 
                traj_file + ' -i ../../travis-input-files/input_rdf_N_Hw.txt')
    os.system('travis -p ../../traj-outputs/' + 
                traj_file + ' -i ../../travis-input-files/input_rdf_N_Ow.txt')
    # remove the .agr and .log files
    os.system('rm *.agr')
    os.system('rm *.log')
    os.chdir('..')
os.chdir('..')

# print the run_params dictionary
print("RUN PARAMS")
print(run_params)

# Calculate the fitness of each trajectory, by comparing the RDFs to the RDFs from the paper
# Fitness is the sum of the absolute differences between the RDFs
# The lower the fitness, the better the fit
fitness_vals = {}
paper_rdf_N_Ow = {"r": [], "g(r)": []}
paper_rdf_N_Hw = {"r": [], "g(r)": []}
with open('paper-rdf-data/NOw.csv', 'r') as f:
    for line in f:
        paper_rdf_N_Ow["r"].append(float(line.split(',')[0])*100) # convert from A to pm
        paper_rdf_N_Ow["g(r)"].append(float(line.split(',')[1]))
with open('paper-rdf-data/NHw.csv', 'r') as f:
    for line in f:
        paper_rdf_N_Hw["r"].append(float(line.split(',')[0])*100) # convert from A to pm
        paper_rdf_N_Hw["g(r)"].append(float(line.split(',')[1]))

# Function that calculates the g(r) difference between two RDFs, but
# takes into account that the paper RDFs have a different number of points
# so it interpolates the paper RDFs to the same number of points as the RDFs
# from the trajectories
def calc_rdf_diff(traj_g_r: float, traj_r: float, second_atom: str):
    # Interpolate the paper RDFs to the same number of points as the trajectory RDFs
    if second_atom == "Ow":
        paper_rdf_N_Ow_interp = np.interp(traj_r, paper_rdf_N_Ow["r"], paper_rdf_N_Ow["g(r)"])
        rdf_diff_N_Ow = traj_g_r - paper_rdf_N_Ow_interp
        rdf_diff_N_Ow_abs = np.absolute(rdf_diff_N_Ow)
        return rdf_diff_N_Ow_abs
    elif second_atom == "Hw":
        paper_rdf_N_Hw_interp = np.interp(traj_r, paper_rdf_N_Hw["r"], paper_rdf_N_Hw["g(r)"])
        rdf_diff_N_Hw = traj_g_r - paper_rdf_N_Hw_interp
        rdf_diff_N_Hw_abs = np.absolute(rdf_diff_N_Hw)
        return rdf_diff_N_Hw_abs

for traj_file in traj_files:
    fitness_vals[traj_file] = 0
    total_sum = 0

    # Read the RDFs from the trajectory
    traj_rdf_N_Ow = {"r": [], "g(r)": []}
    traj_rdf_N_Hw = {"r": [], "g(r)": []}
    with open('travis-rdf/' + traj_file + '/rdf_NO3_H2O_[Nr_Oo].csv', 'r') as f:
        # skip the first line
        next(f)
        for line in f:
            traj_rdf_N_Ow["r"].append(float(line.split('; ')[0]))
            traj_rdf_N_Ow["g(r)"].append(float(line.split('; ')[1]))
    with open('travis-rdf/' + traj_file + '/rdf_NO3_H2O_[Nr_Ho].csv', 'r') as f:
        # skip the first line
        next(f)
        for line in f:
            traj_rdf_N_Hw["r"].append(float(line.split('; ')[0]))
            traj_rdf_N_Hw["g(r)"].append(float(line.split('; ')[1]))

    # Calculate the fitness of the trajectory
    for i in range(len(traj_rdf_N_Ow["r"])):
        total_sum += calc_rdf_diff(traj_rdf_N_Ow["g(r)"][i], traj_rdf_N_Ow["r"][i], "Ow")
    for i in range(len(traj_rdf_N_Hw["r"])):
        total_sum += calc_rdf_diff(traj_rdf_N_Hw["g(r)"][i], traj_rdf_N_Hw["r"][i], "Hw")

    fitness_vals[traj_file] = total_sum

# Order the trajectories by fitness value
# The lower the fitness value, the better the fit
fitness_vals_sorted = sorted(fitness_vals.items(), key=lambda x: x[1])
# print the fitness values
print("FITNESS VALUES")
for traj_file in fitness_vals_sorted:
    print(traj_file)

# Select the top 3 and breed them to get 6 new trajectories
# The breeding is done by averaging the values of the parameters
# of the two parents
top_3 = fitness_vals_sorted[:3]
top_3_names = [x[0] for x in top_3]
top_3_params = [run_params[x] for x in top_3_names]
print("TOP 3")
print(top_3_params)

# Breed the top 3 to get 6 new trajectories
new_params = []
for i in range(3):
    for j in range(i+1, 3):
        new_params.append({})

        # Choose the new parameters by randomly choosing one of the parents
        # for each parameter
        new_params[-1]['A'] = np.random.choice([top_3_params[i]['A'], top_3_params[j]['A']])
        new_params[-1]['rho'] = np.random.choice([top_3_params[i]['rho'], top_3_params[j]['rho']])
        new_params[-1]['mult_OnOw'] = np.random.choice([top_3_params[i]['mult_OnOw'], top_3_params[j]['mult_OnOw']])
        new_params[-1]['mult_OnHw'] = np.random.choice([top_3_params[i]['mult_OnHw'], top_3_params[j]['mult_OnHw']])

        new_params.append({})

        # Choose the new parameters by randomly choosing one of the parents
        # for each parameter
        new_params[-1]['A'] = np.random.choice([top_3_params[i]['A'], top_3_params[j]['A']])
        new_params[-1]['rho'] = np.random.choice([top_3_params[i]['rho'], top_3_params[j]['rho']])
        new_params[-1]['mult_OnOw'] = np.random.choice([top_3_params[i]['mult_OnOw'], top_3_params[j]['mult_OnOw']])
        new_params[-1]['mult_OnHw'] = np.random.choice([top_3_params[i]['mult_OnHw'], top_3_params[j]['mult_OnHw']])

# Add a random mutation to two of the new parameters of each new trajectory
# Mutation is byt multiplying the parameter by a random number between 0.9 and 1.1
for i in range(len(new_params)):
    # Choose two parameters to mutate
    params_to_mutate = np.random.choice(['A', 'rho', 'mult_OnOw', 'mult_OnHw'], 2, replace=False)
    # Mutate the parameters
    for param in params_to_mutate:
        new_params[i][param] *= np.random.uniform(1 - mut, 
                                                  1 + mut) #np.random.uniform(0.9, 1.1)

print("Mutation percentage: " + str(mut_perc))
print("Mutation: " + str(mut))
# print the new parameters and parents
print("PARENTS")
for parent in top_3_params:
    print(parent)
print("CHILDREN")
for child in new_params:
    print(child)
    print(" - " + str(child['A']) + "_" + str(child['rho']) +
          "_" + str(child['mult_OnOw']) + "_" + str(child['mult_OnHw']) + ".lmp")