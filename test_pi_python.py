import mpmath

from pi_python import pi_nth_digit

MAX_PRECISION_REQUIRED = 17
mpmath.mp.dps = MAX_PRECISION_REQUIRED

def test_pi_nth_digit():
    pi_str = str(mpmath.pi)
    for i in range(1, len(pi_str) - 2): 
        expected_digit = int(pi_str[i+1])
        calculated_digit = pi_nth_digit(i)
        assert calculated_digit == expected_digit, f"Mismatch at the {i}th digit of pi."