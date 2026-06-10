# Tutorial: jak najlepiej nagrać prezentację confrisk

Praktyczny przewodnik nagrania prezentacji (16 slajdów + demo na żywo) — od przygotowania,
przez scenariusz slajd-po-slajdzie i komendy demo, po montaż i checklistę. Cel: **12–15 minut**,
spójny obraz i dźwięk, czytelne demo terminala i raportu HTML z auto-fixem.

---

## 1. Co właściwie nagrywamy

Najlepszy efekt daje **hybryda**: slajdy + wstawka z żywym demem narzędzia.

```mermaid
flowchart LR
    A[Slajdy 1–11<br/>narracja] --> B[Demo na żywo<br/>terminal + raport HTML]
    B --> C[Slajdy 12–16<br/>testy, wnioski]
    B -. nagrane wcześniej .-> D[wstawka wideo<br/>na slajdzie „Demo"]
```

Dwa warianty demo:
- **A — na żywo podczas nagrania** (płynne, ale ryzyko wpadki). Zalecane, jeśli przećwiczone.
- **B — osobny klip wklejony na slajd „Demo"** (slajd 14). Bezpieczniejsze; demo nagrywasz raz,
  do skutku, i wstawiasz. *Patrz sekcja 6.*

> Wskazówka: nawet robiąc demo na żywo, **nagraj też klip zapasowy** (wariant B). Jeśli na żywo coś
> się posypie, w montażu podmieniasz fragment. Masz też gotowe `docs/sample-reports/*.html` jako
> awaryjny obraz raportów.

---

## 2. Sprzęt i oprogramowanie

| Element | Zalecenie |
|---------|-----------|
| **Rejestrator ekranu** | **OBS Studio** (darmowy, najlepsza kontrola) lub QuickTime (macOS) / wbudowane nagrywanie w PowerPoint |
| **Rozdzielczość / canvas** | 1920×1080 (1080p), 30 fps |
| **Mikrofon** | zewnętrzny / słuchawkowy zamiast wbudowanego; nagrywaj w cichym pomieszczeniu |
| **Audio** | format mono, redukcja szumu (OBS: filtr Noise Suppression) |
| **Webcam (opcjon, „talking head")** | mały kadr w rogu — buduje zaangażowanie, ale nie zasłaniaj treści |

**Nagrywanie wbudowane w PowerPoint** (najprostsze): zakładka *Pokaz slajdów → Nagraj pokaz*.
Nagrywa narrację + przejścia per slajd i pozwala wyeksportować do wideo (*Plik → Eksportuj →
Utwórz wideo*, 1080p). Minus: trudniej wpleść żywe demo terminala — wtedy i tak użyj OBS.

---

## 3. Przygotowanie środowiska (zanim klikniesz „nagrywaj")

**Ekran i pulpit**
- Ustaw rozdzielczość ekranu na 1920×1080 (lub skaluj okno przeglądarki/terminala do 16:9).
- Wyczyść pulpit, wycisz powiadomienia (macOS: tryb skupienia / Do Not Disturb).
- Zamknij zbędne karty i aplikacje; pokaż tylko: prezentację, terminal, przeglądarkę.

**Terminal (kluczowe dla czytelności!)**
- Powiększ czcionkę do **16–20 pt** — widz musi przeczytać komendy.
- Jasny, wysoki kontrast; krótki prompt (ukryj długą ścieżkę).
- Przygotuj komendy w pliku tekstowym do skopiowania — nie wpisuj ich na ślepo.

**Projekt**
- Zbuduj wcześniej (`cargo build --release`), żeby nie nagrywać kompilacji.
- Ustaw konfigurację raz: `export CONFRISK_CONFIG_DIR="$PWD/config"`.
- Miej otwarte gotowe raporty z `docs/sample-reports/` jako plan B.

---

## 4. Scenariusz slajd-po-slajdzie (≈13 min)

Deck ma 16 slajdów. Orientacyjne czasy — dostosuj do własnego tempa (~40–50 s/slajd + 3–4 min demo).

| # | Slajd | Czas | Co powiedzieć / pokazać |
|---|-------|------|--------------------------|
| 1 | Tytuł | 0:30 | Przywitanie, jedno zdanie: „confrisk — kontekstowa ocena ryzyka bezpieczeństwa". |
| 2 | Agenda | 0:30 | Zapowiedz 6 bloków; nie czytaj wszystkiego. |
| 3 | Motywacja | 1:00 | Ataki npm 2025 (qix, Shai-Hulud) — realny powód powstania. |
| 4 | Problem | 0:45 | Klasyczne skanery: sama dotkliwość ≠ ryzyko. |
| 5 | Cel i założenia | 0:45 | Trzy filary: kontekst, JSON-config, objaśnialność. |
| 6 | Architektura | 1:00 | Biblioteka + 3 binarki; wspólny sterownik CLI. |
| 7 | Model ryzyka | 1:00 | Wzór `risk = severity × asset × exposure × confidence`; pasma. |
| 8 | Przykład kontekstowy | 0:45 | Ten sam SSH: dev 4.95 → prod 10.9 → crown-jewel 12.9. |
| 9 | Konfiguracja | 0:45 | `CONFRISK_CONFIG_DIR` + kolejność rozstrzygania. |
| 10 | Skanery | 0:45 | npm i gradle, wspólny trait `Scanner`. |
| 11 | Separacja kodu | 0:45 | trait + `cli::run` → binarka ≈ 13 linii. |
| — | **DEMO NA ŻYWO** | **3:30** | *Patrz sekcja 5.* Przełącz na terminal/przeglądarkę. |
| 12 | Wyniki | 0:45 | Formaty text/json/html; kody wyjścia w CI. |
| 13 | Raport HTML | 1:00 | Jak powstaje; wspólny dla 3 skanerów; przycisk „Napraw". |
| 14 | Demo (wideo) | — | Jeśli wariant B: tu odtwarza się wklejony klip. |
| 15 | Testy | 0:30 | `cargo test`, decyzje projektowe. |
| 16 | Wnioski | 0:45 | Podsumowanie + kierunki rozwoju. Podziękowanie. |

> Demo umieść po slajdzie 11 (po „jak to zbudowane") i przed „Wyniki" — widz najpierw rozumie
> koncept, potem widzi go w akcji, a slajdy 12–13 podsumowują to, co zobaczył.

---

## 5. Demo na żywo — dokładny scenariusz i komendy

Czas ~3:30. Mów wolno, rób pauzę po każdej komendzie, by widz przeczytał wynik.

**0. Reset (przed nagraniem, poza kadrem)**
```sh
cd confrisk
cargo build --release
export CONFRISK_CONFIG_DIR="$PWD/config"
```

**1. Skan zależności npm → kody wyjścia w CI (~50 s)**
```sh
# Projekt z podatnymi pakietami z blocklisty
confrisk-npm --path examples/npm-vuln-demo --fail-on high --exit-code; echo "exit=$?"
```
Powiedz: „kod wyjścia ≠ 0 — w git hooku albo CI to **blokuje commit / build**".

**2. Wyjście JSON dla pipeline'u (~25 s)**
```sh
confrisk-gradle --path examples/gradle-project-demo --format json | head -n 25
```
Powiedz: „gotowe do parsowania — id, severity, risk, priority, risk_band".

**3. Raport HTML — sedno demo (~90 s)**
```sh
confrisk-npm    --path examples/npm-vuln-demo      --format html --out raport-npm.html
confrisk-gradle --path examples/gradle-project-demo --format html --out raport-gradle.html
confrisk        --asset production                  --out raport-os.html
open raport-npm.html      # macOS (Linux: xdg-open)
```
W przeglądarce pokaż:
- jasny, czytelny układ; status i podsumowanie liczb;
- **kliknij rozwijany finding** (`<details>`) — opis, dowód, rozbicie scoringu;
- **kliknij „Napraw automatycznie"** → „Skopiowano — wklej w terminalu";
- u góry **„Pobierz skrypt naprawczy (fix.sh)"** → pokaż pobrany plik.

**4. Auto-fix w praktyce (~30 s, opcjonalnie)**
```sh
# wklejasz skopiowaną komendę, np.:
npm audit fix          # albo komenda z findingu systemowego, np. chmod 640 /etc/shadow
```
Powiedz: „przeglądarka nie zmienia systemu — daje gotową komendę / skrypt do uruchomienia".

**5. Kontekst (~25 s, mocna puenta)**
```sh
confrisk-npm --path examples/npm-vuln-demo --asset dev        --format json | grep risk_band | head
confrisk-npm --path examples/npm-vuln-demo --asset crown-jewel --format json | grep risk_band | head
```
Powiedz: „ta sama podatność, inny profil zasobu → inne pasmo ryzyka".

> **Plan B:** jeśli `npm audit` nie zwróci podatności (brak sieci) lub coś nie zadziała — otwórz
> gotowe `docs/sample-reports/raport-npm.html` i `raport-os.html` (mają komplet findingów i przyciski
> auto-fix) i omawiaj na nich. Widz nie pozna różnicy.

---

## 6. Wariant B: wstawienie klipu demo na slajd „Demo" (slajd 14)

1. Nagraj samo demo (sekcja 5) w OBS do pliku `demo.mp4` — powtarzaj aż będzie czyste.
2. W PowerPoint otwórz `docs/Prezentacja_confrisk.pptx`, slajd **14 („Demo")**.
3. *Wstaw → Wideo → To urządzenie…* → wskaż `demo.mp4` (lub przeciągnij plik na ramkę z ikoną ▶).
4. Dopasuj rozmiar do ramki, w *Odtwarzanie* ustaw start „Po kliknięciu" lub „Automatycznie".
5. (Opcjonalnie) w polu „Link do nagrania" wpisz URL, jeśli wolisz hostować film zewnętrznie.

---

## 7. Wskazówki nagraniowe

- **Tempo:** mów wolniej niż się wydaje naturalne; rób 1-sekundowe pauzy między myślami.
- **Najpierw powiedz, potem kliknij** — zapowiedz, co zrobisz, dopiero wykonaj.
- **Pauza po wyniku** — daj 2–3 s na przeczytanie outputu terminala/raportu.
- **Nagrywaj sekcjami** — slajdy i demo osobno; łatwiej poprawić wpadkę bez powtarzania całości.
- **Nie ukrywaj drobnych potknięć** — krótka pauza i powtórzenie zdania w montażu znika.
- **Powiększ kursor** (OBS / ustawienia systemu) — łatwiej śledzić klikanie.
- **Jeden monitor w kadrze** — jeśli masz dwa, nagrywaj konkretny ekran, nie cały pulpit.

---

## 8. Post-produkcja

- **Montaż:** DaVinci Resolve (darmowy) lub CapCut/iMovie. Przytnij ciszę na początku/końcu sekcji.
- **Napisy:** dodaj automatyczne (YouTube/Resolve) — zwiększa dostępność i zrozumiałość.
- **Plansze:** krótkie tytuły sekcji ułatwiają orientację (opcjonalnie).
- **Eksport:** MP4 (H.264), 1080p, ~10–12 Mb/s, audio AAC 192 kb/s.
- **Głośność:** znormalizuj do ok. −16 LUFS (spójny poziom dźwięku).

---

## 9. Checklista przed nagraniem

- [ ] `cargo build --release` wykonane (bez nagrywania kompilacji)
- [ ] `export CONFRISK_CONFIG_DIR="$PWD/config"` ustawione
- [ ] Czcionka terminala 16–20 pt, krótki prompt, jasny motyw
- [ ] Powiadomienia wyłączone (Do Not Disturb)
- [ ] Rozdzielczość 1920×1080, OBS canvas 1080p/30fps
- [ ] Mikrofon podłączony, poziom sprawdzony, filtr szumu włączony
- [ ] Komendy demo przygotowane do skopiowania
- [ ] `docs/sample-reports/*.html` otwarte jako plan B
- [ ] Prezentacja w trybie pełnoekranowym, wskaźnik/kursor powiększony
- [ ] Próba całości raz „na sucho" przed nagraniem właściwym

---

**Materiały:** `docs/Prezentacja_confrisk.pptx`, `docs/sample-reports/`,
`examples/npm-vuln-demo/`, `examples/gradle-project-demo/`.
