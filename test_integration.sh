#!/bin/bash

test_integration() {

	echo -e Status "\t" Exp "\t" Act "\t" Test name

	tests_total=0
	tests_passed=0

	while read in; do 
		test_name=$(echo $in | cut -d, -f1)
		expected_exit_code=$(echo $in | cut -d, -f2)
		./target/release/wacc_32 ./test_integration/$test_name >> /dev/null
		actual_exit_code=$?

		if [ $expected_exit_code -eq $actual_exit_code ]
		then
			echo -e PASSED "\t\t\t" $test_name
			tests_passed=$((tests_passed + 1))
		else
			echo -e FAILED "\t" $expected_exit_code "\t" $actual_exit_code "\t" $test_name
		fi
		tests_total=$((tests_total + 1))
		
	done < test_integration/test_list_exit_codes
	
	echo Passing $tests_passed of $tests_total tests

	[[ $tests_passed -eq $tests_total ]]
}

test_integration
