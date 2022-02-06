// clang-format off
#include <bits/stdc++.h>
using namespace std;
#ifdef __LOCAL
#define dbg(x)cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define dbg2(x,y)cerr<<#x<<" = "<<(x)<<", "<<#y<<" = "<<(y) <<" (L"<<__LINE__<<")\n";
#else
#define dbg(x)
#define dbg2(x,y)
#endif
#define bit(n,m)(((n)>>(m))&1)
#define rep(i,n)for(int i=0;i<(int)n;i++)
#define all(v) v.begin(),v.end()
#define each(x,A) for(auto&& x:A)
template<class T>inline bool chmin(T &a,const T &b){bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b){bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const map<T1,T2>& map){for(auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const pair<T1,T2>& pair){stream<<"("<<pair.first<<", "<<pair.second<<")";return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os, const array<T,sz> &arr){os << '[';for(auto v:arr)os<<v<<", ";os <<']';return os;}
template<class T>using V=vector<T>;
void yn(bool ans){cout<<(ans?"Yes":"No")<<"\n";}
using Int=long long;
using i128=__int128_t;
const Int LINF=5LL<<58;
const int INF=5<<28;
// === ACL ====
#include <atcoder/all>
using mint=atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on
const int MX = 1e6 + 5;
mint fact[MX + 1];
mint ifact[MX + 1];
void init() {
  fact[0] = fact[1] = ifact[1] = ifact[0] = 1;
  for (int i = 2; i <= MX; ++i) {
    fact[i] = fact[i - 1] * i;
  }
  ifact[MX] = fact[MX].inv();
  for (int i = MX - 1; i > 0; --i) {
    ifact[i] = ifact[i + 1] * (i + 1);
  }
}
mint comb(int n, int k) {
  assert(n >= k);
  return fact[n] * ifact[k] * ifact[n - k];
}
struct DP {
  mint dp;
  int t;
  DP(mint dp = 1, int t = 0) : dp(dp), t(t) {}

  DP &operator+=(const DP &a) {
    dp *= a.dp;
    dp *= comb(t + a.t, t);
    t += a.t;
    return *this;
  }
  DP operator-(const DP &a) {
    DP res(*this);
    res.t -= a.t;
    res.dp /= comb(res.t + a.t, res.t);
    res.dp /= a.dp;
    return res;
  }

  DP addRoot() {
    DP res(*this);
    res.t++;
    return res;
  }
};
void _solve() {
  // live
  Int N;
  cin >> N;
  V<V<Int>> to(N);
  rep(i, N - 1) {
    int a, b;
    cin >> a >> b;
    --a, --b;
    to[a].push_back(b), to[b].push_back(a);
  }

  V<DP> dp(N);
  auto dfs = [&](auto self, int v, int prev = -1) -> void {
    each(u, to[v]) {
      if (u == prev) continue;
      self(self, u, v);
      dp[v] += dp[u].addRoot();
    }
  };
  auto bfs = [&](auto self, int v, int prev = -1) -> void {
    each(u, to[v]) {
      if (u == prev) continue;
      DP d = dp[v] - dp[u].addRoot();
      dp[u] += d.addRoot();
      self(self, u, v);
    }
  };

  dfs(dfs, 0);
  bfs(bfs, 0);
  each(v, dp) cout << v.dp << endl;
}

signed main() {
  init();
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}