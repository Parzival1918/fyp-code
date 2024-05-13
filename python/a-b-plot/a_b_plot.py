#!.venv/bin/python3

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

filename = "joined.csv"

df = pd.read_csv(filename, header=None)
volume = df.iloc[:, 0]
area = df.iloc[:, 1]
step = df.iloc[:, 2]

fits = []
limits_low = []
limits_high = []
for step_val in pd.unique(df[2]):
    vol_steps = df.loc[df.iloc[:, 2] == step_val, 0].to_numpy()
    area_steps = df.loc[df.iloc[:, 2] == step_val, 1].to_numpy()
    min_x = min(vol_steps)
    max_x = max(vol_steps)
    min_y = min(area_steps)
    max_y = max(area_steps)
    limits_high.append([max_x, max_y])
    limits_low.append([min_x, min_y])

    lin_fit = np.polynomial.Polynomial.fit(vol_steps, area_steps, deg=1)
    fits.append(lin_fit)

    # need to calculate fit from normalised data 
    # to tell how slop changes compared to each run
    scaled_v = (vol_steps - min_x) / (max_x - min_x)
    scaled_a = (area_steps - min_y) / (max_y - min_y)
    fit = np.polynomial.Polynomial.fit(scaled_v, scaled_a, deg=1)
    print(fit)

fig, ax = plt.subplots()
scatter = ax.scatter(volume, area, c=step, alpha=0.8)
for i, fit in enumerate(fits):
    x = np.array([limits_low[i][0], limits_high[i][0]])
    ax.plot(x, fit(x))

ax.set_xlabel("Bulk atoms", fontsize=15)
ax.set_ylabel("Surface atoms", fontsize=15)
ax.set_title("Surface and Bulk atoms over run")

ax.grid(True)
fig.tight_layout()
fig.colorbar(scatter)

plt.show()
