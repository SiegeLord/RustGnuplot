cmake_minimum_required(VERSION 2.8)
project(RustGnuplot NONE)

list(APPEND CMAKE_MODULE_PATH "${CMAKE_SOURCE_DIR}/cmake")
find_package(rustc)
find_package(rustdoc)
include(Rust)

set(GNUPLOT_ROOT src/lib.rs)
set(EXAMPLE1_ROOT examples/example1.rs)
set(EXAMPLE2_ROOT examples/example2.rs)
set(CUSTOM_CSS_SRC "${CMAKE_SOURCE_DIR}/doc/main.css")
set(CUSTOM_CSS_DEST "${CMAKE_BINARY_DIR}/doc/gnuplot/main.css")

set(RUSTC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")
set(RUSTDOC_FLAGS "-L${CMAKE_BINARY_DIR}/lib")

# Get the dependencies of all the crates
get_rust_deps(${GNUPLOT_ROOT} GNUPLOT_DEPS)
get_rust_deps(${EXAMPLE1_ROOT} EXAMPLE1_DEPS)
get_rust_deps(${EXAMPLE2_ROOT} EXAMPLE2_DEPS)

# Build the library
rust_crate(${GNUPLOT_ROOT}
           TARGET_NAME GNUPLOT
           DESTINATION lib
           DEPENDS "${GNUPLOT_DEPS}")

add_custom_target(library_target
                  ALL
                  DEPENDS ${GNUPLOT_FULL_TARGET})

# Build examples
rust_crate(${EXAMPLE1_ROOT}
           TARGET_NAME EXAMPLE1
           DESTINATION examples
           DEPENDS "${GNUPLOT_FULL_TARGET};${EXAMPLE1_DEPS}")

rust_crate(${EXAMPLE2_ROOT}
           TARGET_NAME EXAMPLE2
           DESTINATION examples
           DEPENDS "${GNUPLOT_FULL_TARGET};${EXAMPLE2_DEPS}")

add_custom_target(examples_target
                  ALL
                  DEPENDS ${EXAMPLE1_FULL_TARGET} ${EXAMPLE2_FULL_TARGET})


# Build documentation
rust_doc(${GNUPLOT_ROOT}
         TARGET_NAME GNUPLOT_DOC
         DESTINATION doc
         DEPENDS "${GNUPLOT_DEPS}")

# Copy the custom CSS
set(counter 0)
foreach(dest IN ITEMS ${CUSTOM_CSS_DEST})
   file(RELATIVE_PATH rel_dest "${CMAKE_BINARY_DIR}" "${dest}")
   add_custom_target("DELETE_STOCK_CSS_${counter}"
                     DEPENDS ${GNUPLOT_DOC_FULL_TARGET}
                     COMMAND "${CMAKE_COMMAND}" -E remove "${dest}"
                     COMMENT "Deleting stock ${rel_dest}")

   add_custom_command(OUTPUT "${dest}"
                      DEPENDS "DELETE_STOCK_CSS_${counter}"
                      DEPENDS "${CUSTOM_CSS_SRC}"
                      COMMAND "${CMAKE_COMMAND}" -E copy "${CUSTOM_CSS_SRC}" "${dest}")
   math(EXPR counter "${counter} + 1")
endforeach()

add_custom_target(doc
                  DEPENDS ${GNUPLOT_DOC_FULL_TARGET} ${CUSTOM_CSS_DEST})

# Install library
install(FILES ${GNUPLOT_ARTIFACTS}
        DESTINATION lib)
