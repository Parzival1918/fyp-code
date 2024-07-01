import sys
import os

def get_data_files(filename: str):
    res_file = ""
    with open(filename, 'r') as f:
        for line in f:
            if "variable res string" in line:
                res_file = "solutions/"+line.split()[3]

    check_file(res_file)
    check_file("data.lmp")

    return res_file

def check_file(filename: str):
    if filename == "":
        print(f"ERROR could not read reservoir filename from file")
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

def write_to_file_data(filename: str, offset: float, highest_sol: float):
    lines = []
    with open(filename, 'r') as f:
        for line in f:
            if "variable res_shift equal" in line:
                line = f"variable res_shift equal    {offset} # z shift when appending reservoir (angstroms)\n"
            if "variable freeze equal" in line:
                line = f"variable freeze equal    {highest_sol} # freeze height, set to height of slab and solution (angstroms)\n"
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
    print("Script needs 1 argument: [RESERVOIR OFFSET, in angstrom]")
    print("There is an optional extra argument to force the script to write the result to the input LAMMPS file: false/true (default false)")
    sys.exit() 

#Extract names of slab and solution files from input.lmp
res_file = get_data_files("input.lmp")
sol_file = "data.lmp"

#Calculate highest and lowest particles in slab
_, highest_sol = read_data(sol_file)
print(f"Highest particle z-position in slab + solution is: {highest_sol}")

#Calculate highest and lowest particles in solution
lowest_res, _ = read_data(res_file)
print(f"Lowest particle z-position in reservoir is: {lowest_res}")

#Calculate offset that must be added to LAMMPS input script
lmp_offset = highest_sol - lowest_res + offset
print(f"Offset that must be applied is: {lmp_offset}")

#Write to input file
if write_to_file:
    print("WRITING TO FILE... ", end="")
    write_to_file_data("input.lmp", lmp_offset, highest_sol)
    print("DONE")
