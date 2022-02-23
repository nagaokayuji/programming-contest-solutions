import sys
def input(): return sys.stdin.readline().rstrip()
def li(): return list(map(int, input().split()))
def li1(): return list(map(lambda x: int(x)-1, input().split()))
def yn(a): print("Yes" if a else "No")


def わーい():
    N, = li()
    S = list(input())
    T = list(input())

    sc = S.count('1')
    tc = T.count('1')
    if tc > sc or (sc % 2 != tc % 2):
        print(-1)
        return

    s_ones = []
    for i, c in enumerate(S):
        if c == '1':
            s_ones.append(i)
    s_ones = s_ones[::-1]

    ans = 0
    i = 0
    while i < N:
        while s_ones and s_ones[-1] <= i:
            s_ones.pop()
        if S[i] == T[i]:
            i += 1
            continue

        if not s_ones:
            print(-1)
            return
        nx_i = s_ones.pop()
        assert S[nx_i] == '1'

        if T[i] == '0':
            S[i] = '0'
            S[nx_i] = '0'
            ans += nx_i - i
        else:
            S[nx_i] = '0'
            ans += nx_i - i
        i += 1
    print(ans)


if __name__ == '__main__':
    sys.setrecursionlimit(10**8)
    MOD = 10**9+7
    わーい()
