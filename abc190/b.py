n,s,d = map(int,input().split())
ans=False
for _ in range(n):
  x,y = map(int,input().split())
  if x<s and y>d:
    ans=True
    break

if ans:
  print("Yes")
else:
  print("No")