// clang-format off
#include <bits/stdc++.h>
using namespace std;
#ifdef __LOCAL
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define dbg2(x,y) cerr<<#x<<" = "<<(x) <<", "<<#y<<" = "<<(y) <<" (L"<<__LINE__<< ")\n";
#else
#define dbg(x)
#define dbg2(x,y)
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
  int N, M;
  cin >> N >> M;
  V<V<Int>> g(N);

  rep(i, M) {
    int a, b;
    cin >> a >> b;
    --a, --b;
    g[a].push_back(b);
    g[b].push_back(a);
  }

  int ans = 0;
  rep(v, N) {
    int smallerc = 0;
    each(nv, g[v]) {
      if (v > nv) smallerc++;
    }
    if (smallerc == 1) ans++;
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