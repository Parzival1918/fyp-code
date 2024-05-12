#!.venv/bin/python3

import pandas as pd
import matplotlib.pyplot as plt

filename = "joined.csv"

df = pd.read_csv(filename, header=None)
volume = df.iloc[:, 0]
area = df.iloc[:, 1]
step = df.iloc[:, 2]

fig, ax = plt.subplots()
scatter = ax.scatter(volume, area, c=step, alpha=0.8)

ax.set_xlabel("Bulk atoms", fontsize=15)
ax.set_ylabel("Surface atoms", fontsize=15)
ax.set_title("Surface and Bulk atoms over run")

ax.grid(True)
fig.tight_layout()
fig.colorbar(scatter)

plt.show()
