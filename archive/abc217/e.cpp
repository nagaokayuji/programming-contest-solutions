#include <bits/stdc++.h>

#include <atcoder/all>
#define rep(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define rrep(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define debug(x) cerr << #x << " = " << x << "\n"
#define debugv(x) \
    rep(f, 0, (x.size() - 1)) cerr << x[f] << (f == (x.size() - 1) ? "\n" : " ")
#define debug2(x, y)                         \
    cerr << "(" << #x << "," << #y << ") = " \
         << "(" << x << "," << y << ")\n"
#define all(x) x.begin(), x.end()
#define each(a, x) for (auto &a : (x))
#define chmin(x, y) x = min(x, y)
#define chmax(x, y) x = max(x, y)
#define yn(x) puts((x ? "Yes" : "No"))
using namespace std;
using pii = pair<int, int>;
using vi = vector<int>;

void solve()
{
    int Q;
    int INF = 1000000005;
    cin >> Q;
    deque<int> q;
    int c, x;
    bool sorted = false;
    int notv = INF;
    deque<int> q2;
    priority_queue<int, vector<int>, greater<int>> pq;
    priority_queue<int, vector<int>, greater<int>> kakutei;
    bool lazy = false;
    rep(_, 1, Q)
    {
        cin >> c;
        if (c == 1)
        {
            cin >> x;
            q.push_back(x);
            pq.push(x);
            notv = min(notv, x);
        }
        else if (c == 2)
        {
            if (!kakutei.empty())
            {
                cout << kakutei.top() << endl;
                kakutei.pop();
            }
            else
            {
                cout << *q.begin() << endl;
                q.pop_front();
            }
        }
        else
        {
            while (!q.empty())
            {
                kakutei.push(q.front());
                q.pop_front();
            }
            // q.clear();
        }
    }
}
signed main()
{
    cout << setprecision(12);
    solve();
    return 0;
}