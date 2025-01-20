// #include <benchmark/benchmark.h>
#include <immintrin.h> // 包含 AVX 指令集头文件

__m256 mm256_fmadd_ps_test() {
    // 创建包含相同值的向量
    __m256 a = _mm256_set1_ps(1.0f); // 向量 a 的每个元素都是 1.0
    __m256 b = _mm256_set1_ps(2.0f); // 向量 b 的每个元素都是 2.0
    __m256 c = _mm256_set1_ps(3.0f); // 向量 c 的每个元素都是 3.0

    // 执行 FMA 操作：a * b + c
    return _mm256_fmadd_ps(a, b, c);
}
int main()
{
    __m256 a = _mm256_set1_ps(1.0f);
    __m256 b = _mm256_set1_ps(2.0f);
    __m256 c = _mm256_set1_ps(3.0f);

    // 执行_fma 操作 1000 次

    c = _mm256_fmadd_ps(a, b, c); // 执行_fma 操作

}
