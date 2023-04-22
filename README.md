# psearch

very simple search tool for byte patterns in files.

Example File:

```
# testfile/testfile.hex

00000000: 1dd8 8444 0cd2 9a0c 0165 8f6d 1aa6 0e48  ...D.....e.m...H
00000010: 7454 abd8 5d70 e9ea 769a 0462 3063 39c8  tT..]p..v..b0c9.
00000020: 366c 4d0d d6a9 92df 3a4c 956c 7a8b beee  6lM.....:L.lz...
00000030: cde2 6f4c 8215 235d c64e baf4 d953 26f8  ..oL..#].N...S&.
00000040: b258 886d 9875 f504 10d5 5d19 b154 954a  .X.m.u....]..T.J
00000050: d149 ba8f 7468 ff0e cd47 c5a7 9eb0 e36b  .I..th...G.....k
00000060: 527d 7608                                R}v.
```

Usage:

```
# put the search pattern(s) in a file
# you can have as many files as you want
# one file may contain multiple patterns
# if you can replace single bytes with "?" as a wildcard

$ cat patterns/testpattern.pat 
b2 58 ? 6d

$ ./psearch
usage: ./psearch [pattern directory] [file-to-search]
       ./psearch -p "10 20 30 ?" [file-to-search]

$ ./psearch patterns testfile/testfile.hex
[+] reading patterns from directory patterns
[+] reading pattern from file patterns/testpattern.pat


[+] starting search...


[*] searching for pattern patterns/testpattern.pat
[FOUND] pattern patterns/testpattern.pat ([Byte(b2), Byte(58), WildCard, Byte(6d)]) found at offsets [40]

# enter a pattern directly for a quick search
$ ./psearch -p "be ee ? ? 6f" testfile/testfile.hex
[FOUND] pattern found at offsets [2e]
```

Applications:

- add cryptographic constants to a pattern file, for example MD5 constants, use psearch as a crypto identification tool
- search for mnemonics
- search for significant byte patterns in malware (IOCs)

## 0xca7
