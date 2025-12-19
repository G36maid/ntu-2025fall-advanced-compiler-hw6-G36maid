/*
 * Benchmark for Dead Store Elimination (DSE) Pass
 * 
 * This benchmark demonstrates measurable improvements from DSE optimization.
 * It contains various patterns of dead stores that should be eliminated.
 */

#include <stdio.h>
#include <stdlib.h>

/*
 * Test 1: Simple dead store pattern
 * First store is immediately overwritten without being read
 */
int test_simple_dead_store(int n) {
    int result = 0;
    
    // Dead store - never read
    result = 100;
    
    // This store overwrites the previous value
    result = n * 2;
    
    return result;
}

/*
 * Test 2: Multiple dead stores in sequence
 * Several stores are overwritten before being used
 */
int test_multiple_dead_stores(int a, int b) {
    int x = 0;
    int y = 0;
    
    // Dead stores - all overwritten
    x = 10;
    y = 20;
    x = 30;
    y = 40;
    
    // Live stores
    x = a;
    y = b;
    
    return x + y;
}

/*
 * Test 3: Dead stores in a loop
 * Loop-invariant dead stores that get overwritten
 */
int test_loop_dead_stores(int n) {
    int sum = 0;
    int temp = 0;
    
    for (int i = 0; i < n; i++) {
        // Dead store - overwritten in each iteration
        temp = 999;
        
        // Live store
        temp = i * i;
        sum += temp;
    }
    
    return sum;
}

/*
 * Test 4: Complex pattern with live and dead stores
 * Mix of stores that should and shouldn't be eliminated
 */
int test_complex_pattern(int *arr, int size) {
    int count = 0;
    int last = 0;
    
    for (int i = 0; i < size; i++) {
        // Dead store in most iterations (overwritten)
        last = -1;
        
        if (arr[i] > 0) {
            // Live store
            last = arr[i];
            count++;
        }
    }
    
    // This makes 'last' potentially live at loop exit
    if (count > 0) {
        return last;
    }
    
    return 0;
}

/*
 * Test 5: Benchmark with significant dead store overhead
 * This pattern has many redundant stores that waste memory bandwidth
 */
void test_memory_intensive(int *data, int size) {
    int cache[16];
    
    for (int i = 0; i < size; i++) {
        // Initialize cache with dead stores
        for (int j = 0; j < 16; j++) {
            cache[j] = 0;    // Dead stores
        }
        
        // Overwrite with live stores
        for (int j = 0; j < 16; j++) {
            cache[j] = data[i] + j;  // Live stores
        }
        
        // Use the cache
        int sum = 0;
        for (int j = 0; j < 16; j++) {
            sum += cache[j];
        }
        
        data[i] = sum;
    }
}

/*
 * Main benchmark driver
 */
int main(int argc, char *argv[]) {
    const int ITERATIONS = 1000000;
    
    printf("Starting DSE benchmark...\n");
    
    // Test 1
    int sum1 = 0;
    for (int i = 0; i < ITERATIONS; i++) {
        sum1 += test_simple_dead_store(i);
    }
    printf("Test 1 (simple dead store): sum = %d\n", sum1);
    
    // Test 2
    int sum2 = 0;
    for (int i = 0; i < ITERATIONS; i++) {
        sum2 += test_multiple_dead_stores(i, i + 1);
    }
    printf("Test 2 (multiple dead stores): sum = %d\n", sum2);
    
    // Test 3
    int sum3 = 0;
    for (int i = 0; i < 1000; i++) {
        sum3 += test_loop_dead_stores(1000);
    }
    printf("Test 3 (loop dead stores): sum = %d\n", sum3);
    
    // Test 4
    int test_arr[100];
    for (int i = 0; i < 100; i++) {
        test_arr[i] = (i % 2 == 0) ? i : -i;
    }
    int sum4 = 0;
    for (int i = 0; i < 10000; i++) {
        sum4 += test_complex_pattern(test_arr, 100);
    }
    printf("Test 4 (complex pattern): sum = %d\n", sum4);
    
    // Test 5
    int *data = malloc(1000 * sizeof(int));
    for (int i = 0; i < 1000; i++) {
        data[i] = i;
    }
    for (int i = 0; i < 100; i++) {
        test_memory_intensive(data, 1000);
    }
    printf("Test 5 (memory intensive): data[500] = %d\n", data[500]);
    free(data);
    
    printf("Benchmark completed.\n");
    return 0;
}
