# scripts/plot_tot_gain.py
from pathlib import Path
import csv
import matplotlib.pyplot as plt

repo_root = Path(__file__).resolve().parents[1]   # .../kuhn_cfr
csv_path = repo_root / "src" / "tot_gain.csv"     # src/tot_gain.csv

xs, ys = [], []
with csv_path.open() as f:
    r = csv.DictReader(f)
    for row in r:
        xs.append(int(row["iter"]))
        ys.append(float(row["tot_gain"]))

plt.plot(xs, ys)
plt.xlabel("iteration")
plt.ylabel("tot_gain")
plt.tight_layout()
plt.show()
