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
int N, P, K;
vector<vector<ll>> A;

bool isok(ll x, bool strict) {
    vector<vector<ll>> dist = A;
    for (auto &r : dist) {
        for (auto &v : r) {
            if (v == -1) {
                v = x;
            }
        }
    }
    // cout << dist << endl;
    FOR(k, 0, N - 1) FOR(i, 0, N - 1) FOR(j, 0, N - 1) {
        // chmin(dist[i][j], dist[i][k] + dist[k][j]);
        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
    }

    // cout << dist << endl;
    int cnt = 0;
    FOR(i, 0, N - 1) {
        FOR(j, i + 1, N - 1) {
            if ((i != j) && dist[i][j] <= P) {
                cnt++;
            }
        }
    }
    // cout << x << " " << cnt << endl;
    if (strict) {
        return cnt > K;
    }
    return cnt >= K;
}

void solve() {
    cin >> N >> P >> K;
    FOR(i, 0, N - 1) {
        vector<ll> a(N);
        cin >> a;
        A.emplace_back(a);
    }
    ll ok = 0;
    ll ng = 1e15;
    while (ng - ok > 1) {
        auto mid = (ok + ng) / 2;
        if (isok(mid, true)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    ll lower = ok;
    ok = 0;
    ng = 1e15;
    while (ng - ok > 1) {
        auto mid = (ok + ng) / 2;
        if (isok(mid, false)) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ll upper = ok;
    if (upper - lower > 1e12) {
        cout << "Infinity" << endl;
        return;
    }
    cout << upper - lower << endl;
}

signed main() {
    cout << setprecision(12);
    solve();
    return 0;
}