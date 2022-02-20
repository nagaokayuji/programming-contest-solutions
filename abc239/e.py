import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))


def _solve():
    N, Q = li()
    X = li()
    AB = [tuple(li1()) for _ in range(N-1)]
    g = [[] for _ in range(N)]
    for a, b in AB:
        g[a].append(b)
        g[b].append(a)

    VK = [tuple(li1()) for _ in range(Q)]

    dp = [[-1] * 21 for _ in range(N)]

    def dfs(v=0, prev=-1):
        l = [X[v]]
        for to in g[v]:
            if prev == to:
                continue
            dfs(to, v)
            for x in dp[to]:
                if x == -1:
                    break
                l.append(x)
        l.sort(reverse=True)
        for i in range(min(21, len(l))):
            dp[v][i] = l[i]

    dfs()
    for v, k in VK:
        print(dp[v][k])


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
