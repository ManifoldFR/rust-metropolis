import rust_metropolis as rmt
import numpy as np
from scipy.stats import cauchy
import matplotlib.pyplot as plt

n_samples = 30000
n_bins = int(2 * n_samples ** (2 / 3))

x0 = 1.0  # initial value for Markov chain
vals = rmt.random_walk.sample_random_walk(n_samples, x0)

plt.hist(vals, bins=n_bins, alpha=0.8, density=True)
window = 8
xlims = (1.5 - window, 1.5 + window)

xvals = np.linspace(*xlims, 200)
yvals = cauchy.pdf(xvals, 1.5)
plt.plot(xvals, yvals)
plt.xlim(*xlims)
plt.show()
