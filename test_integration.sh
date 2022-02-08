#!/bin/bash

test_integration() {

	while read in; do 
		test_name=$(echo $in | cut -d, -f1)
		expected_exit_code=$(echo $in | cut -d, -f2)
		./target/release/wacc_32 ./test_integration/$test_name >> /dev/null
		actual_exit_code=$?

		echo $test_name
		echo $expected_exit_code
		echo $actual_exit_code
		
	done < test_integration/test_list_exit_codes
}

test_integration
