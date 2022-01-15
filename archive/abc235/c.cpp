// clang-format off
#include <bits/stdc++.h>
using namespace std;
#define FOR(i, a, n) for (int i = (int)(a); i < (int)(n); ++i)
#define FORR(i, a, n) for (int i = (int)(a); i > (int)(n); --i)
#define ALL(a)  (a).begin(),(a).end()
#define len(a)  int((a).size())
#define dbg(x) cerr << #x << " = " << (x) << " (L" << __LINE__ << ")\n";
template <typename T>istream &operator>>(istream &is, vector<T> &vec) {for (auto &v : vec) is >> v;return is;}
template <typename T>ostream &operator<<(ostream &os, const vector<T> &vec) {os << '[';for (auto v : vec) os << v << ',';os << ']';return os;}
template <typename T1, typename T2>ostream &operator<<(ostream &stream, const map<T1, T2>& map){for (typename map<T1, T2>::const_iterator it = map.begin();it != map.end();++it){stream << (*it).first << " --> " << (*it).second << endl;}return stream;}
template <typename T, size_t sz>ostream &operator<<(ostream &os, const array<T, sz> &arr) {os << '[';for (auto v : arr) os << v << ',';os << ']';return os;}
typedef long long ll;
// clang-format on

void solve() {
    int N, Q;
    cin >> N >> Q;
    vector<ll> A(N);
    cin >> A;

    map<ll, vector<ll>> mp;
    FOR(i, 0, N) { mp[A[i]].push_back(i); }
    FOR(i, 0, Q) {
        int x, k;
        cin >> x >> k;
        k--;

        if (len(mp[x]) > k) {
            cout << mp[x][k] + 1 << endl;
        } else
            cout << -1 << endl;
    }
}

signed main() {
    cout << setprecision(12);
    solve();
    return 0;
}