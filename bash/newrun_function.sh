# Function to create new runs of a system
function newrun {
   # Check that in the wd there is a template folder
   if [ ! -d "template" ]; then
           tput setaf 1; tput bold;
           echo "ERROR: no template folder found"
           tput sgr0
           return 1
   fi
   # Find the last folder named run_*
   last_folder_num=$(ls -1v | grep run_ | tail -1 | cut -d "_" -f 2)
   next_folder_num=$((last_folder_num + 1))
   # Create the new folder run
   mkdir "run_$next_folder_num" && cd "run_$next_folder_num"
   tput setaf 2; echo -n "created new folder: "; tput bold; tput setaf 3; echo "run_$next_folder_num"; tput sgr0
   # Copy the template files
   cp ../template/* ./
   tput setaf 2; echo -n "copied files from: "; tput bold; tput setaf 3; echo "../template/"; tput sgr0
   # Copy the prod_data.lmp file from last run to this new run as data.lmp
   # Check first the file exists
   if [ ! -f ../run_$last_folder_num/prod_data.lmp ]; then
           tput setaf 1; tput bold;
           echo "ERROR: no prod_data.lmp file in run_$last_folder_num"
           cd .. && rm -r run_$next_folder_num
           echo "deleting created folder run_$next_folder_num"
           tput sgr0
           return 1
   fi
   cp ../run_$last_folder_num/prod_data.lmp ./data.lmp
   tput setaf 2; echo -n "copied "; tput bold; tput setaf 3; echo -n "../run_$last_folder_num/prod_data.lmp "; tput sgr0; tput setaf 2; echo -n "as "; tput bold; tput setaf 3; echo "data.lmp"; tput sgr0
   # Change the job name
   # Check first the file exists
   if [ ! -f run.lmp ]; then
           tput setaf 1; tput bold;
           echo "ERROR: no run.lmp file in run_$next_folder_num"
           tput sgr0
   else
           new_job_name=$(cat run.lmp | grep "#SBATCH --job-name" | cut -d "=" -f 2)
           sed -i "/#SBATCH --job-name=/ s/$/_$next_folder_num/" run.lmp && tput setaf 2; echo -n "job name: "; tput setaf 3; tput bold; echo "${new_job_name}_$next_folder_num"; tput sgr0
   fi
   # List contents of new folder
   echo "new folder contents:"
   ls
}
