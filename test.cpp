#include <bindings.hpp>
#include <iostream>
using namespace std;

int main() {
    uint32_t n_samples = 40;
    double *res = sample_mh_randomwalk(n_samples);
    for (int i=0; i < n_samples; i++) {
        if (i > 0 && i % 5 == 0) {
            cout << endl;
        }
        cout << "Val #" << i << " : " << res[i] << "\t";
    }
    return 0;
}
