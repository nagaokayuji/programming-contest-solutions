// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
#define FOR(i, a, n) for (int i=(int)(a); i<(int)(n); ++i)
#define FORR(i, a, n) for (int i=(int)(a); i>(int)(n); --i)
#define ALL(a) (a).begin(),(a).end()
#define len(a) int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
template <typename T>inline bool chmin(T &a, const T &b) {bool cmp = a > b;if (a > b) a = b;return cmp;}
template <typename T>inline bool chmax(T &a, const T &b) {bool cmp = a < b;if (a < b) a = b;return cmp;}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v: vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v: vec)os << v << ',';os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (typename map<T1, T2>::const_iterator it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<',';os <<']';return os;}
typedef long long ll;
using mint = atcoder::modint998244353;
ostream &operator<<(ostream &os, const vector<mint> &vec) {os << '[';for (auto v : vec) os << v.val() << ',';os << ']';return os;}
// clang-format on

void solve() {
    string N;
    int M;
    cin >> N;
    cin >> M;
    vector<int> C(M);
    cin >> C;

    int ln = len(N);
    mint dp[1 << 10][2];
    mint sum[1 << 10][2];
    dp[0][1] = 1;

    FOR(i, 0, ln) {
        int val = N[i] - '0';
        mint ndp[1 << 10][2];
        mint nsum[1 << 10][2];
        FOR(strict, 0, 2) {
            FOR(d, 0, 10) {
                if ((strict == 1) && (d > val)) {
                    continue;
                }
                int nx_strict = ((strict == 1) && (val == d));

                FOR(bts, 0, (1 << 10)) {
                    if (dp[bts][strict] == 0) continue;

                    int nx_bts = bts | (1 << d);
                    if ((bts + d) == 0) nx_bts = 0;  // zero

                    ndp[nx_bts][nx_strict] += dp[bts][strict];
                    nsum[nx_bts][nx_strict] +=
                        sum[bts][strict] * 10 + d * dp[bts][strict];
                }
            }
        }
        swap(dp, ndp);
        swap(sum, nsum);
    }

    int target = 0;
    for (int x : C) {
        target |= (1 << x);
    }

    mint ans = 0;
    FOR(bits, 0, 1 << 10) {
        if ((bits & target) == target) {
            ans += sum[bits][0] + sum[bits][1];
        }
    }
    cout << ans.val() << endl;
}

signed main() {
    solve();
    return 0;
}