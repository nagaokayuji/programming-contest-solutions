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
using mint = atcoder::modint1000000007;
ostream &operator<<(ostream &os, const vector<mint> &vec) {os << '[';for (auto v : vec) os << v.val() << ',';os << ']';return os;}
// clang-format on
int N;
vector<pair<int, int>> xy;
void solve() {
    cin >> N;
    xy.resize(N);
    FOR(i, 0, N) cin >> xy[i].first >> xy[i].second;
    double ans = 0.0;
    for (auto a : xy)
        for (auto b : xy) {
            auto dif = make_pair(a.first - b.first, a.second - b.second);
            chmax(ans, sqrt(dif.first * dif.first + dif.second * dif.second));
        }
    cout << ans << endl;
}

signed main() {
    cout << setprecision(12);
    assert(true);
    solve();
    return 0;
}