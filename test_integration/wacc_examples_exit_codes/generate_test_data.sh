generate_our_test_data() {
  while read in; do 

    input_file=$(echo $in | cut -d, -f1)
    our_output_file=$(echo $in | cut -d, -f2)

    mkdir -p $(dirname $our_output_file)
    ../../target/release/wacc_32 $input_file $our_output_file --analysis >/dev/null 2>&1
    echo $? > $our_output_file

  done < ./test_list
}

generate_their_test_data() {
  while read in; do 
    input_file=$(echo $in | cut -d, -f1)
    their_output_file=$(echo $in | cut -d, -f3)

    valid=`echo $input_file | grep -v 'syntaxErr\|semanticErr' | wc -l`
    syntaxErr=`echo $input_file | grep syntaxErr | wc -l`
    semanticErr=`echo $input_file | grep semanticErr | wc -l`

    if [ $valid -eq 1 ] 
    then
      exit_code=0
    elif [ $syntaxErr -eq 1 ] 
    then
      exit_code=100
    elif [ $semanticErr -eq 1 ] 
    then
      exit_code=200
    fi

    mkdir -p $(dirname $their_output_file)
    echo $exit_code > $their_output_file

  done < ./test_list
}

if [ "$1" = "ours" ] 
then
  generate_our_test_data
elif [ "$1" = "theirs" ] 
then
  generate_their_test_data
fi

