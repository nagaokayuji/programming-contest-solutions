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
  int H, W;
  cin >> H >> W;
  int rs, cs;
  cin >> rs >> cs;
  int rt, ct;
  cin >> rt >> ct;
  rs--, cs--, rt--, ct--;

  V<string> S(H);
  cin >> S;

  dbg(S);

  int d1[4] = {-1, 0, 0, 1};
  int d2[4] = {0, -1, 1, 0};
  dbg(d1);
  dbg(d2);

  V<V<V<int>>> dist(H, V<V<int>>(W, V<int>(4, INF)));
  deque<tuple<int, int, int>> q;
  rep(i, 4) {
    dist[rs][cs][i] = 0;
    q.emplace_back(rs, cs, i);
  }
  while (!q.empty()) {
    auto [i, j, d] = q.front();
    dbg(i);
    dbg(j);
    int dis = dist[i][j][d];
    dbg(dis);
    q.pop_front();
    rep(diri, 4) {
      int di = d1[diri], dj = d2[diri];
      int nxi = i + di, nxj = j + dj;
      if (0 <= nxi && nxi < H && 0 <= nxj && nxj < W && S[nxi][nxj] == '.') {
        if (diri == d) {
          if (chmin(dist[nxi][nxj][d], dis))
            q.push_front(make_tuple(nxi, nxj, d));
        } else {
          if (chmin(dist[nxi][nxj][diri], dis + 1))
            q.emplace_back(nxi, nxj, diri);
        }
      }
    }
  };
  ;
  cout << *min_element(all(dist[rt][ct])) << endl;
}

signed main() {
  cout << setprecision(12);
  ios::sync_with_stdio(false);
  assert(true);
  _solve();
  return 0;
}