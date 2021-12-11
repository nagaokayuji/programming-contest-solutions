from collections import defaultdict
def mi(): return map(int, input().split())
def li(): return list(mi())


X = int(input())
M = int(input())
d = int(max(str(X)))


def conv(n, m):
    ret = 0
    l = []
    while n:
        l.append(n % 10)
        n //= 10
    base = 1
    for x in l:
        ret += x * base
        base *= m
    return ret


if len(str(X)) == 1:
    if conv(X, d+1) <= M:
        print(1)
    else:
        print(0)
    exit()

ok = 0
ng = 10**20
while ng-ok > 1:
    mid = (ok+ng)//2
    if conv(X, mid) <= M:
        ok = mid
    else:
        ng = mid


if conv(X, d+1) > M:
    print(0)
    exit()
else:
    print(ok-d)
