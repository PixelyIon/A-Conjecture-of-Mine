# This script is a simple test for the following conjecture:

# Let S: N -> N be the sum of the digits of a positive integer.
# For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.

from time import time

def sum_digits(n):
    parc = abs(n)
    sum_d = 0

    while parc > 0:
        sum_d += parc % 10
        parc //= 10

    return sum_d

def get_counterexmpl(n):
    for a in range(n + 1):
        for b in range(a, n + 1):
            diff = sum_digits(a + b) - (sum_digits(a) + sum_digits(b))

            if not diff % 9 == 0:
                return (a, b)
    
    return None  


print("\nThis script is a simple test for the following conjecture:\n")
print("Let S: N -> N be the sum of the digits of a positive integer.")
print("For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an interger.\n")
max_str = input("What value would you like to test the conjecture for? ")
print("\nLOADING. . .")

try:
    maximum = int(max_str)
    if maximum < 0:
        raise ValueError

    start = time()
    counterexmpl = get_counterexmpl(maximum)
    elepsed = time() - start

    print("LOADED. . . in {:.0f}ms\n".format(elepsed * 10**3))

    if counterexmpl == None:
        print("The conjecture is proved for all natural numbers smaller or equals to {}!".format(maximum))
    else:
        (a, b) = counterexmpl
        print("The conjecture is disproved! Here's a counterexample: ({}, {})".format(a, b))
except:
    print("'{}' isn't a natural number!".format(max_str))
