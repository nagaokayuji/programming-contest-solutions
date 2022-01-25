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
  const Int INF = 1e17;
  int N;
  cin >> N;
  V<Int> A(2 * N);
  cin >> A;

  // dp?

  V<V<Int>> dp(2 * N, V<Int>(2 * N + 1, INF));
  rep(i, 2 * N) dp[i][i] = 0;

  for (int length = 2; length <= 2 * N; length += 2) {
    rep(left, 2 * N) {
      int right = left + length;
      if (right > 2 * N) break;
      chmin(dp[left][right],
            dp[left + 1][right - 1] + abs(A[left] - A[right - 1]));
      for (int mid = left + 2; mid < right; mid += 2) {
        chmin(dp[left][right], dp[left][mid] + dp[mid][right]);
      }
    }
  }
  // dbg(dp);
  // // dbg(dp[0][2 * N]);
  cout << dp[0][2 * N] << endl;
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}