
dyn.load("target/debug/libdemo.so")

add <- function(a, b) {
    .Call("wrapper_add", a, b)
}

add(1, 2)


