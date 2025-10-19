# Řešení otázek
### Jméno: AdamTriska
#### Binární zápis:  
01000001 01100100 01100001 01101101 01010100 01110010 01101001 01110011 01101011 01100001

#### Prvních 40 bitů:  
01000001 01100100 01100001 01101101 01010100
#### Dalších 50 bitů:
0111001001 1010010111 0011011010 1101100001 0100000101

### Jednotlivé texty a klíče byly odvozeny jednoduchým rozsekáním na části dané dělky

### Originální texty a klíče

m1: 01000001  
m2: 01100100  
m3: 01100001  
m4: 01101101  
m5: 01010100  

k1: 0111001001  
k2: 1010010111  
k3: 0011011010  
k4: 1101100001  
k5: 0100000101

### Bitové komplementy

m'1: 10111110  
m'2: 10011011  
m'3: 10011110  
m'4: 10010010  
m'5: 10101011  

k'1: 1000110110  
k'2: 0101101000  
k'3: 1100100101  
k'4: 0010011110  
k'5: 1011111010

### Zašifrované texty

c1: 01010100  
c2: 00110110  
c3: 00101000  
c4: 11111011  
c5: 00000010  

cc1: 10101011  
cc2: 11001001  
cc3: 11010111  
cc4: 00000100  
cc5: 11111101

# Rozbor otázek

### 1. Otázka
    Každé ccj je binárním komplementem cj, protože platí cj ⊕ ccj = 11111111.

### 2. Otázka
    Rundovní klíč pro druhou rundu výpočtu bude 10001100. 
    
    Postup:
    1. Permutace P10 pro 0100000101 -> 0010010010
    2. Rozdělení na dva bloky 0010010010 -> 00100 | 10010
    3. Posun o tři bity 00100 | 10010 -> 00001 | 10100
    4. Permutace P8 pro spojené bloky 0000110100 -> 10001100

### 3. Otázka
    Výstup S1 S-boxu pro řetězec 1101 bude 0000, protože první a poslední bity označují poslední řádek a vnitřní bity označují druhý sloupec.