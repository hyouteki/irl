CC=g++
CFLAGS=-Wall
BUILD_NAME=irl
BUILD_DIRS=("fe" "validation")
CPP_FILES=""
TEST_FILE=eg/fib.irl

for dir in ${BUILD_DIRS[@]}; do
    for file in $(find ${dir} -name '*.cpp'); do
        CPP_FILES+="${file} "
    done
done

set -e

$CC main.cpp $CPP_FILES -o $BUILD_NAME $CFLAGS
./$BUILD_NAME $TEST_FILE
rm $BUILD_NAME
