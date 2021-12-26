from pprint import pprint
from collections import defaultdict, Counter, deque
from numba import njit, b1, i1, i4, i8, f8
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
import numpy as np
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
def mi(): return map(int, input().split())


N, K = mi()
if K == 1:
    print(N-1)
    exit()


@njit("void(i8,i8)")
def solve(N, K):
    cnt = np.zeros(N+1, dtype=np.int64)
    ans = 0
    for i in range(2, N+1):
        if cnt[i] == 0:
            j = i + i
            while j <= N:
                cnt[j] += 1
                j += i
        if cnt[i] >= K:
            ans += 1

    print(ans)


solve(N, K)
