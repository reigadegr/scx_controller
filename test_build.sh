cargo fmt
rm -rf output

python3 ./make.py build --debug --nightly || clear_crash


