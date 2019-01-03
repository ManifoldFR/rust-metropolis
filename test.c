#include<stdint.h>
#include<bindings.h>

int main() {
    uint32_t n_samples = 1000;
    const Vec_f64 * res = sample_mh_randomwalk(n_samples);
    return 0;
}