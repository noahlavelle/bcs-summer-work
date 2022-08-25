# std regex library for numberplate mapping
import re
from datetime import datetime

# Times are required in the format HOUR:MINUTE:SECOND
t1, t2, d, numplate = input("Time 1 (HH:MM:SS)"), input("Time 2 (HH:MM:SS)"), int(input("Distance (M)")), input("Numberplate")

if (bool(re.match(r"^(([A-z]){2})(([0-9]){2}) (([A-z]){3})$", numplate))): 
    FMT = '%H:%M:%S'
    tdelta = datetime.strptime(t2, FMT) - datetime.strptime(t1, FMT)
    print("Speed =", d / tdelta.total_seconds())
else:
    print("Numberplate not in format XX00 XXX")
