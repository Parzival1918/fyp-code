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

def write_to_file_data(filename: str, offset: float, highest_slab: float):
    lines = []
    with open(filename, 'r') as f:
        for line in f:
            if "variable sol_shift equal" in line:
                line = f"variable sol_shift equal    {offset} # z shift when appending solution (angstroms)\n"
            if "variable freeze equal" in line:
                line = f"variable freeze equal    {highest_slab} # freeze height, set to height of slab (angstroms)\n"
            lines.append(line)

    with open(filename, 'w') as f:
        for line in lines:
            f.write(line)

args = sys.argv
offset = 0
write_to_file = False
if len(args) == 1:
    print("USING DEFAULT OFFSET: 2")
    offset = 2
elif len(args) == 2:
    try:
        offset = float(args[1])
    except e:
        print(f"ERROR could not parse argument as float: '{args[1]}'")
elif len(args) == 3:
    try:
        offset = float(args[1])
    except e:
        print(f"ERROR could not parse argument as float: '{args[1]}'")

    if args[2] == "true":
        print("SCRIPT WILL WRITE TO 'input.lmp' FILE")
        write_to_file = True
else:
    print("Script needs 1 argument: [SOLUTION OFFSET, in angstrom]")
    print("There is an optional extra argument to force the script to write the result to the input LAMMPS file: false/true (default false)")
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

#Write to input file
if write_to_file:
    print("WRITING TO FILE... ", end="")
    write_to_file_data("input.lmp", lmp_offset, highest_slab)
    print("DONE")
