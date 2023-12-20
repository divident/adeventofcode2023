
from queue import PriorityQueue

def main():
    with open("input1.txt") as f:
        content = f.read().split()
        grid = []
        for c in content:
            grid.append(list(map(int, [*c])))
        

    ans = walk(grid)
    print(f"{ans=}")

def heuristic(goal, curr):
    return abs(goal[0] - curr[0]) + abs(goal[1] - curr[1])

def walk(grid, minv=1, maxv=3):
    n, m = len(grid), len(grid[0])
    best = [[1<<31 for _ in range(m)] for _ in range(n)]

    best[0][0] = 0
    q = PriorityQueue()
    q.put((0, (0, 0, 0)))
    q.put((0, (0, 0, 1)))
    
    while not q.empty():
        cost, (x, y, dir) = q.get()
        if x == n - 1 and y == m - 1:
            for line in best:
                print(line)
            return best[n-1][m-1] - grid[0][0] 

        # can only turn left, right and straight
        for s in [-1, 1]:
            new_cost = best[x][y]
            newx, newy = x, y
            for i in range(1, maxv + 1):
                if dir == 1:
                    newx = x + i * s
                else:
                    newy = y + i * s

                if newx < 0 or newy < 0 or newx > n-1 or newy > m - 1:
                    break
                new_cost += grid[newx][newy]


                if best[newx][newy] > best[x][y] + grid[newx][newy]:
                    best[newx][newy] = new_cost
                    hh = heuristic((n-1, m-1), (newx, newy))
                    if i >= minv:
                        ndir = 1- dir
                        q.put((new_cost + hh, (newx, newy, ndir)))
    

    return -1


main()