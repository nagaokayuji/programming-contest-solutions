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
  int H, W;
  cin >> H >> W;
  auto enc = [&](int i, int j) { return i * W + j; };
  // auto dec = [&](int val) { return make_pair(val / W, val % W); };
  atcoder::dsu dsu(H * W);
  V<V<bool>> field(H, V<bool>(W));

  int di[4] = {-1, 0, 0, 1};
  int dj[4] = {0, -1, 1, 0};
  int Q;
  cin >> Q;
  auto yn = [&](bool v) { cout << (v ? "Yes" : "No") << "\n"; };

  while (Q--) {
    int type;
    cin >> type;
    if (type == 1) {  // fill
      int r, c;
      cin >> r >> c;
      r--;
      c--;

      field[r][c] = true;

      rep(k, 4) {
        auto nr = r + di[k];
        auto nc = c + dj[k];
        if (0 <= nr && nr < H && 0 <= nc && nc < W && field[nr][nc]) {
          dsu.merge(enc(nr, nc), enc(r, c));
        }
      }
    } else {
      int ra, ca, rb, cb;
      cin >> ra >> ca >> rb >> cb;
      ra--;
      ca--;
      rb--;
      cb--;
      yn(field[ra][ca] && dsu.same(enc(ra, ca), enc(rb, cb)));
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