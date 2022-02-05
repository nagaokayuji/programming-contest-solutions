N = int(input())


def dfs(s, mx):
    if (len(s) == N):
        print(s)
        return
    for i in range(mx+2):
        dfs(s + chr(ord('a')+i), max(mx, i))


dfs("a", 0)
