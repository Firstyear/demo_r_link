#include "bindings.h"

double wrapper_add(double a, double b) {
    double result = f64sum(a, b);
    return result * 2;
}

double reverse_call(double a) {
    return a * 2;
}
