generate_our_test_data() {
  echo Generate our test data
}

generate_their_test_data() {
  echo Generate their test data
}

if [ "$1" = "ours" ] 
then
  generate_our_test_data
elif [ "$1" = "theirs" ] 
then
  generate_their_test_data
fi

