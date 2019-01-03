#include<bindings.hpp>

int main() {
    uint32_t n_samples = 1000;
    const Vec<double> *res = sample_mh_randomwalk(n_samples);
}
