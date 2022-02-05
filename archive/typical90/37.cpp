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
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on
using S = Int;
using F = Int;
const Int INF = 5LL << 58;
const F ID = INF;
S op(S a, S b) { return std::max(a, b); }
S e() { return -INF; }
S mapping(F f, S x) { return (f == ID ? x : max(x, f)); }
F composition(F f, F g) { return (f == ID ? g : f); }
F id() { return ID; }

void _solve() {
  int W, N;
  cin >> W >> N;

  // auto dp = V<Int>(W + 1, -INF);
  // dp[0] = 0;
  atcoder::lazy_segtree<S, op, e, F, mapping, composition, id> dp(W + 7);
  dp.set(0, 0);

  rep(_, N) {
    int l, r, v;
    cin >> l >> r >> v;
    auto ndp = dp;
    for (int w = 0; w <= W; w++) {
      auto wv = dp.get(w);
      if ((w + l <= W) && wv >= 0) {
        ndp.apply(min(w + l, W), min(w + r + 1, W + 1), wv + v);
      }
    }
    swap(dp, ndp);
  }

  auto ans = dp.get(W);
  std::cout << (ans >= 0 ? ans : -1) << endl;
}

signed main() {
  std::cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}