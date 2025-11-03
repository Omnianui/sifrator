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
ECB je jednoduché šifrování, kde je každý blok zašifrován samostatně za použití jednotného klíče.

c1: 0111011011011100  
c2: 0000011011011101  
c3: 1111110110110011  

# CBC
CBC před samotným šifrováním použije exklusivní součet na vstupní blok a výstup z předchozího šifrování, výsledek této operace následně zašifruje.

m1 xor: 
c1: 
c2: 
c3: 
# CFB
CFB

c1:
c2:
c3:
# OFB
OFB

c1:
c2:
c3:
# CTR
CTR

c1:
c2:
c3:

# CTS
CTS

Změněný m3: 111111011011

c1:
c2:
c3: