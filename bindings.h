#include <cstdint>
#include <cstdlib>

template<typename T>
struct Vec;

extern "C" {

Vec<double> sample_mh_randomwalk(uint32_t n_samples);

} // extern "C"
