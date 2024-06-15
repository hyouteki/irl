CC=g++
CFLAGS=-Wall
BUILD_NAME=irl
BUILD_DIRS=("fe")
CPP_FILES=""
TEST_FILE=eg/iden.irl

for dir in ${BUILD_DIRS[@]}; do
    for file in ${dir}/*.cpp; do
        if [ -e ${file} ]; then
            CPP_FILES+="${file} "
        fi
    done
done

set -e

$CC main.cpp $CPP_FILES -o $BUILD_NAME $CFLAGS
./$BUILD_NAME $TEST_FILE
rm $BUILD_NAME
