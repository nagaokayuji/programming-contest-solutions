// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
typedef long long ll;
#define FOR(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define FORR(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define all(x) x.begin(), x.end()
template <typename T>inline bool chmin(T &a, const T &b) {bool compare = a > b;if (a > b) a = b;return compare;}
template <typename T>inline bool chmax(T &a, const T &b) {bool compare = a < b;if (a < b) a = b;return compare;}
template <typename T1, typename T2>pair<T1, T2> operator+(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first + r.first, l.second + r.second);}
template <typename T1, typename T2>pair<T1, T2> operator-(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first - r.first, l.second - r.second);}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v : vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v : vec) os << v << ',';os << ']';return os;}
template<typename T1, typename T2>std::ostream &operator<<(std::ostream &stream, const std::map<T1, T2>& map){for (typename std::map<T1, T2>::const_iterator it = map.begin();it != map.end();++it){stream << (*it).first << " --> " << (*it).second << std::endl;}return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for (auto v : arr) os << v << ',';os << ']';return os;}
// clang-format on
const ll MX = 1e6 + 5, INF = 5LL << 58, MOD = 1e9 + 7;

using mint = modint1000000007;

void solve() {
    int N, Q;
    cin >> N >> Q;
    int X[Q], Y[Q], Z[Q];
    ll W[Q];
    FOR(i, 0, Q - 1) { cin >> X[i] >> Y[i] >> Z[i] >> W[i]; }
    mint ans = 1;
    FOR(bit, 0, 59) {  // a[i] のびっと
        mint ret = 0;
        FOR(i, 0, (1 << N) - 1) {
            bool bits[N];
            FOR(j, 0, N - 1) { bits[j] = (i >> j) & 1; }
            bool flag = true;
            FOR(j, 0, Q - 1) {
                bool target = (W[j] >> bit) & 1;
                if ((bits[X[j] - 1] | bits[Y[j] - 1] | bits[Z[j] - 1]) !=
                    target) {
                    flag = false;
                }
            }
            if (flag) ret++;
        }
        ans *= ret;
    }

    cout << ans.val() << endl;
}

signed main() {
    cout << setprecision(12);
    solve();
    return 0;
}