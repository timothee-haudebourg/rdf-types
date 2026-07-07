generate-iso-tests:
    tests/utils/generate-iso-test.rb > tests/iso.rs
    tests/utils/generate-iso-test.rb --negative > tests/iso-negative.rs