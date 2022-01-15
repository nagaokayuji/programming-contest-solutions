import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def inputmi(): return map(int, input().split())
def inputmi1(): return map(lambda x: int(x)-1, input().split())
def inputli(): return list(inputmi())
def inputli1(): return list(inputmi1())
def inputti(): return tuple(inputmi())
def inputti1(): return tuple(inputmi1())
def inputi(): return int(input())


class DSU:
    def __init__(self, n: int = 0):
        self._n = n
        self.parent_or_size = [-1] * n

    def merge(self, a: int, b: int):
        x = self.leader(a)
        y = self.leader(b)

        if x == y:
            return x

        if -self.parent_or_size[x] < -self.parent_or_size[y]:
            x, y = y, x

        self.parent_or_size[x] += self.parent_or_size[y]
        self.parent_or_size[y] = x
        return x

    def same(self, a: int, b: int):
        return self.leader(a) == self.leader(b)

    def leader(self, a: int):
        parent = self.parent_or_size[a]
        while parent >= 0:
            if self.parent_or_size[parent] < 0:
                return parent
            self.parent_or_size[a], a, parent = (
                self.parent_or_size[parent],
                self.parent_or_size[parent],
                self.parent_or_size[self.parent_or_size[parent]]
            )

        return a

    def size(self, a: int):
        return -self.parent_or_size[self.leader(a)]

    def groups(self):
        leader_buf = [self.leader(i) for i in range(self._n)]
        result = [[] for _ in range(self._n)]
        for i in range(self._n):
            result[leader_buf[i]].append(i)
        return list(filter(lambda r: r, result))


def _solve():
    H, W = inputti()

    def enc(h, w):
        return h*W + w

    def dec(i):
        return i//W, i % W

    S = [input() for _ in range(H)]
    dsu = DSU(H*W)

    dx_ = [-1, 0, 0, 1]
    dy_ = [0, -1, 1, 0]
    for i in range(H):
        for j in range(W):
            for dx, dy in zip(dx_, dy_):
                ni = i+dx
                nj = j+dy
                if 0 <= ni < H and 0 <= nj < W:
                    if S[ni][nj] != S[i][j]:
                        dsu.merge(enc(i, j), enc(ni, nj))

    ans = 0
    for group in dsu.groups():
        bc, wc = 0, 0
        for x in group:
            i, j = dec(x)
            if S[i][j] == '.':
                wc += 1
            else:
                bc += 1
        ans += bc*wc
    print(ans)


if __name__ == '__main__':
    _solve()
