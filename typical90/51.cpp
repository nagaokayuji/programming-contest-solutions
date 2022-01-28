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
  Int N, K, P;
  cin >> N >> K >> P;
  V<Int> A(N);
  cin >> A;

  V<Int> A1, A2;
  rep(i, N) {
    if (i < N / 2)
      A1.push_back(A[i]);
    else
      A2.push_back(A[i]);
  }

  auto calc = [](V<Int> A) {
    Int n = A.size();

    V<V<Int>> ret(64);
    rep(bts, 1 << n) {
      int pcnt = __builtin_popcount(bts);
      Int sum = 0;
      rep(i, n) {
        if (bit(bts, i)) {
          sum += A[i];
          //
        }
      }
      ret[pcnt].push_back(sum);
    }
    return ret;
  };

  auto C1 = calc(A1);
  auto C2 = calc(A2);

  Int ans = 0;
  rep(cnt1, K + 1) {
    Int cnt2 = K - cnt1;
    auto v1 = C1[cnt1];
    auto v2 = C2[cnt2];
    sort(all(v1));
    sort(all(v2));

    int n1 = v1.size();
    // int n2 = v2.size();
    for (int i = 0; i < n1; i++) {
      if (v1[i] > P) break;
      Int target = P - v1[i];
      Int c2 = upper_bound(all(v2), target) - v2.begin();
      ans += c2;
    }
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