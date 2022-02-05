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
void _solve() {
  int N, K;
  cin >> N >> K;
  V<Int> A(N);
  cin >> A;

  // corner
  if (N == 1) {
    Int a = A[0];
    if (a < K) {
      cout << 0 << endl;
      return;
    }
    cout << comb(a - 1, K - 1) << endl;
    return;
  }
  // 無理
  if (A[0] < K || (A[0] < (accumulate(A.begin() + 1, A.end(), 0LL) + K))) {
    cout << 0 << endl;
    return;
  }

  // 他の場合は1通り以上ある
  dbg("some");
  Int others = accumulate(A.begin() + 1, A.end(), 0LL);
  Int one = A[0];

  mint base = comb(A[0] - others, K - 1);
  mint ans = base;
  dbg(base);

  // 最後に区別する
  for (int i = 1; i < N; i++) {
    ans *= comb(others, A[i]);
    dbg(others);
    others -= A[i];
  }
  assert(others == 0LL);
  cout << ans << endl;
}

signed main() {
  init();
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}