// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define bit(n,m)(((n)>>(m))&1)
#define rep(i,n)for(int i=0;i<(int)n;i++)
#define all(v) v.begin(), v.end()
#define each(x, A) for(auto&& x :A)
template<class T>inline bool chmin(T &a,const T &b) {bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b) {bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const pair<T1, T2>& pair){stream<<"("<<pair.first<<", "<<pair.second<<")";return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
template<class T>using V=vector<T>;
using Int=long long;
using i128=__int128_t;
const Int INF = 5LL<<58;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void _solve() {
  int N;
  cin >> N;
  V<V<int>> A(N, V<int>(N));
  each(x, A) cin >> x;
  int M;
  cin >> M;
  V<V<bool>> bad(N, V<bool>(N));
  rep(i, M) {
    int x, y;
    cin >> x >> y;
    x--;
    y--;
    bad[x][y] = bad[y][x] = true;
  }
  V<int> r;
  for (int i = 0; i < N; i++) r.push_back(i);
  int ans = 1e9;

  do {
    bool ok = true;
    int sum = 0;
    rep(i, N - 1) if (bad[r[i]][r[i + 1]]) ok = false;
    if (!ok) continue;
    rep(i, N) sum += A[r[i]][i];
    chmin(ans, sum);
  } while (next_permutation(all(r)));
  if (ans == 1e9)
    cout << -1 << endl;
  else
    cout << ans << endl;
}
signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}