"""
Make a histogram
"""
import numpy as np
from scipy.stats import cauchy
from matplotlib import pyplot as plt
import json

with open("examples/output.json") as f:
    data = json.load(f)

burn_in = 500
data = data[burn_in:]
M = len(data)
print("No. of samples kept:", M)
n_bins = int(4*M**(1/3))
plt.hist(data, bins=n_bins, rwidth=0.8, density=True)
xlims = (-8, 4)
plt.xlim(*xlims)
xvals = np.linspace(*xlims, num=300)
dist_center = -1.5
plt.plot(xvals, cauchy.pdf(xvals, dist_center))
plt.show()
