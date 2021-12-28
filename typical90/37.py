import sys
INF = 10**14
def input(): return sys.stdin.readline().rstrip()


def mi(): return map(int, input().split())
def li(): return list(mi())
def ti(): return tuple(mi())


class SegmentTree:
    def __init__(self,
                 n: int,
                 initial: any):
        m = 1
        while m <= (n+1):
            m *= 2

        self.m = m
        self.n = n
        self.array = [initial] * (2*m)
        self.initial = initial

    def update(self, index: int, val: any):
        pos = self.m + index
        self.array[pos] = val  # update
        while pos > 1:  # update forward root
            pos >>= 1
            self.array[pos] = max(self.array[pos*2], self.array[pos*2+1])

    def add(self, index: int, val: any):
        self.update(index, self.array[self.m+index]+val)

    def get(self, l: int, r: int):
        '''
        get value of [l,r)
        '''
        ret = self.initial
        l += self.m
        r += self.m
        while l < r:
            if l & 1:  # その要素を使う
                ret = max(ret, self.array[l])
                l += 1
            l >>= 1
            if r & 1:
                ret = max(ret, self.array[r-1])
                r -= 1
            r >>= 1
        return ret


W, N = mi()
LRV = [ti() for _ in range(N)]

seg = SegmentTree(n=(W+5), initial=-INF)


dp = [[-INF] * (W+1) for _ in range(N+1)]

dp[0][0] = 0
seg.update(0, 0)
for i, (l, r, v) in enumerate(LRV):

    for weight in range(W+1):
        dp[i+1][weight] = dp[i][weight]
        ll = max(0, weight-r)
        rr = max(0, weight-l+1)
        if ll >= rr:
            continue
        vv = max(seg.get(ll, rr)+v, dp[i+1][weight])
        dp[i+1][weight] = vv
    for j in range(W+1):
        seg.update(j, dp[i+1][j])

if dp[N][W] > 0:
    print(dp[N][W])
else:
    print(-1)
