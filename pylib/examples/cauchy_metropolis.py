import rust_metropolis as rmt
import numpy as np
from scipy.stats import cauchy
import matplotlib.pyplot as plt

n_samples = 20000
n_bins = int(2 * n_samples ** (1 / 2))
dist_center = 1.5


def p(x):
    return 1 / (1 + (x - dist_center)**2)


x0 = 1.0  # initial value for Markov chain
vals = rmt.random_walk.sample_random_walk(n_samples, x0, p)

mean_val = np.mean(vals)

plt.hist(vals, bins=n_bins, alpha=0.8, density=True)
window = 8
xlims = (dist_center - window, dist_center + window)

xvals = np.linspace(*xlims, 200)
yvals = cauchy.pdf(xvals, dist_center)
plt.plot(xvals, yvals)
plt.xlim(*xlims)
plt.axvline(mean_val, 0, 1, )

plt.show()
