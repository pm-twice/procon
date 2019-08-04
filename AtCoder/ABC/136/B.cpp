#include<iostream>
using namespace std;

unsigned GetDigit(unsigned num){
    unsigned digit=0;
    while(num!=0){
        num /= 10;
        digit++;
    }
    return digit;
}

int main(){
    cin.tie(0);
    ios::sync_with_stdio(false);

    unsigned int N, count = 0;

    cin >> N;

    for(int i=1; i<=N; i++){
        if(GetDigit(i) % 2 != 0)
            count++;
    }

    cout << count << endl;

    return 0;
}