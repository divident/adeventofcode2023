from dataclasses import dataclass
import enum

import numpy as np


class Dir(enum.IntEnum):
    U = 0
    D = 1
    R = 2
    L = 3

@dataclass
class Point2D:
    x: int
    y: int

def shoelace(points):
    vertices = [(e.x, e.y) for e in points]
    a = np.vstack((vertices, vertices[0]))
    S1 = sum(a[:-1,0] * a[1:,1])
    S2 = sum(a[:-1,1] * a[1:,0])
    return abs(S1-S2)/2

def main():

    with open("input2.txt") as f:
        content = f.read().split("\n")
    
    points = []
    x, y = 0, 0
    points.append(Point2D(x,y))
    line_sz = 0
    for line in content:
        hex = line[line.find("(")+2:-1]
        size = int(hex[:-1], 16)

        dir = int(hex[-1])
        line_sz += size
        if dir == 0:
            y += size
        elif dir == 2:
            y -= size
        elif dir == 3:
            x += size
        elif dir == 1:
            x -= size
        points.append(Point2D(x, y))
    
    area = shoelace(points) + line_sz // 2 + 1
    print(area)
    



main()