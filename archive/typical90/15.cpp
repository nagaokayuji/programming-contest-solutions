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

const int MX = 1e5 + 5;
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

void _solve() {
  init();
  int N;
  cin >> N;

  for (int k = 1; k <= N; k++) {
    mint ans = 0;
    for (int a = 1; a <= N; a++) {
      if (N - (k - 1) * (a - 1) < a) break;
      ans += comb(N - (k - 1) * (a - 1), a);
    }
    cout << ans << endl;
  }
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}