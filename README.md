# pi-digits
Compute nth digit of Pi using an asymptotic formula from [Plouffe (2022)](https://arxiv.org/abs/2201.12601). 

Currently implemented in the following programming languages:
- Python


## Explanation


For $n \geq 3$, the nth digit of $\pi$ is obtained by first calculating $\pi_n$.

$$\pi_n = \left( \frac{2 (-1)^{n+1} (2n)!}{2^{2n} B_{2n} (1 - 2^{-2n}) (1 - 3^{-2n}) (1 - 5^{-2n}) (1 - 7^{-2n}) (1 - 11^{-2n})} \right)^{\frac{1}{2n}}$$

where $B_{2n}$ is the [Bernoulli number](https://en.wikipedia.org/wiki/Bernoulli_number) of $2n$.

Then the nth digit of $\pi$ is given by:

$$d_n = \text{int} \left( 10 \text{frac} \left( 10^{n-1} \pi_{n-1} \right) \right)$$

The Bernoulli number can be obtained using the [Zeta function](https://en.wikipedia.org/wiki/Riemann_zeta_function) as follows:

$$B_{2n} = -2n * \zeta(1 - 2n)$$

## Usage

Make sure to have `mpmath` installed.

`pip install mpmath`

```python
from pi_python import pi_nth_digit
pi_nth_digit(10) # 5
```

To run tests:
```bash
pip install pytest
test_pi_python.py -s
```

## Credits

This asymptotic formula for calculating the nth digit of pi was discovered by [Simon Plouffe in 2022](https://arxiv.org/abs/2201.12601). The paper also discusses a way to calculate the nth digit of pi using [Euler numbers](https://en.wikipedia.org/wiki/Euler_numbers).

Big thanks to [Martin Bauer](https://twitter.com/martinmbauer/status/1614571838721622022?s=20&t=IznMtorWVeNbjlX-A5obNw) for the inspiration and illustration of this formula.

