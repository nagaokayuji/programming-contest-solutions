import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def mi1(): return map(lambda x: int(x)-1, input().split())


def _solve():
    R, C = mi()
    A = [list(mi()) for _ in range(R)]
    ans = 0
    all1 = (1 << R)-1
    used = [False]*(1 << R+2)
    for rbit in range(1 << R):
        if used[rbit]:
            continue
        used[rbit] = used[all1 ^ rbit] = True

        sum = 0
        for j in range(C):
            cnt_col = 0
            for row in range(R):
                if rbit >> row & 1:
                    cnt_col += (A[row][j] ^ 1)
                else:
                    cnt_col += A[row][j]
            sum += max(cnt_col, R-cnt_col)
        ans = max(ans, sum)
    print(ans)


if __name__ == '__main__':
    _solve()
