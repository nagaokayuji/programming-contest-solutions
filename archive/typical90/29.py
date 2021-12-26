from pprint import pprint
import sys
from heapq import heappush, heappop, heapify, heappushpop, heapreplace
# from numba import njit, b1, i1, i4, i8, f8
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


class LazySegTree:
    import typing

    def __init__(
            self,
            op: typing.Callable[[typing.Any, typing.Any], typing.Any],
            e: typing.Any,
            mapping: typing.Callable[[typing.Any, typing.Any], typing.Any],
            composition: typing.Callable[[typing.Any, typing.Any], typing.Any],
            id: typing.Any,
            v: typing.Union[int, typing.List[typing.Any]]) -> None:
        self._op = op
        self._e = e
        self._mapping = mapping
        self._composition = composition
        self._id = id

        if isinstance(v, int):
            v = [e] * v

        self._n = len(v)
        self._log = self._ceil_pow2(self._n)
        self._size = 1 << self._log
        self._array = [e] * (self._size << 1)
        self._lz = [self._id] * self._size
        for i in range(self._n):
            self._array[self._size + i] = v[i]
        for i in range(self._size - 1, 0, -1):
            self._update(i)

    def _ceil_pow2(n: int):
        x = 0
        while (1 << x) < n:
            x += 1
        return x

    def set(self, p: int, x: typing.Any) -> None:
        assert 0 <= p < self._n

        p += self._size
        for i in range(self._log, 0, -1):
            self._push(p >> i)
        self._array[p] = x
        for i in range(1, self._log + 1):
            self._update(p >> i)

    def get(self, p: int) -> typing.Any:
        assert 0 <= p < self._n

        p += self._size
        for i in range(self._log, 0, -1):
            self._push(p >> i)
        return self._array[p]

    def prod(self, left: int, right: int) -> typing.Any:
        if left == right:
            return self._e

        left += self._size
        right += self._size

        for i in range(self._log, 0, -1):
            if ((left >> i) << i) != left:
                self._push(left >> i)
            if ((right >> i) << i) != right:
                self._push(right >> i)

        sml = self._e
        smr = self._e
        while left < right:
            if left & 1:
                sml = self._op(sml, self._array[left])
                left += 1
            if right & 1:
                right -= 1
                smr = self._op(self._array[right], smr)
            left >>= 1
            right >>= 1

        return self._op(sml, smr)

    def all_prod(self) -> typing.Any:
        return self._array[1]

    def apply(self, left: int, right: typing.Optional[int] = None,
              f: typing.Optional[typing.Any] = None):
        assert f is not None

        if right is None:
            p = left

            p += self._size
            for i in range(self._log, 0, -1):
                self._push(p >> i)
            self._array[p] = self._mapping(f, self._array[p])
            for i in range(1, self._log + 1):
                self._update(p >> i)
        else:
            if left == right:
                return

            left += self._size
            right += self._size

            for i in range(self._log, 0, -1):
                if ((left >> i) << i) != left:
                    self._push(left >> i)
                if ((right >> i) << i) != right:
                    self._push((right - 1) >> i)

            l2 = left
            r2 = right
            while left < right:
                if left & 1:
                    self._all_apply(left, f)
                    left += 1
                if right & 1:
                    right -= 1
                    self._all_apply(right, f)
                left >>= 1
                right >>= 1
            left = l2
            right = r2

            for i in range(1, self._log + 1):
                if ((left >> i) << i) != left:
                    self._update(left >> i)
                if ((right >> i) << i) != right:
                    self._update((right - 1) >> i)

    def max_right(
            self, left: int, g: typing.Callable[[typing.Any], bool]) -> int:
        assert 0 <= left <= self._n
        assert g(self._e)

        if left == self._n:
            return self._n

        left += self._size
        for i in range(self._log, 0, -1):
            self._push(left >> i)

        sm = self._e
        first = True
        while first or (left & -left) != left:
            first = False
            while left % 2 == 0:
                left >>= 1
            if not g(self._op(sm, self._array[left])):
                while left < self._size:
                    self._push(left)
                    left *= 2
                    if g(self._op(sm, self._array[left])):
                        sm = self._op(sm, self._array[left])
                        left += 1
                return left - self._size
            sm = self._op(sm, self._array[left])
            left += 1

        return self._n

    def min_left(self, right: int, g: typing.Any) -> int:
        assert 0 <= right <= self._n
        assert g(self._e)

        if right == 0:
            return 0

        right += self._size
        for i in range(self._log, 0, -1):
            self._push((right - 1) >> i)

        sm = self._e
        first = True
        while first or (right & -right) != right:
            first = False
            right -= 1
            while right > 1 and right % 2:
                right >>= 1
            if not g(self._op(self._array[right], sm)):
                while right < self._size:
                    self._push(right)
                    right = 2 * right + 1
                    if g(self._op(self._array[right], sm)):
                        sm = self._op(self._array[right], sm)
                        right -= 1
                return right + 1 - self._size
            sm = self._op(self._array[right], sm)

        return 0

    def _update(self, k: int):
        self._array[k] = self._op(self._array[2*k], self._array[2*k+1])

    def _all_apply(self, k: int, f: typing.Any) -> None:
        self._array[k] = self._mapping(f, self._array[k])
        if k < self._size:
            self._lz[k] = self._composition(f, self._lz[k])

    def _push(self, k: int) -> None:
        self._all_apply(2 * k, self._lz[k])
        self._all_apply(2 * k + 1, self._lz[k])
        self._lz[k] = self._id


W, N = mi()

INF = 10**19
seg = LazySegTree(
    op=lambda a, b: max(a, b),
    e=0,
    mapping=lambda f, x: x if f == INF else f,
    composition=lambda f, g: g if f == INF else f,
    id=INF,
    v=W+1
)

ans = []
for l, r in [tuple(mi()) for _ in range(N)]:
    v = seg.prod(l-1, r) + 1
    ans.append(v)
    seg.apply(l-1, r, v)
print(*ans)
