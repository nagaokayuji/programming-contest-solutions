from pprint import pprint
import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    H, W, K = readmi()
    s = [input() for _ in range(H)]
    field = [[None]*W for _ in range(H)]
    indk = 1
    prvi = [-1]
    for i in range(H):
        rowc = s[i].count("#")
        if rowc:
            prvi.append(i)
            for j in range(W):
                for k in range(i, prvi[-2], -1):
                    field[k][j] = indk
                if s[i][j] == '#' and rowc > 1:
                    rowc -= 1
                    indk += 1
            indk += 1
        else:
            if indk > 1:
                field[i] = field[i-1].copy()

    for row in field:
        print(*row)


if __name__ == '__main__':
    _solve()
