import sys
def input(): return sys.stdin.readline().rstrip()
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


def _solve():
    N, K = inputti()
    A = inputli()

    if K == 1:
        print(sum(A))
        return

    ok = 0
    ng = 10**18
    while ng-ok > 1:
        mid = (ok+ng)//2
        s = sum(map(lambda x: min(mid, x), A))
        if s >= K*mid:
            ok = mid
        else:
            ng = mid
    print(ok)


if __name__ == '__main__':
    _solve()
