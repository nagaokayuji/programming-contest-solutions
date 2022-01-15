import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())
def li(): return list(mi())
def li1(): return list(mi1())
def ti(): return tuple(mi())
def ti1(): return tuple(mi1())


def _solve():
    N = int(input())
    AB = [li() for _ in range(N)]
    CD = [li() for _ in range(N)]
    AB.sort()
    CD.sort()
    pairs = 0
    usedr = [False]*N

    for c, d in CD:
        mxb, ind = -1, None
        for i, (a, b) in enumerate(AB):
            if usedr[i]:
                continue
            if c < a:
                break
            if a < c and b < d and mxb < b:
                mxb, ind = b, i
        if ind is not None:
            usedr[ind] = True
            pairs += 1
    print(pairs)


if __name__ == '__main__':
    _solve()
