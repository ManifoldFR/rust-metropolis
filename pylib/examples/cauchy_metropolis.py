import rust_metropolis as rmt
import numpy as np
from scipy.stats import cauchy
import matplotlib.pyplot as plt

n_samples = 10000
n_bins = int(2 * n_samples ** (1 / 3))

x0 = 1.5
vals = rmt.random_walk.sample_random_walk(n_samples, x0)


plt.hist(vals, bins=n_bins, alpha=0.8, density=True)
xlims = plt.xlim()

xvals = np.linspace(*xlims, 200)
yvals = cauchy.pdf(xvals, x0)
plt.plot(xvals, yvals)

plt.show()
