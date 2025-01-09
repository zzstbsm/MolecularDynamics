set datafile separator ','
set key autotitle columnhead

# Plot pressure
set xlabel "Time"
set ylabel "Pressure" 
set style line 100 lt 1 lc rgb "grey" lw 0.5 
set grid ls 100 

plot filename using 1:5 with lines
