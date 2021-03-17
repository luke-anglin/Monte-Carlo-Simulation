# Luke Anglin and Tobi Solarin
# rng.py
# Finds numbers u 51, 52, and 53 with the given parameters.
import numpy as np
import sys
import pprint as pp
# Increase rec limit
sys.setrecursionlimit(10000)
# Define params
a = 24693
c = 1753
K = pow(2, 15)
n = 1000

# Write recursive function


def rng(x):
    # Check if this is the seed
    if (x == 0):
        return 1000

    else:
        return (a * rng(x-1) + c) % K


print(rng(51)/K)
print(rng(52)/K)
print(rng(53)/K)

