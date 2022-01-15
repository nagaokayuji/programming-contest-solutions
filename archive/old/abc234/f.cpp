// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
using namespace atcoder;
typedef long long ll;
#define rep(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define rrep(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define all(x) x.begin(), x.end()
template <typename T>inline bool chmin(T &a, const T &b) {bool compare = a > b;if (a > b) a = b;return compare;}
template <typename T>inline bool chmax(T &a, const T &b) {bool compare = a < b;if (a < b) a = b;return compare;}
template <typename T1, typename T2>pair<T1, T2> operator+(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first + r.first, l.second + r.second);}
template <typename T1, typename T2>pair<T1, T2> operator-(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first - r.first, l.second - r.second);}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v : vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v : vec) os << v << ',';os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){stream<<"["<<endl; for (auto it = map.begin();it != map.end();++it){stream << (*it).first << ": " << (*it).second << endl;} stream<<"]"<<endl; return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for (auto v : arr) os << v << ',';os << ']';return os;}
// clang-format on
const ll MX = 1e6 + 5, INF = 5LL << 58, MOD = 1e9 + 7;

using mint = modint998244353;

mint fact[MX + 1];
mint ifact[MX + 1];
void init() {
    fact[0] = fact[1] = 1;
    rep(i, 2, MX) fact[i] = fact[i - 1] * i;
    rep(i, 0, MX) ifact[i] = fact[i].inv();
}
mint comb(int n, int k) { return fact[n] * ifact[k] * ifact[n - k]; }

void solve() {
    init();
    string s;
    cin >> s;

    int n = s.size();

    vector<mint> dp = vector<mint>(n + 1);
    dp[0] = 1;

    map<char, int> counts;
    for (auto c : s) {
        counts[c]++;
    }

    for (auto count : counts) {
        int val = count.second;
        // cout << "val: " << val << endl;
        auto dp2 = vector<mint>(n + 1);

        rep(length, 0, n) {
            rep(cnt, 0, min(length, val)) {
                dp2[length] += dp[length - cnt] * comb(length, cnt);
            }
        }
        dp = dp2;
    }
    mint ans = 0;
    rep(i, 1, n) { ans += dp[i]; }
    cout << ans.val() << endl;
}

signed main() {
    cout << setprecision(12);
    solve();
    return 0;
}