n = int(input())
a = list(map(int, input().split()))

l = 0
r = 0
ans = 0
st = set()
while l < n:
    if l > r:
        r = l
    while r < n and a[r] not in st:
        st.add(a[r])
        ans = max(ans, len(st))
        r += 1
    ans = max(ans, len(st))
    st.remove(a[l])
    l += 1
print(ans)
