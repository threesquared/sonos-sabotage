set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    cross run --target $TARGET
    cross run --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
