// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
#define FOR(i, a, n) for (int i = (int)(a); i < (int)(n); ++i)
#define FORR(i, a, n) for (int i = (int)(a); i > (int)(n); --i)
#define ALL(a)  (a).begin(),(a).end()
#define len(a)  int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
#define MIN(v) *min_element(all(v))
#define MAX(v) *max_element(all(v))
template <typename T>inline bool chmin(T &a, const T &b) {bool compare = a > b;if (a > b) a = b;return compare;}
template <typename T>inline bool chmax(T &a, const T &b) {bool compare = a < b;if (a < b) a = b;return compare;}
template <typename T1, typename T2>pair<T1, T2> operator+(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first + r.first, l.second + r.second);}
template <typename T1, typename T2>pair<T1, T2> operator-(const pair<T1, T2> &l, const pair<T1, T2> &r) {return make_pair(l.first - r.first, l.second - r.second);}
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v : vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v : vec) os << v << ',';os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (typename map<T1, T2>::const_iterator it = map.begin();it != map.end();++it){stream << (*it).first << " --> " << (*it).second << endl;}return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for (auto v : arr) os << v << ',';os << ']';return os;}
typedef long long ll;
using i128 = __int128_t;
using mint = atcoder::modint1000000007;
const double PI = acos(-1.0);
const ll MOD = 1e9 + 7;
// clang-format on

void solve() {
    int N, M, Q;
    cin >> N >> M >> Q;
    auto edges = vector<tuple<int, int, int, int>>();
    int u, v, w;
    FOR(_, 0, M) {
        cin >> u >> v >> w;
        u--;
        v--;
        edges.emplace_back(u, v, w, -1);
    };
    FOR(i, 0, Q) {
        cin >> u >> v >> w;
        u--;
        v--;
        edges.emplace_back(u, v, w, i);
    };
    vector<bool> anss(Q, false);
    sort(ALL(edges), [](auto a, auto b) { return get<2>(a) < get<2>(b); });

    auto dsu = atcoder::dsu(N);

    for (auto edge : edges) {
        auto [u, v, w, i] = edge;
        if (i < 0) {
            if (!dsu.same(u, v)) dsu.merge(u, v);
        } else {
            if (!dsu.same(u, v)) anss[i] = true;
        }
    }
    for (auto x : anss) cout << (x ? "Yes" : "No") << endl;
}

signed main() {
    cout << setprecision(12);
    assert(true);
    solve();
    return 0;
}