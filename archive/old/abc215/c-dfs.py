s, k = input().split()
k = int(k)

st = set()


def dfs(s, left):
    if not left:
        st.add(s)
        return
    for i, left_c in enumerate(left):
        dfs(s + left_c, left[:i]+left[i+1:])


dfs("", s)

print(sorted(st)[k-1])
