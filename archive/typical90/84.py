def solve():
    N = readi()
    S = input()

    xs = []
    os = []
    for i, v in enumerate(S):
        if v == 'x':
            xs.append(i)
        if v == 'o':
            os.append(i)

    ans = 0
    for i in range(N-1):
        c = S[i]
        if c == 'x':
            j = bisect_left(os, i+1)
            if j < len(os):
                ans += N-os[j]
        if c == 'o':
            j = bisect_left(xs, i+1)
            if j < len(xs):
                ans += N-xs[j]
        # print(l, ans)
    print(ans)


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
    def readmi(): return map(int, input().split())
    def readli(): return list(readmi())
    def readti(): return tuple(readmi())
    def readi(): return int(input())

    MOD = 10**9+7
    solve()
