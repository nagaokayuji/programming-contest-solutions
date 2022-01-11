import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    N = int(input())
    AB = [tuple(mi()) for _ in range(N)]
    vals = set()
    for a, b in AB:
        vals.add(a)
        vals.add(b)

    vals = sorted(vals)
    ans = INF
    for s in vals:
        for t in vals:
            if s > t:
                continue
            sum = 0
            for a, b in AB:
                if a > b:
                    a, b = b, a

                # s -> a -> b -> t
                r1 = abs(a-s) + abs(b-a) + abs(t-b)
                # s -> b -> a -> t
                r2 = abs(b-s) + abs(a-b) + abs(t-a)
                sum += min(r1, r2)

            ans = min(ans, sum)
    print(ans)


if __name__ == '__main__':
    _solve()
