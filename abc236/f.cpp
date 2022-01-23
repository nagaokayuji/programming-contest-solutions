// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
#define bit(n,m)(((n)>>(m))&1)
#define rep1(a)for(int i=0;i<a;i++)
#define rep2(i,a)for(int i=0;i<a;i++)
#define rep3(i,a,b)for(int i=a;i<b;i++)
#define rep4(i,a,b,c)for(int i=a;i<b;i+=c)
#define overload4(a,b,c,d,e,...) e
#define range(...)overload4(__VA_ARGS__,rep4,rep3,rep2,rep1)(__VA_ARGS__)
#define all(v) v.begin(), v.end()
#define each(x, A) for(auto&& x :A)
template<class T>inline bool chmin(T &a,const T &b) {bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b) {bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
template<class T>using V=vector<T>;
template<class T>using VV=V<V<T>>;
using Int=long long;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on

void _solve() {
    int N;
    cin >> N;
    V<Int> C((1 << N) - 1);
    cin >> C;
    Int ans = 0;
    V<Int> dp = V<Int>(N, 1e9 + 3);
    set<Int> used;
    range(bts, 0, 1 << N) {
        int v = 1;
        if (used.find(bts) != used.end()) continue;
        range(b, N) {
            if (bit(bts, b)) {
                if (chmin(dp[b], C[bts - 1])) {
                    v = bts;
                };
            }
        }
        used.insert(v);
    }
    each(x, dp) ans += x;
    cout << ans << endl;
}

signed main() {
    cout << setprecision(12);
    ios::sync_with_stdio(false);
    assert(true);
    _solve();
    return 0;
}