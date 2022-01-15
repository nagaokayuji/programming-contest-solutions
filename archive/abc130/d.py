import sys
from bisect import bisect_left, bisect_right
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, K = mi()
    A = list(mi())
    sumA = np.cumsum([0]+A)
    # print(sumA)

    ans = 0
    # sum_[l,r] = sumA[r] - sumA[l-1]
    for l in range(1, N+1):
        r = bisect_left(sumA, K+sumA[l-1])
        ans += N+1-r
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
