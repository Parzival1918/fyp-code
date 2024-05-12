function style_queue()
{
   center=1 # center text content in screen
   left_align=0
   if [ $center -eq 0 ]; then
           left_align=25
   else
           left_align=$(( (COLUMNS - 39) / 2 ))
   fi
   sub_a=$((left_align - 10))
   title_align=$((left_align - 3))
   if (( $COLUMNS < 60 )); then
           tput setaf 1; tput bold; echo "TERMINAL SCREEN NOT WIDE ENOUGH"; tput sgr0
           return
   fi
   tput setaf 2; tput bold; printf "%${title_align}s\n" "RUNNING JOBS:"; tput sgr0
   squeue -u $USER | while read a _ c _ e f g h; do
           if [[ "$e" == "ST" ]]; then 
                   printf "%${sub_a}s" ""
                   tput smul; tput bold; printf "%10s %10s %10s %6s %14s    \n" "JOB ID" "JOB NAME" "TIME" "NODES" "NODELIST"; tput rmul; tput sgr0
           elif [[ "$e" == "R" ]]; then
                   printf "%${left_align}s" $a
                   tput bold; printf "%10s" $c; tput sgr0
                   printf "%12s" $f
                   printf "%7s" $g
                   printf "%15s" $h
                   printf "\n"
           fi
   done
   echo                                                                                                                                                                                                
   tput setaf 3; tput bold; printf "%${title_align}s\n" "PENDING JOBS:"; tput sgr0
   squeue -u $USER | while read a _ c _ e f g h; do
           if [[ "$e" == "ST" ]]; then 
                   printf "%${sub_a}s" ""
                   tput smul; tput bold; printf "%10s %10s %10s %6s %14s    \n" "JOB ID" "JOB NAME" "TIME" "NODES" "REASON"; tput rmul; tput sgr0
           elif [[ "$e" == "PD" ]]; then
                   printf "%${left_align}s" $a
                   tput bold; printf "%10s" $c; tput sgr0
                   printf "%12s" $f
                   printf "%7s" $g
                   printf "%15s" $h
                   printf "\n"
           fi
   done
}

alias watchq='
while true ; do
  clear # to clear the screen
  style_queue
  sleep 5
done
'

alias q='style_queue'
