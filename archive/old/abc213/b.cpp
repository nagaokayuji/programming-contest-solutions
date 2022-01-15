#include <bits/stdc++.h>

#define rep(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define rrep(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define debug(x) cerr << #x << " = " << x << "\n"
#define debugv(x) \
  rep(f, 0, (x.size() - 1)) cerr << x[f] << (f == (x.size() - 1) ? "\n" : " ")
#define debug2(x, y)                       \
  cerr << "(" << #x << "," << #y << ") = " \
       << "(" << x << "," << y << ")\n"
#define all(x) x.begin(), x.end()
#define each(a, x) for (auto &a : (x))
#define chmin(x, y) x = min(x, y)
#define chmax(x, y) x = max(x, y)
#define int long long
#define yn(x) puts((x ? "Yes" : "No"))
using namespace std;
using pii = pair<int, int>;
using vi = vector<int>;
const int MX = 1e5 + 5, INF = 5LL << 57, MOD = 1e9 + 7;

void solve()
{
  int N;
  cin >> N;
  vector<pii> A(N);
  rep(i, 0, N - 1)
  {
    int x;
    cin >> x;
    A[i] = make_pair(x, i + 1);
  }
  sort(all(A));
  cout << (A[N - 2].second) << endl;
}
signed main()
{
  cout << setprecision(12);
  solve();
  return 0;
}