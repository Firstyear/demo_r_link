#include "bindings.h"

double wrapper_add(double a, double b) {
    double result = f64sum(a, b);
    return result * 2;
}


