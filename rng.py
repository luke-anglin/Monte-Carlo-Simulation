# Luke Anglin and Tobi Solarin
# rng.py
# Finds numbers u 51, 52, and 53 with the given parameters.

# Define params 
a = 24693
c = 1753 
K = pow(2, 15)

# Write recursive function 

def rng(x):
    # Check if this is the seed
    if (x == 0):
        return 1000 

    else:
        return (((a * rng(x-1) + c) % K)/K)

print(rng(51))
print(rng(52))
print(rng(53))
