// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define dbg(x) cerr<<#x<<" = "<<(x)<<" (L"<<__LINE__<< ")\n";
template<typename T>inline bool chmin(T &a,const T &b) {bool cmp=a>b;if(a>b)a=b;return cmp;}
template<typename T>inline bool chmax(T &a,const T &b) {bool cmp=a<b;if(a<b)a=b;return cmp;}
template<typename T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<typename T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template<typename T1,typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template<typename T,size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
typedef long long ll;
// === ACL ====
#include <atcoder/all>
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
// ============
// clang-format on
const ll INF = 1LL << 56;
int N;
vector<ll> A;

void solve() {
    cin >> N;
    A.resize(N);
    cin >> A;

    vector<vector<ll>> sum = vector<vector<ll>>(N + 1, vector<ll>(N + 1, 0));
    vector<vector<ll>> dp = vector<vector<ll>>(N + 1, vector<ll>(N + 1, 0));
    for (int l = 0; l < N; l++) {
        sum[l][l + 1] = A[l];
        for (int r = l + 2; r <= N; r++) {
            sum[l][r] = sum[l][r - 1] + A[r - 1];
        }
    }

    for (int length = 2; length <= N; length++) {
        for (int l = 0; l + length <= N; l++) {
            int r = l + length;
            dp[l][r] = INF;
            for (int mid = l + 1; mid < r; mid++) {
                chmin(dp[l][r], dp[l][mid] + dp[mid][r] + sum[l][r]);
            }
        }
    }
    cout << dp[0][N] << "\n";
}

signed main() {
    cout << setprecision(12);
    ios::sync_with_stdio(false);
    solve();
    return 0;
}