#include <bindings.hpp>
#include <iostream>

int main() {
    uint32_t n_samples = 1000;
    double *res = sample_mh_randomwalk(n_samples);
    std::cout << "Res " << res << std::endl;
    return 0;
}
