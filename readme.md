# Phone Keypad Encoder Decoder
Encodes a text to phone key presses, decodes key presses string to actual text.

```
1      2 abc  3 def
4 ghi  5 jkl  6 mno
7 pqrs 8 tuv  9 wxyz
       0 <spc>
```
## Usage
- Encoding:
    $ PhoneKeypadEncoderDecoder encode <plain_text>

- Decoding:
    $ PhoneKeypadEncoderDecoder decode <encoded_text>