def solve(n, x):
    # [1,2,3,4,...,n-1,n]
    # 3 つ
    ans = 0
    for a in range(1, n+1):
        for b in range(a+1, n+1):
            c = x-a-b
            if b < c <= n:
                ans += 1

    print(ans)


while True:
    n, x = map(int, input().split())
    if n == x == 0:
        break
    solve(n, x)
