n = int(input())


def f(x):
    return x*(x+1)//2


def g(l, r):
    return f(r)-f(l)


def isOK(length):
    ok = 0
    ng = 10**12
    while ng-ok > 1:
        mid = (ok+ng)//2
        if g(mid, mid+length) <= n:
            ok = mid
        else:
            ng = mid
    return g(ok, ok+length) == n


ans = 0
for i in range(1, n+1):
    if f(i) > n:
        break
    if isOK(i):
        ans += 2
print(ans)
