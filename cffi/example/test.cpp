#include "bindings.hpp"
#include <iostream>
#include <vector>
using namespace std;

int main(int argc, char* argv[]) {
    int n_samples = 6;
    double x0 = 0.5;
    vector<double> samples(n_samples);
    sampleMHrandomWalk(n_samples, x0, &samples[0]);
    cout << "C++ call returns:" << endl;
    cout << "[";
    for (int i=0; i < n_samples; i++) {
        if (i > 0 && i % 8 == 0) {
            cout << endl;
        }
        cout << samples[i] << "\t";
    }
    cout << "]" << endl;
    return 0;
}
