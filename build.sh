#!/bin/sh

#
# Triaka Core SDK build script
#

# Global constants
STABLE="triaka-compiler"
UNSTABLE=""
SRCROOT="$(pwd)"
TEMP_DIR="$SRCROOT/target"
SCRIPT_VERSION="1.0.0"
METAFILES="README.md LICENSE SDK_VERSION"

# Prints help information of the build script
help() {
    echo
    echo "Triaka Core SDK build script"
    echo
    echo "Usage:"
    echo "  help                Prints this help information."
    echo "  build               Builds the SDK."
    echo "  make-image          Makes a .tar.gz archive for the SDK."
    echo "  make-appactor       Makes an AppActor package for the SDK."
    echo "  clean               Cleans the build."
    echo "  version             Prints the version information."
    echo
    echo
    echo "Configuration:"
    echo "  By default, only stable components will be built. To enable unstable components, set environment variable:"
    echo
    echo "      TRIAKA_SDK_UNSTABLE=1"
    echo
    echo "  To pass extra flags to cargo via environment variable 'CARGOFLAGS'. Convetional environment variables, like 'RUSTFLAGS', 'CC', 'CFLAGS' are also available."
    echo
    echo "  To build an AppActor package, the AppActor SDK is required. Specify the path of the AppActor SDK via environment variable 'APPACTOR_SDK_ROOT', or the script will attempt to find the SDK automatically."
    echo
}

# Makes an AppActor package for the SDK.
make_appactor() {
    echo "CRITICAL:"
    echo "  Function not implemented."
    echo
    echo "Reason:"
    echo "  AppActor is not released yet."
    exit 2
}

# Builds the project.
build() {
    if [ "$TRIAKA_SDK_UNSTABLE" = "1" ]; then
	for c in $UNSTABLE; do
	    cd $c
	    cargo build $CARGOFLAGS
	    cd ..
	done
    fi
    for i in $STABLE; do
	cd $i
	cargo build --release $CARGOFLAGS
	cd ..
    done
}

# Makes an image of the SDK.
make_image() {
    build

    rm -rf $TEMP_DIR

    mkdir -p $TEMP_DIR/triaka-core-sdk
    mkdir $TEMP_DIR/triaka-core-sdk/bin

    if [ "$TRIAKA_SDK_UNSTABLE" = "1" ]; then
	for c in $UNSTABLE; do
	    cd $c
	    cp target/debug/$c $TEMP_DIR/triaka-core-sdk/bin/
	    cd ..
	done
    fi
    for i in $STABLE; do
	cd $i
	strip target/release/$i
	cp target/release/$i $TEMP_DIR/triaka-core-sdk/bin/
	cd ..
    done

    for i in $METAFILES; do
	cp $i $TEMP_DIR/triaka-core-sdk
    done

    cd $TEMP_DIR/
    tar cf triaka-core-sdk.tar triaka-core-sdk
    gzip triaka-core-sdk.tar

    echo "Successfully built target/triaka-core-sdk.tar.gz image."
}

# Cleans the build.
clean() {
    for i in $STABLE $UNSTABLE; do
	cd $i
	cargo clean $CARGOFLAGS
	cd ..
    done
}

# Prints the version information.
version() {
    cat SDK_VERSION
}

# Entrypoint of the script
main() {
    if [ "$1" = "help" ]; then
        help
    elif [ "$1" = "build" ]; then
	build
    elif [ "$1" = "clean" ]; then
	clean
    elif [ "$1" = "make-image" ]; then
	make_image
    elif [ "$1" = "make-appactor" ]; then
	make_appactor
    elif [ "$1" = "version" ]; then
	version
    else
	echo "CRITICAL:"
	echo "  Either no module was specified, or the module was not found."
	echo
	echo "Help:"
	echo "  Run './build.sh help' for help."
	exit 1
    fi
}

# Calls `main`
main $@
