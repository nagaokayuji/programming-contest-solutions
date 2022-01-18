// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define bit(n,m)(((n)>>(m))&1)
#define rep(i,n)for(int i=0;i<(int)(n);++i)
#define all(v) v.begin(), v.end()
template<class T>inline bool chmin(T &a,const T &b) {bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b) {bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
template<class T>using V=vector<T>;
template<class T>using VV=V<V<T>>;
typedef long long ll;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void __solve() {
    int N;
    cin >> N;
    VV<int> g(N);
    rep(i, N - 1) {
        int x, y;
        cin >> x >> y;
        x--;
        y--;
        g[x].push_back(y);
        g[y].push_back(x);
    }

    VV<mint> dp(N, V<mint>(2));

    auto dfs = [&](auto self, int x, int prv = -1) -> void {
        dp[x][0] = dp[x][1] = 1;
        for (auto &nx : g[x]) {
            if (nx == prv) continue;
            self(self, nx, x);
            dp[x][0] *= dp[nx][0] + dp[nx][1];
            dp[x][1] *= dp[nx][0];
        }
    };
    dfs(dfs, 0);
    cout << (dp[0][0] + dp[0][1]) << endl;
}

signed main() {
    cout << setprecision(12);
    ios::sync_with_stdio(false);
    assert(true);
    __solve();
    return 0;
}