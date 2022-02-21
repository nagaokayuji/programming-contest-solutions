import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    N, = li()
    A = li()[:min(12, N)]
    N = len(A)
    for i in range(N):
        A[i] %= MOD

    ls = []
    for b in range(1, 1 << N):
        l = []
        _sum = 0
        for i in range(N):
            if b >> i & 1:
                l.append(i)
                _sum += A[i]
        ls.append((_sum % MOD, l[:]))

    ls.sort()
    prv_s = -1
    prv_l = []
    for s, l in ls:
        if prv_s == s:
            yn(1)
            print(len(prv_l), *map(lambda x: x+1, prv_l))
            print(len(l), *map(lambda x: x+1, l))
            return
        prv_s = s
        prv_l = l[:]

    yn(0)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 200
    わーい()
