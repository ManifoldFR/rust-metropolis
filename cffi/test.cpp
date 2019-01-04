#include "bindings.hpp"
#include <iostream>
#include <vector>
using namespace std;

int main() {
    uint32_t n_samples = 6;
    double x0 = 0.5;
    vector<double> samples(n_samples, 0.0);
    sample_mh_randomwalk(n_samples, x0, &samples[0]);
    for (int i=0; i < n_samples; i++) {
        if (i > 0 && i % 5 == 0) {
            cout << endl;
        }
        cout << "val #" << i << " : " << samples[i] << "\t";
    }
    return 0;
}
