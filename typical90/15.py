from pprint import pprint
from collections import defaultdict, Counter, deque
from itertools import permutations, combinations, product, combinations_with_replacement, groupby, accumulate
from math import sqrt, gcd, factorial
from bisect import bisect_left, bisect_right
from heapq import heappush, heappop, heapify, heappushpop, heapreplace

N = int(input())
MOD = 10**9+7

if N == 1:
    print(1)
    exit()
if N == 2:
    print(3)
    print(2)
    exit()


class ModComb:
    '''
    O(N) で初期化、O(1) でクエリ処理
    二項係数を求める
    '''

    def __init__(self, N=10**6, MOD=10**9 + 7):
        '''
        最大 N まで
        '''
        self.MOD = MOD
        self.N = N
        self.factorial = [1] * (N+1)

        for i in range(2, N+1):
            self.factorial[i] = self.factorial[i-1]*i % MOD

        self.inv_factorial = [1] * (N+1)
        # N! の逆元を先に計算
        self.inv_factorial[N] = pow(self.factorial[N], MOD-2, MOD)
        # (N-1)! / (N!) * N = 1
        for i in range(N, 1, -1):
            self.inv_factorial[i-1] = self.inv_factorial[i]*i % MOD

    def comb(self, n, k):
        '''
        二項係数 nCk
        '''
        if n < k or n < 0 or k < 0:
            return 0
        return self.factorial[n] * self.inv_factorial[k] * self.inv_factorial[n-k] % self.MOD

    def comb_multi(self, n, k):
        '''
        重複組合せ nHk
        '''
        return self.comb(n+k-1, k)


md = ModComb()

for k in range(1, N+1):
    ans = 0
    for i in range(1, N+1):
        n = N-(k-1)*(i-1)
        if n < i:
            break
        ans += md.comb(n, i)
    print(ans % MOD)
