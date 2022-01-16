// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
#define FOR(i, a, n) for (int i=(int)(a); i<(int)(n); ++i)
#define FORR(i, a, n) for (int i=(int)(a); i>(int)(n); --i)
#define ALL(a) (a).begin(),(a).end()
#define len(a) int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
#define endl "\n"
template <typename T>inline bool chmin(T &a, const T &b) {bool cmp = a > b;if (a > b) a = b;return cmp;}
template <typename T>inline bool chmax(T &a, const T &b) {bool cmp = a < b;if (a < b) a = b;return cmp;}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v: vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v: vec)os << v << ", ";os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<" -> "<<(*it).second<<endl;};return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for(auto v: arr)os<<v<<", ";os <<']';return os;}
typedef long long ll;
using mint = atcoder::modint998244353;
ostream &operator<<(ostream &os, const vector<mint> &vec) {os << '[';for (auto v : vec) os << v.val() << ", ";os << ']';return os;}
// clang-format on
const int MX = 1e5;
mint fact[MX + 1];
mint ifact[MX + 1];
void init() {
    fact[0] = fact[1] = 1;
    FOR(x, 2, MX + 1) fact[x] = fact[x - 1] * x;
    ifact[0] = ifact[1] = 1;
    ifact[MX] = fact[MX].inv();
    for (int x = MX - 1; x >= 2; x--) {
        ifact[x] = ifact[x + 1] * (x + 1);
    }
}
mint comb(int n, int k) { return fact[n] * ifact[k] * ifact[n - k]; }

void solve() {
    string S;
    cin >> S;
    ;
    map<int, int> freq;
    for (auto c : S) freq[c - 'a']++;
    int n = len(S);

    vector<mint> dp(n + 1);
    dp[0] = 1;

    for (auto c : freq) {
        int cnt = c.second;
        vector<mint> ndp(n + 1);
        FOR(k, 0, cnt + 1) {
            FOR(j, 0, n - k + 1) {
                int nj = j + k;
                ndp[nj] += dp[j] * comb(nj, j);
            }
        }
        dp = ndp;
    }

    mint ans = 0;
    FOR(i, 1, n + 1) { ans += dp[i]; }
    cout << ans.val() << endl;
}

signed main() {
    init();
    cout << setprecision(12);
    ios::sync_with_stdio(false);
    assert(true);
    solve();
    return 0;
}