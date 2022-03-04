generate_our_test_data() {
  while read in; do 

    input_file=$(echo $in | cut -d, -f1)
    our_output_file=$(echo $in | cut -d, -f2)

    mkdir -p $(dirname $our_output_file)

    echo --------- stdout --------- > $our_output_file
    cat $input_file >> $our_output_file
    exit_code=$(echo $?)
    echo -------------------------- >> $our_output_file

    echo Exit code: $exit_code >> $our_output_file

  done < ./test_list
}

generate_their_test_data() {
  while read in; do 
    input_file=$(echo $in | cut -d, -f1)
    their_output_file=$(echo $in | cut -d, -f3)

    mkdir -p $(dirname $their_output_file)

    echo --------- stdout --------- > $their_output_file
    cat $input_file >> $their_output_file
    exit_code=$(echo $?)
    echo -------------------------- >> $their_output_file

    echo Exit code: $exit_code >> $their_output_file

  done < ./test_list
}

if [ "$1" = "ours" ] 
then
  generate_our_test_data
elif [ "$1" = "theirs" ] 
then
  generate_their_test_data
fi

