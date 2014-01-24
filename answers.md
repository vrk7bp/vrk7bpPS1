Title: Problem Set 1 Answers
Author: Venkata-Gautam Kanumuru (vrk7bp)

1: User-Agent: Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/31.0.1650.63 Safari/537.36
	Mozilla, Chrome, and Safari all represent web browsers, but I am not sure why all three are in the User-Agent output, and also don't understand the numbers next to them. I also assume the the words in the first paranthesis represent the Operating System that is being run, which is Linux in this case.

2: I would assume that the reason Rust finds it unsafe to change a global variable will be for reasons affilitated with memory. It could be that the storage of the global variables differs from that of the rest of the program, and accessing this spot in memory can leave some potential for aspects of the code to get screwed up. This is the only intuitive answer that comes to mind, especially since languages such as Java let this go.

