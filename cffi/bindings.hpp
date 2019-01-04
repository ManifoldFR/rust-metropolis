#include <cstdint>
#include <cstdlib>

extern "C" {

// Generates a random walk Markov chain.
void randomWalk(uint32_t n_samples, double x0, double sigma, double *out_buf);

// Generates a distribution sample and copies its data to the output buffer.
void sampleMHrandomWalk(uint32_t n_samples, double x0, double *out_buf);

} // extern "C"
