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

void _solve() {
  Int N, B;
  cin >> N >> B;

  // how many f(x)?
  // m-f(m) = b
  // m = b + f(m)
  auto f = [&](Int x) {
    Int now = x;
    Int ret = 1LL;
    while (now) {
      ret *= (now % 10);
      now /= 10;
    }
    return ret;
  };

  auto pow = [&](Int a, Int b) {
    Int ret = 1LL;
    Int now = a;
    while (b) {
      if (b & 1) {
        ret *= now;
      }

      b >>= 1;
      now *= now;
      if (ret > N * 10) {
        return Int(N * 10 + 1);
      }
    }
    return ret;
  };
  dbg(pow(2, 10));
  assert(pow(2, 10) == 1024);
  assert(pow(3, 3) == 27);
  const Int UPPER = N - B;
  Int ans = 0;
  rep(two, 40) {
    Int pow2 = pow(2, two);
    if (pow2 > UPPER) break;
    rep(three, 40) {
      if (two + three > 34) break;
      Int pow3 = pow(3, three);
      if (pow3 > UPPER / pow2) break;

      rep(five, 34) {
        if (two + three + five > 34) break;
        Int pow5 = pow(5, five);
        if (pow5 > UPPER / pow2 / pow3) break;

        rep(seven, 34) {
          if (two + three + five + seven > 34) break;
          Int pow7 = pow(7, seven);
          if (pow7 > UPPER / pow2 / pow3 / pow5) break;

          Int prd = pow2 * pow3 * pow5 * pow7;
          Int m = prd + B;
          if (m > N) break;
          if (f(m) == prd) ans++;
        }
      }
    }
  }
  if ((f(B) == 0) && N >= B) ans++;
  cout << ans << endl;
}

signed main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  cout << setprecision(12);
  _solve();
  return 0;
}