import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, K = mi()
    A = list(mi())

    ans = INF
    for bits in range(1 << N):
        if bin(bits).count("1") < K:
            continue

        mx = 0
        cost = 0
        for i, height in enumerate(A):
            if bits >> i & 1:
                if mx >= height:
                    cost += mx-height+1
                    mx += 1
                else:
                    mx = height
            else:
                mx = max(mx, height)
        ans = min(ans, cost)
    print(ans)


if __name__ == '__main__':
    _solve()
