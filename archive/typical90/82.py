def solve():
    L, R = mi()
    ans = 0

    for i in range(1, 20):
        # L to R
        l = max(L, 10**(i-1))
        r = min(R, 10**i - 1)
        if l > r:
            continue
        ans += i * (r*(r+1)//2 - l*(l-1)//2)
        ans %= MOD
    print((ans+MOD) % MOD)


if __name__ == '__main__':
    from pprint import pprint
    import sys
    from collections import defaultdict, Counter, deque
    from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
    from math import sqrt, gcd, factorial
    from bisect import bisect_left, bisect_right
    from heapq import heappush, heappop, heapify, heappushpop, heapreplace
    # from numba import njit, void, b1, i1, i4, i8, f8
    # import numpy as np
    INF = float('inf')
    def input(): return sys.stdin.readline().rstrip()
    def mi(): return map(int, input().split())
    def li(): return list(mi())
    def ti(): return tuple(mi())
    def i(): return int(input())

    MOD = 10**9+7
    solve()
