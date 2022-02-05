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
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void _solve() {
  int N, S;
  cin >> N >> S;
  V<V<bool>> dp(N + 1, V<bool>(S + 1));
  dp[0][0] = true;
  V<pair<int, int>> ab;
  rep(i, N) {
    int a, b;
    cin >> a >> b;
    ab.emplace_back(a, b);
    rep(s, S + 1) {
      if (s + a <= S) dp[i + 1][s + a] = dp[i + 1][s + a] || dp[i][s];
      if (s + b <= S) dp[i + 1][s + b] = dp[i + 1][s + b] || dp[i][s];
    }
  }
  if (!dp[N][S]) {
    puts("Impossible");
    return;
  }

  auto now = S;
  string ret = "";
  for (int i = N - 1; i >= 0; i--) {
    dbg(now);
    int a = ab[i].first, b = ab[i].second;
    if (now >= a && dp[i][now - a]) {
      now -= a;
      ret += "A";
      continue;
    } else {
      assert(dp[i][now - b]);
      now -= b;
      ret += "B";
    }
  }
  reverse(all(ret));
  cout << ret << endl;
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}