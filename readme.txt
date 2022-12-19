My name is Yann, I work individually

I have received help from Dr. Daniels documentations provided on the assginment handout as well as the rust book especially chapters 15 on boxes as well as the rust library.
Dr. Daniels and some TA's helped me to confirm my understanding of the assignemnt

I don't know for sure what is correctly implemented but my program seem to work and so everythink seems to be decently implemented

Compared to my design, my program counter is not a box pointer to a word but just a regular number counter as it was much easier for me to implement

achitecture and modules, abstraction implemented, secret known
- main.rs --secrets "machine.rs::UM" + "dinst.rs::Dinst" + "rumload.rs"
- rumload.rs // read instructions from a file
	- dinst.rs // UM instruction parser and machine executer (struct) --secret "machine"
	- machine.rs // UM (struct) --secret "memory + register + error"
		- memory.rs // UM memory (struct)
		- registers.rs // UM registers/CPU (struct)
	- error.rs // UM error handler (struct)
	
time to execute 50 millions instructions: 46.93 seconds
my rum took 79.84 seconds to execute 85,070,522 instructions (midmark.um number of instructions I've calculated using a counter inside of the loop in main)
Therefore for 50 millions instructions it takes:
	50,000,000 * 79.84 / 85.070.522 = 46.93 seconds
I've used Instant:now() before the first instruction (before the loop in my main) then  I've printed now.elapsed() right before the machine halts by extiting 0

I have spent about 4 hours analyzing the assignment and doing the design simultaneously. 
But I should actually say more than 5 because I did spend some time confirming my understanding with my professor

Then it took me 4 hours to finish my implementation and about 30 minitues to debug my code

I've spend some extra time improving how my code looks, how it is organize as well as testing and slightly improving the speed slightly from 112 seconds on midmark.um to 77 seconds.
I feel like I should be able to increase the speed by a lot by modifying my implementation of memory with a segmented memory being a vec of vec or just a regular 1 dimentional vector using my array2.

