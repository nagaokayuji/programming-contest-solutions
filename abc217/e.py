from collections import deque
import heapq

Q = int(input())
heap = []
q = deque()

for _ in range(Q):
    query = tuple(map(int, input().split()))
    if query[0] == 1:
        q.append(query[1])
    elif query[0] == 2:
        if heap:
            print(heapq.heappop(heap))
        else:
            print(q.popleft())
    else:
        while q:
            heapq.heappush(heap, q.pop())
