cmake_minimum_required(VERSION 2.8)
project(RustGnuplot NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(cargo)
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(GNUPLOT_ROOT src/lib.rs)
set(EXAMPLE1_ROOT examples/example1.rs)
set(EXAMPLE2_ROOT examples/example2.rs)
set(EXAMPLE3_ROOT examples/example3.rs)
set(EXAMPLE4_ROOT examples/example4.rs)
set(ANIMATION_EXAMPLE_ROOT examples/animation_example.rs)
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")
set(STOCK_CSS "${CMAKE_BINARY_DIR}/doc/main.css;${CMAKE_BINARY_DIR}/doc/rustdoc.css;")
set(CARGO_DEPENDENCY_DIR "${CMAKE_BINARY_DIR}/argparse_dep/target/debug/deps")

set(RUSTC_FLAGS -L${CMAKE_BINARY_DIR}/lib -L ${CARGO_DEPENDENCY_DIR})
set(RUSTDOC_FLAGS -L${CMAKE_BINARY_DIR}/lib -L ${CARGO_DEPENDENCY_DIR})

cargo_dependency(argparse_dep
                 PACKAGE_NAMES argparse-rs
                 PACKAGE_VERSIONS =0.1.0)

# Get the dependencies of all the crates
get_rust_deps(${GNUPLOT_ROOT} GNUPLOT_DEPS)

# Build the library
rust_crate(${GNUPLOT_ROOT}
           TARGET_NAME GNUPLOT
           DESTINATION lib
           DEPENDS "${GNUPLOT_DEPS}"
           OTHER_RUSTC_FLAGS --crate-type rlib)

add_custom_target(library_target
                  ALL
                  DEPENDS ${GNUPLOT_FULL_TARGET})

# Build examples
rust_crate_auto(${EXAMPLE1_ROOT}
                TARGET_NAME EXAMPLE1
                DESTINATION examples
                DEPENDS "${GNUPLOT_FULL_TARGET}")

rust_crate_auto(${EXAMPLE2_ROOT}
                TARGET_NAME EXAMPLE2
                DESTINATION examples
                DEPENDS "${GNUPLOT_FULL_TARGET}")

rust_crate_auto(${EXAMPLE3_ROOT}
                TARGET_NAME EXAMPLE3
                DESTINATION examples
                DEPENDS "${GNUPLOT_FULL_TARGET}")

rust_crate_auto(${EXAMPLE4_ROOT}
                TARGET_NAME EXAMPLE4
                DESTINATION examples
                DEPENDS "${GNUPLOT_FULL_TARGET}")

rust_crate_auto(${ANIMATION_EXAMPLE_ROOT}
                TARGET_NAME ANIMATION_EXAMPLE
                DESTINATION examples
                DEPENDS "${GNUPLOT_FULL_TARGET}")

add_custom_target(examples_target
                  ALL
                  DEPENDS ${EXAMPLE1_FULL_TARGET} ${EXAMPLE2_FULL_TARGET} ${EXAMPLE3_FULL_TARGET} ${EXAMPLE4_FULL_TARGET} ${ANIMATION_EXAMPLE_FULL_TARGET})


# Build documentation
rust_doc(${GNUPLOT_ROOT}
         TARGET_NAME GNUPLOT_DOC
         DESTINATION doc
         DEPENDS "${GNUPLOT_DEPS}")

# Copy the custom CSS
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET}
                  COMMAND "${CMAKE_COMMAND}" -E remove -f ${STOCK_CSS}
                  COMMENT "Deleting stock CSS")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET} ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${GNUPLOT_ARTIFACTS}
        DESTINATION lib)
