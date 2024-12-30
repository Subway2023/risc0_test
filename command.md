# compile program
## risc0
cd /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/methods
cargo build

## sp1
cd /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/program
cargo prove build


<!-- 
risc0 program path: 
/mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/riscv-guest/methods/program_risc0/riscv32im-risc0-zkvm-elf/release/program_risc0


sp1 program path: /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program_sp1

 -->

# diff test run
/mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/debug/host execute /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/riscv-guest/methods/program_risc0/riscv32im-risc0-zkvm-elf/release/program_risc0 /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/elf-compilation/riscv32im-succinct-zkvm-elf/release/program_sp1 /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/data


/mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/target/debug/host execute /mnt/sdd1/sbw/zkvm/poc/test.elf /mnt/sdd1/sbw/zkvm/poc/test.elf /mnt/sdd1/sbw/zkvm/diffTest_back/differentRun/data

