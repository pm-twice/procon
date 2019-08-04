#include<iostream>
#include<vector>
using namespace std;

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    int N;
    cin >> N;

    vector<int> H(N);
    for(int i=0; i<N; i++)
        cin >> H.at(i);

    for(int i=N-2; i>=0; i--){
        if(H[i] <= H[i+1])
            continue;
        else
            H[i]--;
    }
    
    bool res = true;
    for(int i=0; i<N-1; i++){
        int cur =H.at(i), nxt = H.at(i+1);
        if(cur <= nxt)
            continue;
        else{
            res = false;
            break;
        } 
    }

    if(res == true)
        cout << "Yes" << endl;
    else
        cout << "No" << endl;
    
    return 0;
}