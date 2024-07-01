import sys
import os

def get_data_files(filename: str):
    slab_file = ""
    sol_file = ""
    with open(filename, 'r') as f:
        for line in f:
            if "variable slab string" in line:
                slab_file = "slabs/"+line.split()[3]
            elif "variable sol string" in line:
                sol_file = "solutions/"+line.split()[3]

    check_file(slab_file)
    check_file(sol_file)

    return slab_file, sol_file

def check_file(filename: str):
    if filename == "":
        print(f"ERROR could not read solution and/or slab filenames from file")
        sys.exit()
    if not os.path.isfile(filename):
        print(f"ERROR file does not exist: '{filename}'")
        sys.exit()

def read_data(filename: str):
    lowest = 1000000
    highest = 0

    with open(filename, 'r') as f:
        for line in f:
            if line.startswith("Atoms # full"):
                f.readline() # skip blank line afterwards
                break

        for line in f:
            if line.strip() == "": # reached end of Atoms section
                break

            z_pos = line.split()[6]
            try:
                z_pos = float(z_pos)
            except e:
                print(f"ERROR parsing atom position in: '{line}'")
                sys.exit()

            if z_pos < lowest:
                lowest = z_pos
            elif z_pos > highest:
                highest = z_pos

    if highest == 0:
        print(f"ERROR could not find Atoms section in file: '{filename}'")
        sys.exit()

    return lowest, highest


args = sys.argv
offset = 0
if len(args) == 1:
    print("USING DEFAULT OFFSET: 2")
    offset = 2
elif len(args) == 2:
    try:
        offset = float(args[1])
    except e:
        print(f"ERROR could not parse argument as float: '{args[1]}'")
else:
    print("Script needs 1 argument: [SOLUTION OFFSET, in angstrom]")
    sys.exit() 

#Extract names of slab and solution files from input.lmp
slab_file, sol_file = get_data_files("input.lmp")

#Calculate highest and lowest particles in slab
_, highest_slab = read_data(slab_file)
print(f"Highest particle z-position in slab is: {highest_slab}")

#Calculate highest and lowest particles in solution
lowest_sol, _ = read_data(sol_file)
print(f"Lowest particle z-position in solution is: {lowest_sol}")

#Calculate offset that must be added to LAMMPS input script
lmp_offset = highest_slab - lowest_sol + offset
print(f"Offset that must be applied is: {lmp_offset}")
