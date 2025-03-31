#include <stdio.h>

int main(int argc, char** argv) {
    printf("Hello from SmashLang!\n");
    printf("This should be visible in the output.\n");
    
    // Simulate the if statement
    // const a = 10; const b = 20;
    if (10 < 20) {
        printf("a is less than b\n");
    }
    
    // Simulate the logical operators
    // const c = true; const d = false;
    if (1 && !0) {
        printf("Logical AND works!\n");
    }
    
    if (1 || 0) {
        printf("Logical OR works!\n");
    }
    
    return 0;
}
