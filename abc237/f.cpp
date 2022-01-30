// clang-format off
#include <bits/stdc++.h>
using namespace std;
#ifdef __LOCAL
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#else
#define dbg(x)
#endif
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
const Int LINF = 5LL<<58;
const int INF=5<<28;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint998244353;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void _solve() {
  int N, M;
  cin >> N >> M;

  V<V<V<mint>>> dp(N + 1, V<V<mint>>(11, V<mint>(5)));
  // dp[0][1][0] = 1;
  // dp[1]
  for (int i = 1; i <= M; i++) {
    dp[1][i][1] = 1;
  }

  // rep(i, N) {
  for (int i = 1; i < N; i++) {
    for (int j = 1; j <= M; j++) {
      rep(k, 4) {
        for (int nxtj = 1; nxtj <= j; nxtj++) {
          dp[i + 1][j][k] += dp[i][j][k];
        }

        for (int nxtj = j + 1; nxtj <= M; nxtj++) {
          dp[i + 1][nxtj][k + 1] += dp[i][j][k];
        }
      }
    }
  }
  dbg(dp);
  mint ans = 0;
  for (int i = 1; i <= M; i++) {
    ans += dp[N][i][3];
  }
  cout << ans << endl;
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}