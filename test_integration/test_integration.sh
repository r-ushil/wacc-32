#!/bin/bash

total=0
passed_expected=0
passed_unexpected=0
failed_expected=0
failed_unexpected=0

function run_tests() {
  tests=$(grep -i "$2" "$1/test_list")
  if [ "$tests" = "" ]
  then
    echo Skipping $1
    return
  fi

  echo Deleting old cache for $1 tests.
  rm -rf $1/*_ours
  echo Generating actual data for $1 tests.
  (cd $1 && ./generate_test_data.sh ours)

  while read in; do 
    our_output_file=$(echo $in | cut -d, -f2)
    their_output_file=$(echo $in | cut -d, -f3) 
    expected_test_result=$(echo $in | cut -d, -f4)
  
    difference=`diff $1/$our_output_file $1/$their_output_file`
  
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
        echo "Our output above, theirs below."
        echo "$difference"
      fi
    fi
  
    total=$((total + 1))
    
  done < <(printf "%s\n" "$tests")
}

function main() {
  #run_tests ./wacc_examples_exit_codes "$1"
  # During the extension, we might want to change the assembly
  # output without failing tests, while implementing optimisations for example.
  # run_tests ./wacc_examples_assembled "$1"
  #run_tests ./wacc_examples_executed "$1"
  run_tests ./extension_executed "$1"

  echo "Passed (expected)   :" $passed_expected 
  echo "Passed (unexpected) :" $passed_unexpected
  echo "Failed (expected)   :" $failed_expected
  echo "Failed (unexpected) :" $failed_unexpected

  [[ $passed_unexpected -eq 0 && $failed_unexpected -eq 0 ]]
}

main $1



