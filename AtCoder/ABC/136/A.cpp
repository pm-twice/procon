#include<iostream>
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int A, B, C;

    cin >> A >> B >> C;

    int res = C - (A-B);
    if (res < 0){
        res = 0;
    }

    cout << res << endl;

    return 0;
}