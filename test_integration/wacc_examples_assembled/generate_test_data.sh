generate_our_test_data() {
  while read in; do 

    input_file=$(echo $in | cut -d, -f1)
    our_output_file=$(echo $in | cut -d, -f2)

    mkdir -p $(dirname $our_output_file)
    ../../target/release/wacc_32 $input_file $our_output_file >/dev/null 2>&1

  done < ./test_list
}

generate_their_test_data() {
  while read in; do 
    input_file=$(echo $in | cut -d, -f1)
    their_output_file=$(echo $in | cut -d, -f3)

    mkdir -p $(dirname $their_output_file)
    python3 ../apps/refCompileSimple.py $input_file $their_output_file > /dev/null 2>&1

  done < ./test_list
}

if [ "$1" = "ours" ] 
then
  generate_our_test_data
elif [ "$1" = "theirs" ] 
then
  generate_their_test_data
fi

