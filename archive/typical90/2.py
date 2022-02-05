N = int(input())


def dfs(s, c):
    if c < 0:
        return
    if len(s) == N and not c:
        print(s)
        return
    if len(s) > N:
        return
    dfs(s+"(", c+1)
    dfs(s+")", c-1)


dfs("", 0)
