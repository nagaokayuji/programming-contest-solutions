import sys
from numba import njit, void, b1, i1, i4, i8, f8
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


@njit(f8(i8, i8, i8, i8), cache=True)
def calc(l, r, nl, nr):
    cnt = 0
    all = 0
    for v in range(l, r+1):
        for nx in range(nl, nr+1):
            if v > nx:
                cnt += 1
            all += 1
    return cnt/all


N = int(input())
LR = [ti() for _ in range(N)]
ans = 0
for i, (l, r) in enumerate(LR):
    for nl, nr in LR[i+1:]:
        ans += calc(l, r, nl, nr)
print(ans)
