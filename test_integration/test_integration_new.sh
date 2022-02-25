#!/bin/bash

test_target=./wacc_examples_exit_codes

total=0
passed_expected=0
passed_unexpected=0
failed_expected=0
failed_unexpected=0

while read in; do 
  our_output_file=$(echo $in | cut -d, -f2)
  their_output_file=$(echo $in | cut -d, -f3)
  expected_test_result=$(echo $in | cut -d, -f4)

  difference=`diff $test_target/$our_output_file $test_target/$their_output_file`


  if [ `echo $?` -eq 0 ]
  then
    if [ "$expected_test_result" = "pass" ]
    then
      passed_expected=$((passed_expected + 1))
      echo Passed $their_output_file
    else
      passed_unexpected=$((passed_unexpected + 1))
      echo "Passed $their_output_file (unexpected)"
    fi
  else
    if [ "$expected_test_result" = "fail" ]
    then
      failed_expected=$((failed_expected + 1))
      echo "Failed! $their_output_file"
    else
      failed_unexpected=$((failed_unexpected + 1))
      echo "Failed! $their_output_file (unexpected)"
    fi
    echo "Our output above, theirs below."
    echo "$difference"
  fi

  total=$((total + 1))
  
done < $test_target/test_list

echo "Passed (expected)   :" $passed_expected 
echo "Passed (unexpected) :" $passed_unexpected
echo "Failed (expected)   :" $failed_expected
echo "Failed (unexpected) :" $failed_unexpected
