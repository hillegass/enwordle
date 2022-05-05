# enwordle
Using information theory, this is the optimal wordle player

When you pick a word in Wordle, the game responds with 1 of 243 responses.  There are five space and three possible responses for each (not present, wrong place, right place). 5^3 = 243.  You get the most information about the word if each response is equally likely — that is, in the language of information theory, the response has the most possible entropy.

At each step, this program figures out the word that evoke the most entropy from Wordle.  It even tells you how many bits of information you are likely to get from the game.

To run

```
cargo run --release
```

Example: playing for "brain":

```
Read 12947 words. Making table...
12947 possibilities
	1 tares: 6.20 bits
	2 lares: 6.15 bits
	3 rales: 6.12 bits
	4 rates: 6.10 bits
	5 teras: 6.08 bits
	6 nares: 6.07 bits
	7 soare: 6.06 bits
	8 tales: 6.06 bits
	9 reais: 6.05 bits
What word did you pick? (0 to quit)
4
What did wordle return? (0 = not present, 1 = wrong place, 2 = right place)
11000
306 possibilities
	1 brail: 4.43 bits
	2 drail: 4.41 bits
	3 grail: 4.40 bits
	4 brain: 4.38 bits
	5 groan: 4.38 bits
	6 coria: 4.36 bits
	7 moria: 4.36 bits
	8 drain: 4.36 bits
	9 aroid: 4.35 bits
What word did you pick? (0 to quit)
1
What did wordle return? (0 = not present, 1 = wrong place, 2 = right place)
22220
2 possibilities
	1 brain: 1.00 bits
	2 braid: 1.00 bits
```