"""Using cartesian product approach with filtering the data is the fastest."""

import os

import numpy as np

EXPENSE_SUM = 2020


def read_data(path: str) -> np.ndarray:
    """Read expense data.

    :param path: Path to expense data.
    :return: Expense data as numpy array.
    """
    expenses = np.loadtxt(path, dtype=int)
    print(f"Entries: {len(expenses)}")
    return expenses


def filter_expenses(expenses: np.ndarray) -> np.ndarray:
    """Filter expense data using some quick wins.

    NOTE: Finding min requires parsing the whole array.

    :param expenses: Expense data to filter.
    :return: Filtered expense data.
    """
    min_ = expenses.min()
    limit = EXPENSE_SUM - min_
    print(f"Entries above {limit} can be filtered given min is {min_}")
    expenses = expenses[expenses <= limit]
    print(f"Entries after filtering: {len(expenses)}")
    return expenses


def find_expense_product_with_cartesian_product(
    expenses: np.ndarray, filter: bool
) -> None:
    """Approach 1: Cartesian product and sum.

    :param expenses: Expense data.
    :param filter: Flag to indicate if expenses should be filtered.
    """
    if filter:
        expenses = filter_expenses(expenses)

    # Create cartesion product by tiling and repeating.
    expenses_combinations = np.transpose(
        [np.tile(expenses, len(expenses)), np.repeat(expenses, len(expenses))]
    )
    expenses_sum = np.column_stack(
        (expenses_combinations, expenses_combinations.sum(axis=1))
    )
    result = expenses_sum[expenses_sum[:, 2] == EXPENSE_SUM, :][0]
    print(f"The answer is {result[0]} * {result[1]} = {result[0] * result[1]}")


def find_expense_product_with_sort_and_early_stopping(
    expenses: np.ndarray,
    filter: bool,
) -> None:
    """Approach 2: sort data and early stopping.

    :param expenses: Expense data.
    :param filter: Flag to indicate if expenses should be filtered.
    """
    if filter:
        expenses = filter_expenses(expenses)

    def loop_expenses(expenses_sorted: np.ndarray) -> tuple:
        """Intelligently loop across combinations in the expense data.

        :param expenses_sorted: Sorted expense data.
        :return: Indexes of expenses that sum to EXPENSE_SUM.
        """
        for i in range(0, len(expenses_sorted)):
            for j in range(i, len(expenses_sorted)):
                if expenses_sorted[i] + expenses_sorted[j] == EXPENSE_SUM:
                    return i, j

    expenses_sorted = np.sort(expenses)
    i, j = loop_expenses(expenses_sorted)
    print(
        "The answer is "
        f"{expenses_sorted[i]} * {expenses_sorted[j]} = "
        f"{expenses_sorted[i] * expenses_sorted[j]}"
    )


if __name__ == "__main__":

    data = read_data(f"{os.path.dirname(__file__)}/data.txt")
    find_expense_product_with_cartesian_product(data, filter=True)
