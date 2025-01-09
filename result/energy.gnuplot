set datafile separator ','
set key autotitle columnhead

# # Styles
# set style line 101 lw 3 lt rgb "#f62aa0" # style for targetValue (1) (pink)
# set style line 102 lw 3 lt rgb "#26dfd0" # style for measuredValue (2) (light blue)
# set style line 103 lw 4 lt rgb "#b8ee30" # style for secondYAxisValue (3) (limegreen)

# Plot Energy
set xlabel "Time"
set ylabel "Energy" 
set style line 100 lt 1 lc rgb "grey" lw 0.5 
set grid ls 100 
plot filename using 1:2 with lines, '' using 1:3 with lines, '' using 1:4 with lines 

