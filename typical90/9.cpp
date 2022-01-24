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
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void _solve() {
  int N;
  double PI = acos(-1);
  cin >> N;
  V<pair<Int, Int>> XY(N);
  each(xy, XY) cin >> xy.first >> xy.second;
  // dbg(XY);

  double ans = 0.0;
  rep(i, N) {
    auto mid = XY[i];
    V<double> args;  // 度数法
    rep(j, N) {
      if (i == j) continue;
      auto t = XY[j];
      auto dif = make_pair(t.first - mid.first, t.second - mid.second);
      double arg = atan2(dif.second, dif.first);  // rad
      args.push_back(arg * 180.0 / PI);
      args.push_back(360.0 + args.back());
    }
    sort(all(args));
    // argsの差が180に近いやつを探したい
    each(a, args) {
      auto it = lower_bound(all(args), a + 180.0);
      if (it != args.end()) {
        double dif = *it - a;
        if (dif > 180.0) dif = 360.0 - dif;
        chmax(ans, dif);
      }
      {
        double dif = *(it - 1) - a;
        if (dif > 180.0) dif = 360.0 - dif;
        chmax(ans, dif);
      }
    }
  }
  // output
  cout << ans << endl;
}

signed main() {
  cout << setprecision(15);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}