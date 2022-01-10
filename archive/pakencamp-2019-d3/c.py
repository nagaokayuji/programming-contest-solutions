import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, ().split())


def _solve():
    N, M = mi()
    A = [list(mi()) for _ in range(N)]
    ans = 0
    for t1 in range(M):
        for t2 in range(t1+1, M):
            sum = 0
            for student in A:
                sum += max(student[t1], student[t2])
            ans = max(ans, sum)
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    _solve()
