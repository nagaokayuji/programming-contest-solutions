import sys
from collections import defaultdict, Counter, deque
from functools import lru_cache
import numpy as np
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


@lru_cache(maxsize=105055)
def fact(n):
    if n == 1:
        return 1
    else:
        return fact(n-1)*n % MOD


def _solve():
    N = readi()
    T = [readi() for _ in range(N)]
    T.sort()
    Ts = np.cumsum(T)
    print(sum(Ts))
    cnts = Counter(T)

    ans = 1
    for _, v in cnts.items():
        ans *= fact(v)
        ans %= MOD
    print(ans)

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
