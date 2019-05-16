#include<stdio.h>   
#include<time.h> 

int main() {
    clock_t t1, t2;  
    t1 = clock();  

    long long sum = 0;
    for (int j = 0; j < 100; j++) {
        
        for (int i = 0; i <= 2000000; i++) {
            if (i % 2 == 0) {
                sum += i;
            }
        }
    }

    t2 = clock();   
    float diff = ((float)(t2 - t1) / 1000000.0F ) * 1000;   
    printf("duration: %lld %f", sum, diff / 100);   
    return 0;
}