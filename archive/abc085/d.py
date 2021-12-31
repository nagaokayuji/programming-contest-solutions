import sys
INF = float('inf')
def input(): return sys.stdin.readline().rstrip()
def readmi(): return map(int, input().split())
def readli(): return list(readmi())
def readti(): return tuple(readmi())
def readi(): return int(input())


def _solve():
    N, H = readmi()
    a = []
    b = []
    ab = [readti() for _ in range(N)]
    for _a, _b in ab:
        a.append(_a)
        b.append(_b)
    a.sort()
    b.sort(key=lambda x: -x)
    sh = a[-1]
    cnt = 0
    sm = 0
    for throw in b:
        if throw > sh and sm < H:
            cnt += 1
            sm += throw
    left = max(0, H-sm)
    cnt += (left+sh-1)//sh
    print(cnt)


if __name__ == '__main__':
    _solve()
