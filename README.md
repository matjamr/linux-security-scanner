# confrisk

Framework oceny bezpieczeństwa dla systemów Linux i zależności aplikacji
z kontekstowym scoringiem ryzyka. Zamiast samej dotkliwości, wynik zależy
od krytyczności zasobu, ekspozycji i pewności detekcji:

```
risk     = severity x asset x exposure x confidence
priority = risk / effort
```

## Narzędzia

| Binarka | Zastosowanie |
|---------|--------------|
| `confrisk` | skan konfiguracji systemu Linux (SSH, uprawnienia plików, kernel) |
| `confrisk-npm` | skan zależności npm (blocklista, npm audit, pakiety nieaktualne) |
| `confrisk-gradle` | skan zależności Gradle/Maven (parsowanie build.gradle) |

Wyjście: tekst, JSON (CI/CD) oraz raport HTML z przyciskami naprawy
i zbiorczym skryptem fix.sh.

## Budowanie i instalacja

```bash
cd confrisk
cargo build --release
cargo install --path .        # binarki w ~/.cargo/bin

# konfiguracja (jedno z):
export CONFRISK_CONFIG_DIR="$PWD/config"
# albo: cp -r config ~/.config/confrisk
```

## Użycie

```bash
# skan systemu -> raport HTML
confrisk --asset production --out report.html

# skan zaleznosci, blokada builda w CI
confrisk-npm    --path ./app --fail-on high --exit-code
confrisk-gradle --path ./app --fail-on high --exit-code

# raport HTML dla projektu
confrisk-npm --path ./app --format html --out raport.html
```

Profil zasobu (`--asset`): `dev`, `internal`, `production`, `crown-jewel` —
ta sama podatność dostaje inny wynik ryzyka zależnie od środowiska.

## Konfiguracja

Reguły, kontrole i wagi scoringu są w plikach JSON (katalog `confrisk/config/`):

```
config/
├── categories.json      # kategorie problemow
├── scoring.json         # wagi modelu ryzyka
├── checks/              # kontrole systemowe (1 plik = 1 kontrola)
├── plugins/             # integracje zewnetrznych skanerow
└── rules/
    ├── dependencies.json  # blocklista pakietow (npm + maven)
    └── ports.json         # niebezpieczne porty
```

Nową kontrolę lub regułę dodaje się plikiem JSON, bez rekompilacji.
Lokalizację konfiguracji wskazuje zmienna `CONFRISK_CONFIG_DIR`
(fallback: `~/.config/confrisk`, `/etc/confrisk`, `./config`).

## Struktura repozytorium

```
confrisk/
├── src/                 # kod (Rust): model ryzyka, config, skanery, raport, CLI
├── config/              # konfiguracja JSON
├── examples/            # projekty demonstracyjne (npm, gradle, docker)
└── scripts/             # build-deb.sh itd.
docs/                    # dokumentacja, sprawozdanie, prezentacja, przykładowe raporty
```

## Dokumentacja

- [docs/DOKUMENTACJA_KODU.md](docs/DOKUMENTACJA_KODU.md) — opis modułów i przepływów
- [docs/CONFIG_SYSTEM.md](docs/CONFIG_SYSTEM.md) — format plików konfiguracyjnych
- [docs/CONFRISK_NPM.md](docs/CONFRISK_NPM.md) — skaner npm
- [docs/GRADLE_SCANNER.md](docs/GRADLE_SCANNER.md) — skaner Gradle + bramka w buildzie
- [docs/sample-reports/](docs/sample-reports/) — przykładowe raporty HTML

## Testy

```bash
cd confrisk && cargo test
```
