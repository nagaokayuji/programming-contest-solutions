import sys
from collections import defaultdict, Counter, deque
# from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
# from numpy import searchsorted
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    H, W = readmi()
    ch, cw = readmi()
    dh, dw = readmi()
    S = [input() for _ in range(H)]

    st = (ch-1, cw-1)
    goal = (dh-1, dw-1)

    q = deque()
    q.append((st[0], st[1], 0))
    # dist = [[INF]*W for _ in range(H)]
    dist = np.full((H, W), INF, dtype=np.int64)

    dx = [-1, 0, 0, 1]
    dy = [0, -1, 1, 0]
    while q:
        i, j, count = q.popleft()
        dist[i][j] = min(dist[i][j], count)

        for di, dj in zip(dx, dy):
            nxi, nxj = i+di, j+dj
            if 0 <= nxi < H and 0 <= nxj < W and S[nxi][nxj] == '.':
                if dist[nxi][nxj] > count:
                    q.appendleft((nxi, nxj, count))
                    dist[nxi][nxj] = count

        for di in range(-2, 3):
            for dj in range(-2, 3):
                nxi, nxj = i+di, j+dj
                if 0 <= nxi < H and 0 <= nxj < W and S[nxi][nxj] == '.':
                    if dist[nxi][nxj] > count+1:
                        q.append((nxi, nxj, count+1))
                        dist[nxi][nxj] = count+1

    ans = dist[goal[0]][goal[1]]
    if ans < INF:
        print(ans)
    else:
        print(-1)


if __name__ == '__main__':
    _solve()
