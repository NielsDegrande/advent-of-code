import os
import sys
from timeit import Timer
from typing import Callable

from expense_report_speed import (
    read_data,
    find_expense_product_with_cartesian_product,
    find_expense_product_with_sort_and_early_stopping,
)

# Set number of cycles for speed benchmark.
CYCLES = 10

# Disable printing.
# sys.stdout = open(os.devnull, 'w')

# Read data.
data = read_data(f"{os.path.dirname(__file__)}/data.txt")

# Approach 1 without filtering.
timer_ = Timer(
    lambda: find_expense_product_with_cartesian_product(data, False)
)
print("Approach 1 without filtering: ", timer_.timeit(number=CYCLES))

# Approach 2 without filtering.
timer_ = Timer(
    lambda: find_expense_product_with_sort_and_early_stopping(data, False)
)
print("Approach 2 without filtering: ", timer_.timeit(number=CYCLES))

# Approach 1 with filtered data.
timer_ = Timer(lambda: find_expense_product_with_cartesian_product(data, True))
print("Approach 1 with filtered data: ", timer_.timeit(number=CYCLES))

# Approach 2 with filtered data.
timer_ = Timer(
    lambda: find_expense_product_with_sort_and_early_stopping(data, True)
)
print("Approach 2 with filtered data: ", timer_.timeit(number=CYCLES))
