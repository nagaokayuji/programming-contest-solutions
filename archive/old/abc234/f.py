import sys
from collections import defaultdict, Counter, deque
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


MOD = 998244353


class ModComb:
    def __init__(self, N=10**4, MOD=998244353):
        '''
        最大 N まで
        '''
        self.MOD = MOD
        self.N = N
        self.factorial = [1] * (N+1)

        for i in range(2, N+1):
            self.factorial[i] = self.factorial[i-1]*i % MOD

        self.inv_factorial = [1] * (N+1)
        self.inv_factorial[N] = pow(self.factorial[N], MOD-2, MOD)
        for i in range(N, 1, -1):
            self.inv_factorial[i-1] = self.inv_factorial[i]*i % MOD

    def comb(self, n, k):
        if n < k or n < 0 or k < 0:
            return 0
        return (self.factorial[n] * self.inv_factorial[k] % self.MOD) * self.inv_factorial[n-k] % self.MOD


def _solve():
    S = input()
    cnts = Counter(S)
    md = ModComb()

    N = len(S)
    dp = [0]*(N+1)
    dp[0] = 1

    for count in cnts.values():
        dp2 = [0]*(N+1)
        for length in range(N+1):
            for usecount in range(min(length, count)+1):
                dp2[length] += dp[length-usecount] * \
                    md.comb(length, usecount)
                dp2[length] %= MOD
        dp = dp2
    print(sum(dp[1:]) % MOD)


if __name__ == '__main__':
    _solve()
