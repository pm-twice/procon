#include<iostream>
#include<string>
#include<vector>
#include<algorithm>
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    string S;   // len = 10^5
    cin >> S;
    int N = (int)S.size();

    vector<int> count(N,0);

    int r_st, r_end, l_st, l_end;
    int idx = 0;

    while(idx < N){
        // find start & end of "R+", "L+"
        r_st = idx;
        while(S[idx] == 'R') idx++;
        r_end = max(idx-1, r_st);
        l_st = idx;
        while(S[idx] == 'L') idx++;
        l_end = max(idx-1, l_st);

        count[r_end] = 1 + (r_end - r_st) / 2 + (l_end - r_end) / 2; 
        count[l_st] = 1 + (l_st - r_st) / 2 + (l_end - l_st) / 2; 
    }

    for(int i=0; i<N; i++){
        if(i != 0)
            cout << " ";
        cout << count[i];
    }
    cout << endl;

    return 0;
}