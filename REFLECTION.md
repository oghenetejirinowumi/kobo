Today is the 19th of September, 2026. It's a saturday evening and i have just read the CA 1 brief. From reading the brief i have identified the core tasks of this continuous assessment is to write 5 functions in the src/scanner.rs:

1. run
2. scan_token
3. string
4. number
5. identifier

I think my best approach to this project is to constantly update this "REFLECTION.md" document with my observation, challenges and solutions as i go.

Sir please i will be using strings of "equal to" (=) symbols to section my documentation for better readability.

=====================================
1. run
=====================================

The driver of the entire scanner. from my understanding run is basically the function which will go through every line of the source code for tokens.

I need a condition to check if the function has reached the end of files, if it has it will stop if it has not it will continue.
I need a mechanism in place to know where the start and end of a token is.
I need to scan each token until the source code is run out.
Then i will add an EOF token when i am done.

This function: run, needs to have &mut self because it changes things about the scanner in the function like start, current, line, tokens and errors.

I have started with a while loop to hold the condition. I have marked the beginning of a token to be equal to the current which is the start when we first scan.

I have made an observation that run has to call the function scan_token, so scan_token must exist for run to be tested. 

=====================================
2. scan_token
=====================================

scan_token is the brain of the scanner. It is where every token decision is made.
I am declarating a variable c with the method call for the advance function so the scan_token function can read the current character and advance self.current by one, this way the scanner moves past that character.

I am using Rust's match key word to identify and classify words as their token types.
I have matched patterns to actions for token types for PUNCTUATION and ARITHMETIC symbols.
For EQUALITY, COMPARISON, NEGATION & ASSIGNMENT operators / symbols i am using an if statement to make sure the scanner is not at the EOF (src.len() != 0) and i am peeking so i see the self.current without advancing. I am then assigning token types to each token (SLASH, BangEqual, Bang, EqualEqual, Equal, LessEqual, Less, GreaterEqual, Greater)

=====================================
2. string
=====================================
When scan_token sees a '"' it calls self.string(). self.start points at the opening '"'. self.current will point at the one jsut after since at the begining i already did self.advance(). I will consider self.line() because strings can span multiple lines and the self.line() must increment. When the opening '"' is scanned it looks for the closing '"' or it goes to the EOF.

=====================================
3. number
=====================================
I am using a while loop to check the current character the scanner is on and ensure it is a digit between [0..9] and after it checks if there is a decimal point (.) and ensures that a number follows the decimal point fulfulling the rule for a VALID NUMBER.
=====================================
4. Identifier
=====================================
I am writing a function to decide if lexemes are ordinaly words or reserved keywords. The scanner consumes the whole word and then decides the token type but looking up the word in the KEYWORD TABLE. If it is a keyword, it will emit the keyword's TOKEN. Otherwise, emit IDENTIFIER.

