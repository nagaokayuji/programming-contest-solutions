// clang-format off
#include <bits/stdc++.h>
#include <atcoder/all>
using namespace std;
#define FOR(i, a, n) for (int i = (int)(a); i < (int)(n); ++i)
#define FORR(i, a, n) for (int i = (int)(a); i > (int)(n); --i)
#define ALL(a)  (a).begin(),(a).end()
#define len(a)  int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
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
    int a, N;
    cin >> a >> N;

    int mx = N * 10 + 3;
    vector<int> count = vector<int>(mx, 1e8);
    count[1] = 0;
    deque<int> dq;
    dq.push_back(1);
    while (!dq.empty()) {
        ll val = dq.front();
        dq.pop_front();

        if ((ll)(val * a) < mx && chmin(count[val * a], count[val] + 1)) {
            dq.push_back(val * a);
        }

        if (val >= 10 && val % 10) {
            string s = to_string(val);
            s = s.back() + s.substr(0, s.length() - 1);
            ll nxi = stoi(s);
            if (nxi < mx && chmin(count[nxi], count[val] + 1)) {
                dq.push_back(nxi);
            }
        }
    }
    int ans = count[N];
    if (ans == 1e8) {
        cout << -1 << endl;
    } else {
        cout << ans << endl;
    }
}

signed main() {
    cout << setprecision(12);
    assert(true);
    solve();
    return 0;
}