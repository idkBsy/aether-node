// SUMMARY: AMD HIP kernel for parallelized Ordinary Least Squares (OLS) regression to compute clock skew.
#include <hip/hip_runtime.h>
#include <stdint.h>

typedef struct {
    uint64_t cycle_count;
    uint32_t caplen;
    uint32_t len;
    const uint8_t *data;
} RawSignal;

__device__ double atomicAddDouble(double* address, double val) {
    unsigned long long int* address_as_ull = (unsigned long long int*)address;
    unsigned long long int old = *address_as_ull, assumed;
    do {
        assumed = old;
        old = atomicCAS(address_as_ull, assumed,
                        __double_as_longlong(val + __longlong_as_double(assumed)));
    } while (assumed != old);
    return __longlong_as_double(old);
}

__global__ void compute_means_kernel(
    const RawSignal* signals,
    const double* ref_clocks,
    size_t num_elements,
    double* sum_x,
    double* sum_y
) {
    __shared__ double shared_x[256];
    __shared__ double shared_y[256];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    double local_x = 0.0;
    double local_y = 0.0;

    if (idx < num_elements) {
        local_x = ref_clocks[idx];
        local_y = (double)signals[idx].cycle_count;
    }

    shared_x[tid] = local_x;
    shared_y[tid] = local_y;
    __syncthreads();

    for (int s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            shared_x[tid] += shared_x[tid + s];
            shared_y[tid] += shared_y[tid + s];
        }
        __syncthreads();
    }

    if (tid == 0) {
        atomicAddDouble(sum_x, shared_x[0]);
        atomicAddDouble(sum_y, shared_y[0]);
    }
}

__global__ void compute_ols_kernel(
    const RawSignal* signals,
    const double* ref_clocks,
    size_t num_elements,
    double mean_x,
    double mean_y,
    double* numerator,
    double* denominator
) {
    __shared__ double shared_num[256];
    __shared__ double shared_den[256];

    int tid = threadIdx.x;
    int idx = blockIdx.x * blockDim.x + threadIdx.x;

    double local_num = 0.0;
    double local_den = 0.0;

    if (idx < num_elements) {
        double x = ref_clocks[idx];
        double y = (double)signals[idx].cycle_count;
        double diff_x = x - mean_x;
        double diff_y = y - mean_y;
        
        local_num = diff_x * diff_y;
        local_den = diff_x * diff_x;
    }

    shared_num[tid] = local_num;
    shared_den[tid] = local_den;
    __syncthreads();

    for (int s = blockDim.x / 2; s > 0; s >>= 1) {
        if (tid < s) {
            shared_num[tid] += shared_num[tid + s];
            shared_den[tid] += shared_den[tid + s];
        }
        __syncthreads();
    }

    if (tid == 0) {
        atomicAddDouble(numerator, shared_num[0]);
        atomicAddDouble(denominator, shared_den[0]);
    }
}

extern "C" void launch_skew_estimation(
    const RawSignal* d_signals,
    const double* d_ref_clocks,
    size_t num_elements,
    double* h_delta
) {
    double h_sums[2] = {0.0, 0.0};
    double *d_sum_x, *d_sum_y;
    hipMalloc((void**)&d_sum_x, sizeof(double));
    hipMalloc((void**)&d_sum_y, sizeof(double));
    hipMemcpy(d_sum_x, &h_sums[0], sizeof(double), hipMemcpyHostToDevice);
    hipMemcpy(d_sum_y, &h_sums[1], sizeof(double), hipMemcpyHostToDevice);

    int blockSize = 256;
    int numBlocks = (num_elements + blockSize - 1) / blockSize;

    compute_means_kernel<<<numBlocks, blockSize>>>(d_signals, d_ref_clocks, num_elements, d_sum_x, d_sum_y);
    hipDeviceSynchronize();

    double mean_x, mean_y;
    hipMemcpy(&mean_x, d_sum_x, sizeof(double), hipMemcpyDeviceToHost);
    hipMemcpy(&mean_y, d_sum_y, sizeof(double), hipMemcpyDeviceToHost);
    mean_x /= num_elements;
    mean_y /= num_elements;

    double h_num_den[2] = {0.0, 0.0};
    double *d_numerator, *d_denominator;
    hipMalloc((void**)&d_numerator, sizeof(double));
    hipMalloc((void**)&d_denominator, sizeof(double));
    hipMemcpy(d_numerator, &h_num_den[0], sizeof(double), hipMemcpyHostToDevice);
    hipMemcpy(d_denominator, &h_num_den[1], sizeof(double), hipMemcpyHostToDevice);

    compute_ols_kernel<<<numBlocks, blockSize>>>(d_signals, d_ref_clocks, num_elements, mean_x, mean_y, d_numerator, d_denominator);
    hipDeviceSynchronize();

    double num, den;
    hipMemcpy(&num, d_numerator, sizeof(double), hipMemcpyDeviceToHost);
    hipMemcpy(&den, d_denominator, sizeof(double), hipMemcpyDeviceToHost);

    *h_delta = (den == 0.0) ? 0.0 : (num / den);

    hipFree(d_sum_x);
    hipFree(d_sum_y);
    hipFree(d_numerator);
    hipFree(d_denominator);
}