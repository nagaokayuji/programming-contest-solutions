import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


class RollingHash():
    def __init__(self, s: str, base=10007, mod=2**61 - 1):
        n = len(s)
        self.mod = mod
        self.hash = hash = [0]*(n+1)
        self.pw = pw = [1]*(len(s)+1)

        for i in range(n):
            pw[i+1] = pw[i] * base % mod

        for i in range(n):
            hash[i+1] = (hash[i] * base + ord(s[i])) % mod

    def get(self, l: int, r: int):
        '''
        get hash of [l,r)  (0-indexed)
        '''
        return (self.hash[r] - self.hash[l] * self.pw[r-l]) % self.mod


def _solve():
    ''''''
    N, = li()
    S = input()
    rh = RollingHash(S)

    ans = 0
    for i in range(N):
        for j in range(i, N):
            l = j-i
            while (ans < l) and (j+ans < N) and rh.get(i, i+ans+1) == rh.get(j, j+ans+1):
                ans += 1
    print(ans)


if __name__ == '__main__':
    _solve()
