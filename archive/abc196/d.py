import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def solve():
    H, W, A, B = readmi()

    def dfs(i, bit, A, B):
        if i == H * W:
            return 1
        ret = 0
        if bit >> i & 1:
            ret += dfs(i + 1, bit, A, B)
        if B:
            ret += dfs(i + 1, bit | 1 << i, A, B - 1)
        if A:
            if i % W != W - 1 and not bit & 1 << (i + 1):
                ret += dfs(i + 1, bit | 1 << i | 1 << (i + 1), A - 1, B)
            if i + W < H * W:
                ret += dfs(i + 1, bit | 1 << i | 1 << (i + W), A - 1, B)
        return ret

    print(dfs(0, 0, A, B))


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    solve()
