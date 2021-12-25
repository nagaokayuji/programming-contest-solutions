from pprint import pprint
import sys
def input(): return sys.stdin.readline().rstrip()
def mi(): return map(int, input().split())
def li(): return list(mi())


N = int(input())
S = [input() for _ in range(N)]


st = set()
for i, s in enumerate(S):
    if s not in st:
        st.add(s)
        print(i+1)
