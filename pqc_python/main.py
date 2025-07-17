from sympy import isprime, totient, primitive_root, gcd
from random import getrandbits, choice
import matplotlib.pyplot as plt

def generate_large_prime(bit_length=64):
    while True:
        p = getrandbits(bit_length) | 1  # ensure p is odd
        if isprime(p):
            return p


def generator_output_random(p):
    n = p - 1
    g0 = primitive_root(p)

    # Get all coprime exponents k with gcd(k, n) = 1
    coprime_exponents = [k for k in range(1, n) if gcd(k, n) == 1]

    # Randomly select one exponent
    k = choice(coprime_exponents)

    return pow(g0, k, p) / p  # normalized output in (0, 1)

# --- Main ---
num_primes = 200
bit_length = 64
normalized_outputs = []

print("Generating normalized generator outputs (random index)...")
for _ in range(num_primes):
    p = generate_large_prime(bit_length)
    result = generator_output_random(p)
    normalized_outputs.append(result)

# --- Plot ---
plt.figure(figsize=(10, 6))
plt.hist(normalized_outputs, bins=50, edgecolor='black')
plt.xlabel("Normalized generator output (generator / p)")
plt.ylabel("Frequency")
plt.title("Distribution of generator outputs with random index (large safe primes)")
plt.grid(True, axis='y', linestyle='--', alpha=0.5)
plt.tight_layout()
plt.show()
