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
void yn(bool ans){cout<<(ans?"YES":"NO")<<"\n";}
using Int=long long;
// using i128=__int128_t;
const Int LINF=5LL<<58;
const int INF=5<<28;
using T=tuple<int,int,int>;
// === ACL ====
// #include <atcoder/all>
// using mint=atcoder::modint1000000007;
// ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on
map<T, int> mp;
int ask(int i, int j, int k) {
  dbg(i);
  dbg(j);
  dbg(k);
  i++, j++, k++;
  int mn = min({i, j, k});
  int mx = max({i, j, k});
  int md = i + j + k - mn - mx;
  i = mn;
  j = md;
  k = mx;
  T key = make_tuple(i, j, k);
  if (mp[key]) {
    return mp[key];
  }
  int ret;
  cout << "? " << i << " " << j << " " << k << endl;
  cout.flush();
  cin >> ret;
  assert(ret > 0);
  return mp[key] = ret;
}

void answer(int i, int j) {
  i++, j++;
  cout << "! " << i << " " << j << endl;
  cout.flush();
}

void _solve() {
  mp = map<T, int>();
  ;
  int N;
  cin >> N;
  int mxret = 0;
  int a1 = 0, a2 = 1;
  V<bool> koho(N, true);
  rep(i, N) {
    if (i == a1 || i == a2) continue;
    if (not koho[i]) continue;
    int r1 = ask(a1, a2, i);

    for (int j = i + 1; j < N; j++) {
      if (not koho[j]) continue;
      if (j == a1 || j == a2) continue;

      int r1 = ask(a1, a2, i);
      int r2 = ask(a1, a2, j);
      chmax(mxret, r1);
      chmax(mxret, r2);
      if (r1 == r2) {  // aida
        koho[i] = koho[j] = false;
        break;
      }
      if (r1 < r2) {
        a2 = j;
      }
      if (r1 > r2) {
        a1 = j;
      }
    }
    a1 = i;
  }
  V<int> kohos;
  rep(i, N) if (koho[i]) kohos.push_back(i);
  dbg(kohos);

  V<int> cnt(N);
  for (auto &x : mp) {
    auto [a, b, c] = x.first;
    if (x.second == mxret) {
      cnt[a - 1]++;
      cnt[b - 1]++;
      cnt[c - 1]++;
    }
  }

  V<int> ans;
  rep(i, N) {
    if (cnt[i] == 2) {
      ans.push_back(i);
    }
  }
  answer(ans[0], ans[1]);
}

signed main() {
  cout << setprecision(12);
  assert(true);
  int t;
  cin >> t;
  while (t--) _solve();
  return 0;
}