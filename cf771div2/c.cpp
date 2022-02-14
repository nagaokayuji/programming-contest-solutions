// clang-format off
#include<bits/stdc++.h>
using namespace std;
#ifdef __LOCAL
#define dbg(x)cerr<<#x<<"="<<(x)<<" (L"<<__LINE__<<")\n";
#define dbg2(x,y)cerr<<#x<<"="<<(x)<<", "<<#y<<"="<<(y)<<" (L"<<__LINE__<<")\n";
#else
#define dbg(x)
#define dbg2(x,y)
#endif
#define bit(n,m)(((n)>>(m))&1)
#define rep(i,n)for(int i=0;i<(int)n;i++)
#define all(v) v.begin(),v.end()
#define each(x,A) for(auto&& x:A)
template<class T>inline bool chmin(T &a,const T &b){bool cmp=a>b;if(a>b)a=b;return cmp;}
template<class T>inline bool chmax(T &a,const T &b){bool cmp=a<b;if(a<b)a=b;return cmp;}
template<class T>istream &operator>>(istream &is,vector<T> &vec){for(auto &v: vec)is>>v;return is;}
template<class T>ostream &operator<<(ostream &os,const vector<T> &vec){os<<'[';for (auto v: vec)os<<v<<", ";os<<']';return os;}
template<class T1,class T2>ostream &operator<<(ostream &stream,const map<T1,T2>& map){for(auto it=map.begin();it!=map.end();++it){stream<<(*it).first<<"-> "<<(*it).second<<endl;};return stream;}
template<class T1,class T2>ostream &operator<<(ostream &stream,const pair<T1,T2>& pair){stream<<"("<<pair.first<<", "<<pair.second<<")";return stream;}
template<class T,size_t sz>ostream &operator<<(ostream &os,const array<T,sz> &arr){os<<'[';for(auto v:arr)os<<v<<", ";os<<']';return os;}
template<class T>using V=vector<T>;
void yn(bool ans){cout<<(ans?"Yes":"No")<<"\n";}
using Int=long long;
// using ld=long double;
// using i128=__int128_t;
// istream &operator>>(istream &is,i128 &v){string s;is >> s;v=0;for(int i=0;i<(int)s.size();i++){if(isdigit(s[i])){v=v*10+s[i]-'0';}}if(s[0]=='-'){v*=-1;}return is;} 
// ostream &operator<<(ostream &os,const i128 &v){if(v==0){return (os<<"0");}i128 num=v;if(v<0){os<<'-';num=-num;}string s;for(;num > 0;num /=10){s.push_back((char)(num % 10)+'0');}reverse(s.begin(),s.end());return (os<<s);}
const Int LINF=5LL<<58;const int INF=5<<28;
//===ACL====
// #include<atcoder/all>
// using mint=atcoder::modint1000000007;
// ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
//============
// clang-format on
struct dsu {
 public:
  dsu() : _n(0) {}
  explicit dsu(int n) : _n(n), parent_or_size(n, -1) {}

  int merge(int a, int b) {
    assert(0 <= a && a < _n);
    assert(0 <= b && b < _n);
    int x = leader(a), y = leader(b);
    if (x == y) return x;
    if (-parent_or_size[x] < -parent_or_size[y]) std::swap(x, y);
    parent_or_size[x] += parent_or_size[y];
    parent_or_size[y] = x;
    return x;
  }

  bool same(int a, int b) {
    assert(0 <= a && a < _n);
    assert(0 <= b && b < _n);
    return leader(a) == leader(b);
  }

  int leader(int a) {
    assert(0 <= a && a < _n);
    if (parent_or_size[a] < 0) return a;
    return parent_or_size[a] = leader(parent_or_size[a]);
  }

  int size(int a) {
    assert(0 <= a && a < _n);
    return -parent_or_size[leader(a)];
  }

  std::vector<std::vector<int>> groups() {
    std::vector<int> leader_buf(_n), group_size(_n);
    for (int i = 0; i < _n; i++) {
      leader_buf[i] = leader(i);
      group_size[leader_buf[i]]++;
    }
    std::vector<std::vector<int>> result(_n);
    for (int i = 0; i < _n; i++) {
      result[i].reserve(group_size[i]);
    }
    for (int i = 0; i < _n; i++) {
      result[leader_buf[i]].push_back(i);
    }
    result.erase(
        std::remove_if(result.begin(), result.end(),
                       [&](const std::vector<int> &v) { return v.empty(); }),
        result.end());
    return result;
  }

 private:
  int _n;
  // root node: -1 * component size
  // otherwise: parent
  std::vector<int> parent_or_size;
};

void _solve() {
  int N;
  cin >> N;
  V<int> A(N);
  cin >> A;
  each(x, A) x--;

  auto ds = dsu(N);

  set<int> s;
  rep(i, N) s.insert(i);

  V<int> bigger;
  int mx = 0;

  rep(now, N) {
    if (mx > A[now]) {
      ds.merge(mx, A[now]);
      while (!bigger.empty() && bigger.back() > A[now]) {
        ds.merge(bigger.back(), A[now]);
        // dbg2(bigger.back(), A[now]);
        bigger.pop_back();
      }
      bigger.push_back(mx);
    } else {
      bigger.push_back(A[now]);
    }
    chmax(mx, A[now]);
  }

  cout << ds.groups().size() << endl;
}

signed main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  cout << setprecision(12);
  int t;
  cin >> t;
  while (t--) _solve();
  return 0;
}