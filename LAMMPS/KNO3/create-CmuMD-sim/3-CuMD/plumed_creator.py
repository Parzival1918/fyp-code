#!/bin/python3
## SETTINGS FORMAT
## - File must only have 4 lines with this exact order:
## CONCENTRATION {value}
## FIXED {value}
## DCR {value}
## CRSIZE {value}

import sys

def read_settings(filename):
    settings = {}
    with open(filename, mode="r") as f:
        lines = f.readlines()

        if len(lines) != 4:
            print("ERROR: settings file not formatted properly")
            sys.exit(1)
        
        t_concentration = float(lines[0].split(" ")[1])
        fixed = float(lines[1].split(" ")[1])
        dcr = float(lines[2].split(" ")[1])
        crsize = float(lines[3].split(" ")[1])

    settings["t_concentration"] = t_concentration
    settings["fixed"] = fixed
    settings["dcr"] = dcr
    settings["crsize"] = crsize

    return settings


def read_data(filename):
    atoms = []
    with open(filename, mode="r") as f:
        line = ""
        while True:
            line = f.readline()
            if not line:
                print("ERROR: end-of-file reached without finding any atomic data")
                sys.exit(1)

            if "Atoms" in line:
                break
            
        if not "full" in line:
            print(f"ERROR: atomic style not 'full' in line {line}")
            sys.exit(1)
            
        f.readline()

        while True:
            line = f.readline().strip()
            if not line:
                break
            
            if "Velocities" in line or "Bonds" in line or "Angles" in line or "Impropers" in line:
                break
                    
            if line == "" or line.startswith("#"): # ignore empty or comment lines
                continue

            items = line.split(" ")
            if len(items) != 10:
                print(f"ERROR: line not formatted in atom style 'full': {line}")
                sys.exit(1)
            
            atom = {}
            atom["id"] = int(items[0])           
            atom["type"] = int(items[2])
            
            atoms.append(atom)

    if len(atoms) == 0:
        print("ERROR: no atoms found in data file!")
        sys.exit(1)

    return atoms


def create_plumed_file(filename, k_ids, cl_ids, ow_ids, settings):
    HEAD = """
RESTART

# Plumed script to run CmuMD
# Written by Dr. Stephen Yeandel, modified by Pedro Juan Royo
# See https://doi.org/10.1063/1.4917200 for more details
# Units are PLUMED default i.e. nm,ps,kJ/mol,amu

#========================================================================================
# Define groups of atoms
# =======================================================================================

"""

    CMUMD = """
#=========================================================================================
# CmuMD
#=========================================================================================

# Calculate the density of water, potassium and nitrate in the control region (CR):

# All distances are fractional in z
# FIXED: Position of the interface
# NSV: Number of atoms in the molecule
# DCR: Distance from the interface to the innermost edge of the CR
# CRSIZE: Size of the CR such that the edge of the reservoir is DCR+CRSIZE
# WF: Width of the region where the Fermi function (definining CmuMD forces) is applied
        
"""

    FORCES = """
# Apply CmuMD forces to maintain target ion densities AT X atoms/nm3
# KAPPA: force constant
        
"""

    END = """
# Print to file
# Print densities and total bias potential every N steps

PRINT ARG=n_potassium,n_nitrate,n_water,res_potassium.bias,res_nitrate.bias STRIDE=10000 FILE=CuMD.log

# Clear the buffers every N steps

FLUSH STRIDE=10000
        
"""

    conc = settings["t_concentration"]
    fix = settings["fixed"]
    dcr = settings["dcr"]
    crsize = settings["crsize"]

    with open(filename, mode="w") as f:
        f.write(HEAD)

        f.write("potassium: GROUP ATOMS=")
        f.write(",".join(str(x) for x in k_ids))
        f.write("\n\n")

        f.write("nitrate: GROUP ATOMS=")
        f.write(",".join(str(x) for x in no3_ids))
        f.write("\n\n")

        f.write("water: GROUP ATOMS=")
        f.write(",".join(str(x) for x in ow_ids))
        f.write("\n\n")

        f.write(CMUMD)

        f.write(f"n_potassium: CMUMD GROUP=potassium FIXED={fix} NSV=1 DCR={dcr} CRSIZE={crsize} WF=0.001 ASYMM=1\n")
        f.write(f"n_nitrate: CMUMD GROUP=nitrate FIXED={fix} NSV=1 DCR={dcr} CRSIZE={crsize} WF=0.001 ASYMM=1\n")
        f.write(f"n_water: CMUMD GROUP=water FIXED={fix} NSV=1 DCR={dcr} CRSIZE={crsize} WF=0.001 ASYMM=1\n")

        f.write(FORCES)

        f.write(f"res_potassium: RESTRAINT ARG=n_potassium AT={conc} KAPPA=10000.0\n")
        f.write(f"res_nitrate: RESTRAINT ARG=n_nitrate AT={conc} KAPPA=10000.0\n")

        f.write(END)

        
def main():
    settings_file = "plumed_creator.input"
    data_file = "data.lmp"
    plumed_file = "plumed.in"

    # read settings
    print("Reading settings... ", end="")
    settings = read_settings(settings_file)
    print("done")

    # read lammps data file
    print("Reading atoms from file... ", end="")
    atom_data = read_data(data_file)
    print("done")

    # separate atoms into IDs of Ow, K and Cl
    ow_ids = []
    k_ids = []
    no3_ids = []
    for atom in atom_data:
        if atom["type"] == 1:
            ow_ids.append(atom["id"])
        elif atom["type"] == 5:
            k_ids.append(atom["id"])
        elif atom["type"] == 2:
            no3_ids.append(atom["id"])

    # create plumed.in file
    print("Creating plumed input file... ", end="")
    create_plumed_file(plumed_file, k_ids, no3_ids, ow_ids, settings)
    print("done")


if __name__ == "__main__":
    main()
