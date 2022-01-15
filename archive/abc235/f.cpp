// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
#define FOR(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define FORR(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define ALL(a)  (a).begin(),(a).end()
#define len(a)  int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
template <typename T>inline bool chmin(T &a, const T &b) {bool compare = a > b;if (a > b) a = b;return compare;}
template <typename T>inline bool chmax(T &a, const T &b) {bool compare = a < b;if (a < b) a = b;return compare;}
template <typename T1, typename T2>pair<T1, T2> operator+(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first + r.first, l.second + r.second);}
template <typename T1, typename T2>pair<T1, T2> operator-(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first - r.first, l.second - r.second);}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v : vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v : vec) os << v.val() << ',';os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (typename map<T1, T2>::const_iterator it = map.begin();it != map.end();++it){stream << (*it).first << " --> " << (*it).second << endl;}return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for (auto v : arr) os << v << ',';os << ']';return os;}
typedef long long ll;
using mint = atcoder::modint1000000007;
const double PI = acos(-1.0);
const ll MOD = 1e9 + 7;
// clang-format on
mint dp[1010][1 << 10][2];
void solve() {
    string N;
    int M;
    cin >> N;
    cin >> M;
    vector<int> C(M);
    cin >> C;

    int ln = len(N);
    dp[0][0][1] = 1;
    auto pow10 = vector<mint>(1010, 1);
    FOR(i, 1, 1009) { pow10[i] = pow10[i - 1] * 10; }

    FOR(i, 0, ln - 1) {
        int val = N[i] - '0';
        // dbg(val);
        // mint pw = pow10[ln - i - 1];
        FOR(bts, 0, (1 << 10) - 1) {
            FOR(nxv, 0, val - 1) {
                // if ((bts >> nxv) & 1) {
                if (dp[i][bts][1].val() > 0)
                    dp[i + 1][bts | (1 << nxv)][0] += dp[i][bts][1] * 10 + nxv;
                // }
            }
            FOR(nxv, 0, 9) {
                // if ((bts >> nxv) & 1) {
                if (dp[i][bts][0].val() > 0)
                    dp[i + 1][bts | (1 << nxv)][0] += dp[i][bts][0] * 10 + nxv;
                // }
            }

            dp[i + 1][bts | (1 << val)][1] = dp[i][bts][1] * 10 + val;
        }
    }
    int target = 0;
    for (int x : C) {
        target |= (1 << x);
    }
    cout << "target" << target << endl;
    cout << dp[ln][target][0].val() << endl;
    cout << dp[ln][target][1].val() << endl;
    cout << (dp[ln][target][1] + dp[ln][target][0]).val() << endl;
    cout << dp[1][0][0].val() << endl;
    cout << dp[1][0][1].val() << endl;
}

signed main() {
    cout << setprecision(12);
    assert(true);
    solve();
    return 0;
}