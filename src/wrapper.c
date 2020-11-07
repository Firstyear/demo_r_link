#include "bindings.h"
#include <R.h>
#include <Rinternals.h>

SEXP wrapper_add(SEXP a, SEXP b) {
    SEXP result = PROTECT(allocVector(REALSXP, 1));
    REAL(result)[0] = f64sum(asReal(a), asReal(b));
    UNPROTECT_PTR(result);
    return result;
}


