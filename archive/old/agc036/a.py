def _solve():
    S = int(input())
    height = width = 10**9
    if S <= 10**9:
        height = S
        width = 1
        print(0, 0, 0, height, width, 0)
    else:
        # y * 10**9 - x = S
        #
        y = (S+10**9 - 1)//(10**9)
        x = 10**9 * y - S
        print(0, 0, 10**9, 1, x, y)
        assert y * 10**9 - x == S


if __name__ == '__main__':
    _solve()
