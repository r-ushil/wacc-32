generate_our_test_data() {
  while read in; do 

    input_file=$(echo $in | cut -d, -f1)
    our_output_file=$(echo $in | cut -d, -f2)
    their_output_file=$(echo $in | cut -d, -f3)
    stdin_src="$their_output_file.stdin"
    our_s_file="$our_output_file.s"

    mkdir -p $(dirname $our_output_file)

    echo "Compiling and executing $input_file"

    ../../target/release/wacc_32 $input_file $our_s_file >/dev/null 2>&1

    arm-linux-gnueabihf-gcc -static -o $our_output_file.bin -mcpu=arm1176jzf-s -mtune=arm1176jzf-s $our_s_file

    echo --------- stdout --------- > $our_output_file
    cat $stdin_src 1> $stdin_src.txt 2>/dev/null
    cat inputs.txt >> $stdin_src.txt
    qemu-arm-static -L /usr/arm-linux-gnueabihf/ $our_output_file.bin < $stdin_src.txt >> $our_output_file
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
    arm-linux-gnueabihf-gcc -static -o $their_output_file.bin -mcpu=arm1176jzf-s -mtune=arm1176jzf-s $input_file

    echo --------- stdout --------- > $their_output_file
    qemu-arm-static -L /usr/arm-linux-gnueabihf/ $their_output_file.bin < inputs.txt >> $their_output_file
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

