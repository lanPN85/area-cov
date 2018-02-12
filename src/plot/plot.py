#!usr/bin/env python

from __future__ import print_function

import sys
try:
	import matplotlib.pyplot as plt
except ImportError:
	print('Missing python modules. Run pip install -r requirements.txt to install them.')
	exit(1)
COLORS = ('#a54040', '#406fa5', '#4f824c', '#c69559')


def load_results(path):
	with open(path, 'rt') as f:
		lines = f.readlines()
		w, h = lines[0].strip().split(' ')
		w, h = float(w), float(h)
		cov = float(lines[1].strip())

		# Group circles by their radius for coloring
		circles = {}
		rd = 0.
		for l in lines[2:]:
			x, y, r = l.strip().split(' ')
			x, y, r = float(x), float(y), float(r)
			if r != rd:
				rd = r
				circles[rd] = []
			circles[rd].append((x, y))
		
		return w, h, cov, circles


def draw_circle(center, radius, ax, **kwargs):
	circle = plt.Circle(center, radius, **kwargs)
	ax.add_artist(circle)
	ax.plot(center[0], center[1], 'o', color=kwargs.get('color', 'black'))


def draw_graph(w, h, cov, circles):
	fig = plt.figure()
	ax = fig.add_subplot(111)

	# Plot boundaries
	ax.plot([0, w], [0, 0], color='black')
	ax.plot([w, w], [0, h], color='black')
	ax.plot([0, w], [h, h], color='black')
	ax.plot([0, 0], [0, h], color='black')

	for i, r in enumerate(circles.keys()):
		color = COLORS[i % len(COLORS)]
		for c in circles[r]:
			draw_circle(c, r, ax, alpha=0.5, color=color)
	ax.set_title('Coverage: %.2f' % cov, fontsize=10)
	return fig, ax


if __name__ == '__main__':
	if len(sys.argv) < 3:
		print('Usage: plot.py [INPUT_FILE] [OUTPUT_FILE]')
		exit(1)
	fin = sys.argv[1]
	fout = sys.argv[2]
	if len(sys.argv) == 4 and sys.argv[3] == '--xkcd':
		plt.xkcd()
	plot_name = fin.split('/')[-1].split('.')[0]

	w, h, cov, circles = load_results(fin)
	fig, ax = draw_graph(w, h, cov, circles)
	ax.axis('off')
	fig.suptitle(plot_name, fontsize=14)
	fig.savefig(fout)
