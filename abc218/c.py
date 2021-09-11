n = int(input())
s = []
t = []

for _ in range(n):
    a = input()
    s.append(a)

for _ in range(n):
    t.append(list(input()))


def fndu(a):
    n = len(a)
    for i in range(n):
        for j in range(n):
            if a[i][j] == '#':
                return i
    return -1


def fndl(a):
    n = len(a)
    for j in range(n):
        for i in range(n):
            if a[i][j] == '#':
                return j
    return -1


def fndd(a):
    n = len(a)
    for i in range(n-1, -1, -1):
        for j in range(n):
            if a[i][j] == '#':
                return i
    return -1


def fndr(a):
    n = len(a)
    for j in range(n-1, -1, -1):
        for i in range(n):
            if a[i][j] == '#':
                return j
    return -1


def m(s, t):
    tri = False

    sl = fndl(s)
    tl = fndl(t)
    su = fndu(s)
    tu = fndu(t)

    srow = fndr(s) - sl
    trow = fndr(t) - tl
    scol = fndd(s) - su
    tcol = fndd(t) - tu

    # print(srow, trow, scol, scol)
    # print(sl, tl)

    if sl == -1 or tl == -1:
        return False

    if trow != srow or scol != tcol:
        return False

    # print("started")
    for i in range(n):
        if i + max(su, tu) >= n:
            break
        for j in range(n):
            if j + max(sl, tl) >= n:
                break
            if s[i + su][j + sl] == '#' or t[i + tu][j + tl] == '#':
                tri = True
            if tri and s[i + su][j + sl] == '#' and t[i + tu][j + tl] != '#':
                # print("ng:", i, j)
                return False
    return True


def rot(t):
    ret = [['_'] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            ret[i][j] = t[n-j-1][i]
    return ret


for c in range(4):
    # ans = False
    # print(m(s, t))
    # ans = ans or m(s, t)
    if m(s, t):
        print("Yes")
        exit()
    t = rot(t).copy()

# if ans:
#     print("Yes")
# else:
print("No")
