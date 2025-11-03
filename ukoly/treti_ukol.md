# Řešení otázek
### Jméno: AdamTriska
#### Binární zápis:  
01000001 01100100 01100001 01101101 01010100 01110010 01101001 01110011 01101011 01100001

#### Prvních 48 bitů:  
01000001 01100100 01100001 01101101 01010100 01110010

#### Dalších 16 bitů pro klíč:
0110100101110011

#### Inicializační vektor
0000000000000001

#### Bloky pro šifrování:
m1: 0100000101100100  
m2: 0110000101101101  
m3: 0101010001110010  

# ECB
ECB je jednoduché blokové šifrování, kde je každý blok zašifrován samostatně za použití jednotného klíče.

y1: 0111011011011100  
y2: 0000011011011101  
y3: 1111110110110011  

CT: 01110110 11011100 00000110 11011101 11111101 10110011

# CBC
CBC před samotným šifrováním použije exklusivní součet na vstupní blok a výstup z předchozího šifrování, výsledek této operace následně zašifruje.

m1 xor: 0100000101100101  
y1: 0011011011011001  

m2 xor: 0101011110100100  
y2: 1101100010111101  

m3 xor: 1000110011001111  
y3: 0000011100001101  

CT: 00110110 11011001 11011000 10111101 00000111 00001101

# CFB
V CFB neprochází šifrováním samotný otevřený text, ale výstup z minulého kola, otevřený text je následně xorován s takto vygenerovaným blokem.

E(k,IV): 0101101100100001  
y1: 0001101001000101 

E(k,y1): 0010111110111010  
y2: 0100111011010111

E(k,y2): 1010101111000111  
y3: 1111111110110101

CT: 00011010 01000101 01001110 11010111 11111111 10110101

# OFB
OFB funguje prakticky stejně jako CFB, ale v samotném řetězení se používá pouze zašifrovaný inicializační vektor bez naxorovaného otevřeného textu, výhodou tohoto přístupu je možnost připravit si tyto vektory předem a následně je už pak kombinovat s bloky otevřeného textu pomocí xoru.

s1: 0101101100100001  
s2: 1010100000110110  
s3: 0010110101100000  

y1: 0001101001000101  
y2: 1100100101011011  
y3: 0111100100010010  

CT: 00011010 01000101 11001001 01011011 01111001 00010010

# CTR
CTR šifruje inicializační vektor, ke kterámu přičte pořadí momentální iterace, následně je k tomuto bloku naxorován blok otevřeného textu náležící dané iteraci. Stejně jako u OFB, i zde si můžeme zašifrovaná IV připravit předem.

s1: 0101101100100001  
s2: 1111101100101101  
s3: 1110101100101001  

y1: 0001101001000101  
y2: 1001101001000000  
y3: 1011111101011011  

CT: 00011010 01000101 10011010 01000000 10111111 01011011  

# CTS
CTS je stejné jako CBC, až na zpracování posledních dvou bloků, kde jsou k poslednímu neůplnému bloku přidány nuly. následně jsou také prohozeny poslední dva výsledné zašifrované bloky a z předposledního bloku je použita pouze tolik bitů, kolik bylo v posledním bloku otevřeného textu.

Změněný m3: 010101000111 0000

m1 xor: 0100000101100101  
y1: 0011011011011001  

m2 xor: 0101011110100100  
y2: 1101100010111101  
y'1: 110110001011  
y''2: 1101  

m3 xor: 1000110011001101  
y3: 1010111000011010  

CT: 00110110 11011001 10101110 00011010 11011000 1011