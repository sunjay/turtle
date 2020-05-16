#!/usr/bin/env python3

# Script for graphing the times outputted by the following commands:
#   cargo run --example rotationtest
#   cargo run --example rotationtest2
#   cargo run --example speedtest

import sys
import os.path

from sys import stdin

import numpy as np
import matplotlib.pyplot as plt

if len(sys.argv) > 2 or (len(sys.argv) == 2 and sys.argv[1] == "--help"):
    print("Usage: ./graph_times.py [output_file] [< output_file]")
    print("  If command line argument is provided, a png will be produced")
    print("  If reading from stdin, a plot will be shown interactively")
    sys.exit(1)

elif len(sys.argv) == 2:
    input = open(sys.argv[1])

    base = os.path.basename(sys.argv[1])
    name, _ = os.path.splitext(base)
    output_file = name + ".png"

else:
    input = stdin
    output_file = None

# data = { "label": [(speed, time in ms), ...], ... }
data = {}
for line in input:
    line = line.replace(':', '')
    label, speed, time_ms, *_ = line.split(' ')
    data.setdefault(label, []).append((int(speed), int(time_ms)))

fig, axs = plt.subplots(1, len(data), figsize=(6*len(data), 4))
if len(data) == 1:
    axs = [axs]

colors = iter(plt.cm.rainbow(np.linspace(0, 1, len(data))))
for ax, (label, series), color in zip(axs, data.items(), colors):
    x, y = zip(*series)
    ax.plot(x, y, marker='o', label=label, color=color)

for ax in axs:
    ax.set_xlabel("speed level")
    ax.set_ylabel("time (ms)")
    ax.legend()

if output_file is None:
    plt.show()
else:
    plt.savefig(output_file)
