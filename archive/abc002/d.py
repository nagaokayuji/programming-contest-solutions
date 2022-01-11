import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, M = mi()
    AB = [tuple(mi1()) for _ in range(M)]
    g = [[] for _ in range(N)]
    for a, b in AB:
        g[a].append(b)
        g[b].append(a)

    ans = 0
    for bits in range(1 << N):
        members = []
        for i in range(N):
            if bits >> i & 1:
                members.append(i)
        ok = True
        for a in members:
            for b in members:
                if a == b:
                    continue
                if b not in g[a]:
                    ok = False

        if ok:
            ans = max(ans, len(members))
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
