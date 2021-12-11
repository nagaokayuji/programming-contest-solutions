import sys
def input(): return sys.stdin.readline().rstrip()


N = int(input())


sqrtN = int(N**0.5)


ans = 0
for i in range(1, sqrtN+1):
    ans += i * ((N//i) - (N//(i+1)))

for i in range(1, (N//(sqrtN+1))+1):
    ans += N//i

print(ans)
