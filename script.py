# This script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

from time import time

def sum_digits(n: int) -> int:
    parc = abs(n)
    sum_d = 0

    while parc > 0:
        sum_d += parc % 10
        parc //= 10

    return sum_d

def get_counterexmpl(max: int) -> (int, int):
    sums = get_sums(max)

    for a in range(max + 1):
        for b in range(a, max + 1):
            diff = sums[a + b] - sums[a] - sums[b]

            if not diff % 9 == 0:
                return (a, b)
    
    return None

def get_sums(max: int) -> list:
    output = []

    for i in range(2 * (max + 1) + 1):
        output.append(sum_digits(i))

    return output


print("\nThis script is a simple test for the following conjecture:\n")
print("Let S: N -> N be the sum of the digits of a positive integer.")
print("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n")
max_str = input("What value would you like to test the conjecture for? ")
print("\nLOADING. . .")

try:
    max = int(max_str)
    if max < 0:
        raise ValueError

    start = time()
    counterexmpl = get_counterexmpl(max)
    elepsed = time() - start

    print("LOADED. . . in {:.0f}ms\n".format(elepsed * 10**3))

    if counterexmpl == None:
        print("The conjecture is proved for all natural numbers smaller or equals to {}!".format(max))
    else:
        (a, b) = counterexmpl
        print("The conjecture is disproved! Here's a counterexample: ({}, {})".format(a, b))
except:
    print("'{}' isn't a natural number!".format(max_str))
