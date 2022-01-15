N, K = map(int, input().split())


class ModComb:
    def __init__(self, N=10**6, MOD=10**9 + 7):
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
        if n < k or n < 0 or k < 0:
            return 0
        return (self.factorial[n] * self.inv_factorial[k] % self.MOD) * self.inv_factorial[n-k] % self.MOD

    def comb_multi(self, n, k):
        return self.comb(n+k-1, k)


MOD = 10**9+7
redc = N-K
md = ModComb(N=8000)
for i in range(1, K+1):
    # r r r r r
    #
    place_select = md.comb(redc+1, i)
    place_blue = md.comb(K-1, i-1)
    print(place_select*place_blue % MOD)
