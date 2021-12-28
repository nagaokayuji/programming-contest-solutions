#include <atcoder/all>
#include <bits/stdc++.h>
using namespace std;
using namespace atcoder;
typedef long long ll;
#define rep(i, a, n) for (int i = (int)(a); i <= (int)(n); ++i)
#define rrep(i, a, n) for (int i = (int)(a); i >= (int)(n); --i)
#define all(x) x.begin(), x.end()
template <typename T>
inline bool chmin(T &a, const T &b)
{
    bool compare = a > b;
    if (a > b)
        a = b;
    return compare;
}
template <typename T>
inline bool chmax(T &a, const T &b)
{
    bool compare = a < b;
    if (a < b)
        a = b;
    return compare;
}
using pii = pair<int, int>;
using vi = vector<int>;
const ll MX = 1e5 + 5, INF = 5LL << 57, MOD = 1e9 + 7;

void solve()
{
    ll K;
    cin >> K;

    vector<vector<ll>> dp = vector<vector<ll>>(K + 1, vector<ll>(9, 0));
    dp[0][0] = 1;
    rep(ketawa, 0, K - 1)
    {
        rep(mod9, 0, 8)
        {
            rep(ad, 1, 9)
            {
                if (ketawa + ad > K)
                {
                    break;
                }
                dp[ketawa + ad][(mod9 + ad) % 9] += dp[ketawa][mod9] % MOD;
                dp[ketawa + ad][(mod9 + ad) % 9] %= MOD;
            }
        }
    }
    cout << dp[K][0] % MOD << endl;
}
signed main()
{
    cout << setprecision(12);
    solve();
    return 0;
}