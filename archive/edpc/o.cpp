// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define bit(n,m)(((n)>>(m))&1)
#define rep(i,n)for(int i=0;i<(int)(n);++i)
template<class T>inline bool chmin(T &a,const T &b) {bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b) {bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
template<class T>using vec=vector<T>;
typedef long long ll;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on
int N;

void solve() {
    cin >> N;
    vec<vec<int>> A(N, vec<int>(N));
    for (auto &row : A) {
        cin >> row;
    }

    vec<mint> dp(1 << N);
    dp[0] = 1;
    rep(S, 1 << N) {
        int popc = __builtin_popcount(S);
        rep(j, N) {
            if (bit(S, j) && A[popc - 1][j]) {
                dp[S] += dp[S - (1 << j)];
            }
        }
    }

    cout << dp[(1 << N) - 1] << "\n";
}

signed main() {
    cout << setprecision(12);
    ios::sync_with_stdio(false);
    assert(true);
    solve();
    return 0;
}