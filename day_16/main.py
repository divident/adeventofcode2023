from collections import deque
import enum
from typing import Iterable
class Direction(enum.Enum):
    N = 0
    S = 1
    E = 2
    W = 3

def inside(nx, ny, n, m):
    if 0 <= nx < n and 0 <= ny < m:
        return True
    return False

def new_dir(dir, el):
    if el == '.':
        return dir
    elif el == '-':
        if dir == Direction.E or dir == Direction.W:
            return dir
        else:
            return (Direction.E, Direction.W)
    elif el == '|':
        if dir == Direction.N or dir == Direction.S:
            return dir
        else:
            return (Direction.N, Direction.S)
    elif el == '/':
        if dir == Direction.W:
            return Direction.S
        elif dir == Direction.E:
            return Direction.N
        elif dir == Direction.N:
            return Direction.E
        elif dir == Direction.S:
            return Direction.W
    elif el == "\\":
        if dir == Direction.W:
            return Direction.N
        elif dir == Direction.E:
            return Direction.S
        elif dir == Direction.N:
            return Direction.W
        elif dir == Direction.S:
            return Direction.E

def main():
    with open("data/input1.txt") as input:
        lines = list(map(lambda x: x.strip(), input.read().split("\n")))

    n, m = len(lines), len(lines[0]) 
    max_ans = 0

    max_ans = max(max_ans, walk((0, 0), Direction.E, lines))
    max_ans = max(max_ans, walk((0, 0), Direction.S, lines))


    max_ans = max(max_ans, walk((n-1, m-1), Direction.N, lines))
    max_ans = max(max_ans, walk((n-1, m-1), Direction.W, lines))


    max_ans = max(max_ans, walk((0, m-1), Direction.S, lines))
    max_ans = max(max_ans, walk((0, m-1), Direction.W, lines))


    max_ans = max(max_ans, walk((n-1, 0), Direction.N, lines))
    max_ans = max(max_ans, walk((n-1, 0), Direction.E, lines))

    for i in range(1, n-1):
        max_ans = max(max_ans, walk((i, 0), Direction.E, lines))
    for i in range(1, n-1):
        max_ans = max(max_ans, walk((i, m-1), Direction.W, lines))

    for j in range(1, m-1):
        max_ans = max(max_ans, walk((0, j), Direction.S, lines))

    for j in range(1, m-1):
        max_ans = max(max_ans, walk((n - 1, j), Direction.N, lines))

    print(f"final={max_ans}")

def walk(s, dir, lines):
    q = deque()
    nx, ny = s
    ndir = new_dir(dir, lines[ny][nx])
    if isinstance(ndir, tuple):
        q.append((nx, ny, ndir[0]))
        q.append((nx, ny, ndir[1]))
    else:
        q.append((nx, ny, ndir))

    visited = set()
    n, m = len(lines), len(lines[0])
    while q:
        x, y, dir = q.popleft()

        visited.add((x, y, dir))
        if dir == Direction.N:
            mm = (-1, 0)
        elif dir == Direction.S:
            mm = (1, 0)
        elif dir == Direction.E:
            mm = (0, 1)
        elif dir == Direction.W:
            mm = (0, -1)

        nx = x + mm[0]
        ny = y + mm[1]
        #print(f"{x=} {y=} {dir=}")
        if 0 <= nx < n and 0 <= ny < m:
            ndir = new_dir(dir, lines[nx][ny])
            if isinstance(ndir, tuple):
                if (nx, ny, ndir[0]) not in visited:
                    q.append((nx, ny, ndir[0]))
                if (nx, ny, ndir[1]) not in visited:
                    q.append((nx, ny, ndir[1]))
            else:
                if (nx, ny, ndir) not in visited:
                    q.append((nx, ny, ndir))
                
    ans = len(set([(x, y) for x, y, _ in visited]))
    #print(f"{ans=} {s=}")
    return ans

main() 