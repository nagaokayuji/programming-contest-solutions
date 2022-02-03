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
// template<class T>using V=vector<T>;
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
  Int N, Q;
  cin >> N >> Q;

  vector<int> T(Q), X(Q), Y(Q), V(Q);
  vector<Int> s(N + 1);
  rep(i, Q) {
    cin >> T[i] >> X[i] >> Y[i] >> V[i];
    if (T[i] == 0) {
      s[X[i]] = V[i];
    }
  }
  vector<Int> s2(N + 2);
  rep(i, N + 1) { s2[i + 1] = s[i] - s2[i]; }

  atcoder::dsu dsu(N + 1);
  rep(i, Q) {
    auto [t, x, y, v] = tie(T[i], X[i], Y[i], V[i]);
    auto xx = s2[x], yy = s2[y];
    if (t == 0) {
      dsu.merge(x, y);
    } else {
      if (not dsu.same(x, y)) {
        cout << "Ambiguous" << endl;
        continue;
      }
      if ((x + y) % 2 == 0) {
        cout << v + yy - xx << endl;
      } else {
        cout << xx + yy - v << endl;
      }
    }
  }
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}