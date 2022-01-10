import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    N = int(input())
    S = list(input())

    ans = 0
    for pin in range(1000):
        target = str(pin).zfill(3)
        matchc = 0
        for x in S:
            if x == target[matchc]:
                matchc += 1
            if matchc > 2:
                break
        if matchc == 3:
            ans += 1
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
