from pprint import pprint
import sys
# from numba import njit, void, b1, i1, i4, i8, f8
# import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


N, K = mi()
MOD = 10**5

# mod 10^5 何回たされるか
# MOD 通りしかない→それ以降はあれ


def f(x: int):
    y = sum(map(int, list(str(x))))
    return (x+y) % MOD


nxt = [0]*MOD

for i in range(MOD):
    nxt[i] = f(i)

now = N
while K:
    nnxt = nxt.copy()
    if K & 1:
        now = nxt[now]

    for i in range(MOD):
        nnxt[i] = nxt[nxt[i]]
    nxt = nnxt

    K >>= 1

print(now)
