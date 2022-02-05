import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def _solve():
    N, A, B, C, D = li()
    A -= 1
    B -= 1
    C -= 1
    D -= 1
    S = input() + "#"
    assert(S[A] == '.')
    assert(S[B] == '.')
    assert(S[C] == '.')
    assert(S[D] == '.')

    if C < D:
        ok = True
        for i in range(A, C):
            if S[i] == '#' and S[i+1] == '#':
                ok = False
        for i in range(B, D):
            if S[i] == '#' and S[i+1] == '#':
                ok = False
        yn(ok)
        return

    ok = True
    for i in range(A, C):
        if S[i] == '#' and S[i+1] == '#':
            ok = False
    for i in range(B, D):
        if S[i] == '#' and S[i+1] == '#':
            ok = False

    ok2 = False
    for i in range(B, D+1):
        if S[i-1] == '.' and S[i] == '.' and S[i+1] == '.':
            ok2 = True

    yn(ok and ok2)


if __name__ == '__main__':
    _solve()
