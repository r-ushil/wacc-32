#!/bin/bash

test_integration() {

	echo -e Status "\t" Exp "\t" Act "\t" Test name

	tests_total=0
	tests_passed=0
	tests_failed=0
	expected_passes=0
	expected_failures=0
	unexpected_passes=0
	unexpected_failures=0

	while read in; do 
		test_name=$(echo $in | cut -d, -f1)
		expected_exit_code=$(echo $in | cut -d, -f2)
		expected_test_status=$(echo $in | cut -d, -f3)
		./target/release/wacc_32 ./test_integration/$test_name >>  /dev/null 2>&1
		actual_exit_code=$?

		if [ $expected_exit_code -eq $actual_exit_code ]
		then
      if [ "$expected_test_status" = "pass" ]
      then
			  echo -e PASSED "\t\t\t" $test_name
			  expected_passes=$((tests_passed + 1))
      else
        echo -e PASSED "\t\t\t" $test_name "(unexpected)"
			  unexpected_passes=$((unexpected_passes + 1))
      fi

			tests_passed=$((tests_passed + 1))

		else
      if [ "$expected_test_status" = "fail" ]
      then
			  echo -e FAILED "\t" $expected_exit_code "\t" $actual_exit_code "\t" $test_name
			  expected_failures=$((expected_failures + 1))
      else
        echo -e FAILED "\t" $expected_exit_code "\t" $actual_exit_code "\t" $test_name "(unexpected)"
			  unexpected_failures=$((unexpected_failures + 1))
      fi
			tests_failed=$((tests_failed + 1))
		fi
		tests_total=$((tests_total + 1))
		
	done < test_integration/test_list_exit_codes
	
  echo "Passed (expected)   :" $expected_passes
  echo "Failed (expected)   :" $expected_failures
  echo "Passed (unexpected) :" $unexpected_passes
  echo "Failed (unexpected) :" $unexpected_failures
  echo Passing $tests_passed of $tests_total tests.

  unexpected=$((unexpected_failures + unexpected_passes))

	if [ $unexpected -gt 0 ]
  then
    echo Warning! Unexpected test results
  fi
	[[ $unexpected -eq 0 ]]
}

test_integration
