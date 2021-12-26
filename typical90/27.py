N = int(input())
S = [input() for _ in range(N)]

st = set()
for i, s in enumerate(S):
    if s not in st:
        st.add(s)
        print(i+1)
