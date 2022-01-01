import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, M = readmi()
    g = [[] for _ in range(N)]
    for _ in range(M):
        a, b = readmi()
        a -= 1
        b -= 1
        g[a].append(b)
        g[b].append(a)

    def dfs(now, visited):
        visited_now = visited + [now]
        if len(visited_now) == N:
            return 1
        ret = 0
        for nx in g[now]:
            if nx in visited_now:
                continue
            ret += dfs(nx, visited_now.copy())
        return ret
    print(dfs(0, []))

    return


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    _solve()
