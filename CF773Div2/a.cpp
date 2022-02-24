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
using ld=long double;
using i128=__int128_t;
istream &operator>>(istream &is,i128 &v){string s;is >> s;v=0;for(int i=0;i<(int)s.size();i++){if(isdigit(s[i])){v=v*10+s[i]-'0';}}if(s[0]=='-'){v*=-1;}return is;} 
ostream &operator<<(ostream &os,const i128 &v){if(v==0){return (os<<"0");}i128 num=v;if(v<0){os<<'-';num=-num;}string s;for(;num > 0;num /=10){s.push_back((char)(num % 10)+'0');}reverse(s.begin(),s.end());return (os<<s);}
const Int LINF=5LL<<58;const int INF=5<<28;
const int dx[8]={1,-1,0,0,1,1,-1,-1},dy[8]={0,0,1,-1,1,-1,1,-1};
//===ACL====
// #include<atcoder/all>
// using mint=atcoder::modint1000000007;
// ostream &operator<<(ostream &os,const mint x){os<<x.val();return os;}
//============
// clang-format on

void わーい() {
  //
  V<pair<int, int>> points(3);
  set<int> ys;
  int dup_y = 0;
  rep(i, 3) {
    int x, y;
    // cin >> x >> y;
    cin >> points[i].first >> points[i].second;
    x = points[i].first;
    y = points[i].second;
    if (ys.find(y) != ys.end()) {
      dup_y = y;
    }
    ys.insert(y);
  }
  if (ys.size() == 3) {
    cout << 0 << endl;
    return;
  }

  int ot_y = 0;
  each(y, ys) {
    if (y != dup_y) ot_y = y;
  }

  if (ot_y > dup_y) {
    cout << 0 << endl;
    return;
  }

  V<int> dup_xs;
  each(xy, points) {
    if (xy.second == dup_y) {
      dup_xs.push_back(xy.first);
    }
  }
  dbg(dup_xs);
  cout << abs(dup_xs[1] - dup_xs[0]) << endl;
}

signed main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);
  cout << setprecision(12);
  int t;
  cin >> t;
  while (t--) わーい();
  return 0;
}