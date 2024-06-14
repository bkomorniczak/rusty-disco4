# Laboratorium 4
## Barbara Komorniczak, Grzegorz Karkowski

### Zadanie 1
#### Funckja lfsr
Funkcja lfsr generuje pseudolosowy strumień bitów za pomocą rejestru przesuwnego z liniowym 
sprzężeniem zwrotnym (LFSR). Przechodzi przez kolejne iteracje, aktualizując stan rejestru zgodnie
z określonymi regułami sprzężenia zwrotnego, by generować nowy bit na wyjściu.
```rust
fn lfsr(initial_state: &[u8], feedback_taps: &[usize], output_length: usize) -> Vec<u8> {
let mut state = initial_state.to_vec();
let mut output = Vec::with_capacity(output_length);

    for _ in 0..output_length {
        output.push(state[state.len() - 1]);
        let new_bit = feedback_taps.iter().fold(0, |acc, &tap| acc ^ state[tap]);
        state.rotate_right(1);
        state[0] = new_bit;
    }

    output
}
```

##### Parametry
- initial_state: &[u8] - wektor bajtów reprezentujący początkowy stan rejestru. Każdy element wektora to jeden bit stanu rejestru.
- feedback_taps: &[usize] - wektor indeksów określających pozycje bitów, które są używane do obliczenia nowego bitu sprzężenia zwrotnego.
- output_length: usize - długość strumienia wyjściowego, czyli liczba bitów, które mają zostać wygenerowane.


Pierwszym etapem dzialania funkcji jest inicjalizacja zmiennych:
- state - kopia wektora `initial_state` reprrezentuje bieżący stan rejestru
- output - pusty wektor o dlugosc `output_lenght`, którym będą przechowywane bity klucza
Kolejnym etapem jest generowanie strumienia.
1. W każdej iteracji pętli ostatni bit aktualnego rejestru jest dodawany do `output`.
2. Obliczany jest nowy bit sprzężenia zwrotnego w wyniku operacji XOR wszytkich bitów określonych w `feedback_taps` -> `feedback_taps.iter().fold(0, |acc, &tap| acc ^ state[tap])`
3. Rejest przesuwany jest o jeden w prawo - > `state.rotate_right(1)`
4. Nowy bit sprzężenia zwrotnego jest umieszczany na początku rejestru -> state[0] = new_bit

Zwracany jest strumień bitów klucza

##### Przykład
Załóżmy, że mamy następujące dane wejściowe:

`initial_state = [0, 0, 0, 1]
feedback_taps = [0, 1, 3]
output_length = 5`

Iteracja 1:

Ostatni bit 1 jest dodany do output.<br>
Nowy bit new_bit = 0 ^ 0 ^ 1 = 1.<br>
Stan po przesunięciu: [1, 0, 0, 0].<br>

Iteracja 2:

Ostatni bit 0 jest dodany do output.<br>
Nowy bit new_bit = 1 ^ 0 ^ 0 = 1.<br>
Stan po przesunięciu: [1, 1, 0, 0].<br>

Iteracja 3:

Ostatni bit 0 jest dodany do output.<br>
Nowy bit new_bit = 1 ^ 1 ^ 0 = 0.<br>
Stan po przesunięciu: [0, 1, 1, 0].<br>

Iteracja 4:

Ostatni bit 0 jest dodany do output.<br>
Nowy bit new_bit = 0 ^ 1 ^ 0 = 1.<br>
Stan po przesunięciu: [1, 0, 1, 1].<br>

Iteracja 5:

Ostatni bit 1 jest dodany do output.<br>
Nowy bit new_bit = 1 ^ 0 ^ 1 = 0.<br>
Stan po przesunięciu: [0, 1, 0, 1].<br>
Po zakończeniu output = [1, 0, 0, 0, 1]<br>

### Zadanie 2
W programie są stawiane flagi, które umożliwiają łatwy wybór wektorów do testowania:

    let state_index = 1;
    let taps_index = 1;
Wynik działania algorytmu dla P(x)=1+x^2 +x^5<br>
Linear complexity: 8<br>
Connection polynomial: [1, 1, 1, 1, 0, 1, 0, 1, 0]<br>
Keystream:          [0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1]<br>
Wynik z instrukcji: [1, 0, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 0, 1, 1, 1, 0, 1]

Wynik działania algorytmu dla P(x)=1+x+x^3 +x^5<br>
Linear complexity: 15<br>
Connection polynomial: [1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]<br>
Keystream:          [0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 0, 0]<br>
Wynik z instrukcji: [1, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 1, 1]<br>

Jak widać wyniki nie są identyczne.

### Zadanie 3

#### Algorytm Berlekamp-Massey
Żeby opisać zastosowanie algorytmu w tym programie trzeba zdefiniować następujące pojęcia:

1. Wielomian połączenia - Reprezentuje on relacje między bitami w sekwencji generowanej przez rejestr przesuwający z liniowym sprzężeniem zwrotnym (LFSR). Wielomian ten pozwala na zrozumienie,
   jak bity w sekwencji są powiązane i może być używany do przewidywania przyszłych bitów na podstawie poprzednich bitów.
2. Złożoność liniowa - informuje nas, jak trudne jest przewidywanie przyszłych bitów na podstawie wcześniejszych bitów.
   Im wyższa złożoność liniowa, tym trudniejsza do złamania jest sekwencja.

```rust
fn berlekamp_massey(s: &[u8]) -> (usize, Vec<u8>) {
    let n = s.len();
    let mut c = vec![0; n]; // connection
    let mut b = vec![0; n]; //temp
    let mut t = vec![0; n]; //temp
    c[0] = 1;
    b[0] = 1;
    let mut l = 0; //zlozonosc
    let mut m = 0; // pozycja ostatniej zmiany
    let mut n_iter = 0; //pozycja w sekwencji

    while n_iter < s.len() {
        let mut d = s[n_iter];//rozbieznosc
        for i in 1..=l {
            d ^= c[i] * s[n_iter - i];  //rozbieznosc miedzy obecnym bitem a bitem z wielomiany polaczenia
        }
        if d == 1 {  //update wielomian polaczenia
            t.copy_from_slice(&c);
            for i in 0..(n_iter - m) {
                if b[i] == 1 {
                    c[i + n_iter - m] ^= 1; //update c xorujac z przezsunieta wersja b
                }
            }
            if l <= n_iter / 2 { //te updaty z algo
                l = n_iter + 1 - l;
                m = n_iter;
                b.copy_from_slice(&t);
            }
        }
        n_iter += 1;
    }

    (l, c[..=l].to_vec())
}
```

Wynik działania algorytmu dla P(x)=1+x+x^3 +x^5<br>
Linear complexity: 15<br>
Connection polynomial: [1, 1, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0]<br>

Złożoność liniowa wynosi 15. Jest to miara minimalnej długości rejestru, który może wygenerować daną sekwencje. 
Wysoka złożoność obliczeniowa oznacza, że sekwecja jest trudna do złamania, odporna na ataki.
Wielomian połączenia  można zapisać jako: C(x) = 1+x+x^3 +x^7 +x^9 +x^11 +x^13

#### Zadanie 4
Pliki wynikowe znajdują się pod ścieżką: src/resources/




