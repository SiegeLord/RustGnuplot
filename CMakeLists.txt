cmake_minimum_required(VERSION 2.8)
project(RustGnuplot NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(GNUPLOT_ROOT src/lib.rs)
set(EXAMPLE1_ROOT examples/example1.rs)
set(EXAMPLE2_ROOT examples/example2.rs)
set(EXAMPLE3_ROOT examples/example3.rs)
set(EXAMPLE4_ROOT examples/example4.rs)
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/main.css")

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

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

add_custom_target(examples_target
                  ALL
                  DEPENDS ${EXAMPLE1_FULL_TARGET} ${EXAMPLE2_FULL_TARGET} ${EXAMPLE3_FULL_TARGET} ${EXAMPLE4_FULL_TARGET})


# Build documentation
rust_doc(${GNUPLOT_ROOT}
         TARGET_NAME GNUPLOT_DOC
         DESTINATION doc
         DEPENDS "${GNUPLOT_DEPS}")

# Copy the custom CSS
file(RELATIVE_PATH rel_dest "${CMAKE_BINARY_DIR}" "${CUSTOM_CSS_DEST}")
add_custom_target("DELETE_STOCK_CSS"
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET}
                  COMMAND "${CMAKE_COMMAND}" -E remove "${CUSTOM_CSS_DEST}"
                  COMMENT "Deleting stock ${rel_dest}")

add_custom_command(OUTPUT "${CUSTOM_CSS_DEST}"
                   DEPENDS "DELETE_STOCK_CSS"
                   DEPENDS "${CUSTOM_CSS_SRC}"
                   COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${CUSTOM_CSS_DEST}")

add_custom_target(doc
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET} ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${GNUPLOT_ARTIFACTS}
        DESTINATION lib)
