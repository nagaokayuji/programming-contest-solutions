from pprint import pprint
import sys
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def solve():
    N, K = readti()
    V = readli()

    ans = 0
    for popc in range(1, min(N+1, K+1)):
        for lc in range(popc+1):
            for rc in range(popc-lc+1):
                q = []
                sm = 0
                for i in range(lc):
                    heappush(q, V[i])
                    sm += V[i]
                for j in range(rc):
                    heappush(q, V[N-1-j])
                    sm += V[N-1-j]
                ans = max(ans, sm)
                for _ in range(K-popc):
                    if q:
                        sm -= heappop(q)
                        ans = max(ans, sm)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
