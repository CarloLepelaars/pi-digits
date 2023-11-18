import mpmath 

def pi_nth_digit(n):
    def pi_n(n):
        B2n = -2*n * mpmath.zeta(1 - 2*n)
        nominator = 2 * (-1)**(n+1) * mpmath.factorial(2 * n)
        denominator = 2**(2 * n) * B2n * (1 - 2**(-2 * n)) * (1 - 3**(-2 * n))\
                                          * (1 - 5**(-2 * n)) * (1 - 7**(-2 * n))
        return (nominator / denominator) ** (1 / (2 * n))
    if n < 0:
        raise ValueError("n must be a non-negative integer")
    elif n == 0:
        return 3
    elif n == 1:
        return 1
    elif n == 2:
        return 4
    else:
        pi_n_minus_1 = pi_n(n-1)
        # Use mpmath for fractional part extraction
        frac_part = mpmath.fmod(pi_n_minus_1 * mpmath.power(10, n - 1), 1)
        dn = int(mpmath.floor(10 * frac_part))
        return dn
