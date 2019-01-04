#include <cstdint>
#include <cstdlib>

extern "C" {

// Generates a distribution sample and copies its data to the output buffer.
void sampleMHrandomWalk(uint32_t n_samples, double x0, double *out_buf);

} // extern "C"
