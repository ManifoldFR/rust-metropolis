"""
Make a histogram
"""
import numpy as np
from scipy.stats import laplace
from matplotlib import pyplot as plt
import json

with open("output.json") as f:
    data = json.load(f)

burn_in = 1000
data = data[burn_in:]
M = len(data)
print("No. of samples kept:", M)
n_bins = int(2*M**(1/3))
plt.hist(data, bins=n_bins, rwidth=0.8, density=True)
xlims = plt.xlim()
xvals = np.linspace(*xlims, num=100)
plt.plot(xvals, laplace.pdf(xvals))
plt.show()
