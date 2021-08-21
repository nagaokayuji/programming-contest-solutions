n, m = map(int, input().split())
a = list(set((map(int, input().split()))))
a.sort()

# m 以下で 全部と互いに素
# gcd(1, N) = 1


class SmallestPrimeFactors:
    '''
    高速素因数分解
    最小の素数を返す
    O(N log N)
    '''

    def __init__(self, N: int):
        table = [i for i in range(N+1)]
        i = 2
        while i*i <= N:
            # 素数の場合
            if table[i] == i:
                j = i
                while j <= N:
                    if table[j] == j:
                        table[j] = i
                    j += i
            i += 1
        self._table = table

    def prime_factorize(self, N: int):
        factors = []
        n = N
        while n > 1:
            factor = self._table[n]
            n //= factor
            factors.append(factor)
        return factors


K = 10**5 + 4
spf = SmallestPrimeFactors(K+1)

nglist = [False] * (K)
nglist[0] = True
calculated_prime = [False] * (K)
calculated_prime[0] = True
calculated_prime[1] = True

for x in a:
    if x == 1:
        continue
    now = x
    for now in spf.prime_factorize(x):
        sp = now
        if calculated_prime[sp]:
            continue
        calculated_prime[sp] = True
        for y in range(sp, m+1, sp):
            nglist[y] = True

ans = []

for i in range(1, m+1):
    if not nglist[i]:
        ans.append(i)
print(len(ans))
print(*ans)
