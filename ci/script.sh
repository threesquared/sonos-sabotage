set -ex

main() {
    cross build --target $TARGET
    cross build --target $TARGET --release

    cross test --target $TARGET
    cross test --target $TARGET --release
}

# we don't run the "test phase" when doing deploys
if [ -z $TRAVIS_TAG ]; then
    main
fi
