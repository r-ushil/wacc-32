generate_our_test_data() {
  while read in; do 

    input_file=$(echo $in | cut -d, -f1)
    our_output_file=$(echo $in | cut -d, -f2)
    our_s_file="$our_output_file.s"

    mkdir -p $(dirname $our_output_file)

    ../../target/release/wacc_32 $input_file $our_s_file >/dev/null 2>&1
    echo Compiler exit code: $? > $our_output_file

    arm-linux-gnueabihf-gcc -static -o $our_output_file.bin -mcpu=arm1176jzf-s -mtune=arm1176jzf-s $our_s_file

    echo --------- stdout --------- >> $our_output_file
    qemu-arm-static -L /usr/arm-linux-gnueabihf/ $our_output_file.bin < inputs.txt >> $our_output_file
    exit_code=$(echo $?)
    echo -------------------------- >> $our_output_file

    echo Exit code: $exit_code >> $our_output_file

  done < ./test_list
}

if [ "$1" = "ours" ] 
then
  generate_our_test_data
fi

