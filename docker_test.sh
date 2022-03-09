docker build -t test_integration --target test_integration .
docker run test_integration $1
