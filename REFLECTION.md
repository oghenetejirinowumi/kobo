The reflection

==========================================================================================================================

QUESTION 1:

1. Cite the line where you decide that a . begins a fractional part rather than being a stray
character. What does your scanner do with the input 5., and why is that what section 1.4
requires?

ANSWER:
scanner.rs - Line 156 

if self.peek() == '.' && self.peek_next().is_ascii_digit()

This checks both peek() (the dot) and peek_next() (the digit after the dot).
For 5.: after reading 5, peek() returns '.' but peek_next() returns 
a space, a newline, or EOF. That is not a digit, so the condition is false. The dot is not consumed. NUMBER '5' is emitted, then the next call to scan_token sees the . and reports Character is not part of any token.

Section 1.4 says a number is digits, optionally followed by . and one or more digits. A lone dot after digits doesn't fit, so leaving it for the error is exactly what the spec requires.

==========================================================================================================================

QUESTION 2:

2. Cite every line where you change the line counter. Explain which line your EOF token ends
up carrying for a file that ends in two blank lines, and why section 6.1 asks for that rather than
the file’s last line.

scanner.rs - Line 117 
self.line += 1

This handles new lines between tokens

scanner.rs - Line 135
self.line += 1;

This handles new lines inside a multi-line strings

The EOF line is set in by: scanner.rs - Line 38

For a file ending in two blank lines, the two \n characters increment self.line to 3, but self.tokens.last() still points at the last real token.

Section 6.1 requires this because a trailing newline must not change the token stream. The spec chooses the last real token's line to make the output stable.

==========================================================================================================================

QUESTION 3:

3. Nameonetestintests/phase-1/thatyoufailedatsomepoint. Citethelineyouchanged
to fix it, and say plainly what you had misunderstood. “I made a typo” is not an answer; “I was
incrementing line before adding the token, so the token was reported on the next line” is.

ANSWER:
peek & peek_next in the dot check.
Decimal numbers example: 12.75 was being scanned as NUMBER '12' followed by an error on the ., because the condition used self.peek() for both the lower and upper bounds of the digit check.

Fix: Changed self.peek() <= '9' to self.peek_next() <= '9'.

==========================================================================================================================



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
=====================================
4. Identifier
=====================================
