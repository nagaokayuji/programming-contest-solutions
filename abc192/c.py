from collections import defaultdict
def mi(): return map(int, input().split())


N, K = mi()


def f(a):
    l = []
    while a:
        l.append(a % 10)
        a //= 10
    l.sort()
    g1 = 0
    for x in l:
        g1 *= 10
        g1 += x
    g2 = 0
    for x in l[::-1]:
        g2 *= 10
        g2 += x
    return g2-g1


for _ in range(K):
    N = f(N)
print(N)
