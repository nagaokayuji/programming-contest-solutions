import sys
from collections import defaultdict, Counter, deque
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    N, M = mi()
    A = list(mi())
    BC = [tuple(mi()) for _ in range(M)]
    counts = Counter(A)
    q = []
    for k, v in counts.items():
        heappush(q, (-k, v))

    for b, c in BC:
        heappush(q, (-c, b))

    ans = 0
    count = 0
    while count < N:
        val, c = heappop(q)
        ad = min(c, N-count)
        count += ad
        ans += -ad*val
    print(ans)


if __name__ == '__main__':
    _solve()
