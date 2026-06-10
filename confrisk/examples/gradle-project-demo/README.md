# Gradle Project Security Demo

Demo Gradle project with intentionally vulnerable dependencies for testing `confrisk-gradle`.

## Vulnerable Dependencies

This project includes several known vulnerable libraries:

- **log4j-core 2.14.1** - Log4Shell vulnerability (CVE-2021-44228)
- **spring-core 5.3.5** - Spring4Shell vulnerability (CVE-2022-22965)
- **jackson-databind 2.12.0** - Deserialization vulnerabilities
- **commons-collections 3.2.1** - Remote code execution via deserialization

## Running Security Scan

```bash
# From confrisk root directory
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --config config

# With fail-on-high for CI/CD
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --fail-on high --exit-code

# JSON output
cargo run --bin confrisk-gradle -- --path examples/gradle-project-demo --format json
```

## Expected Results

The scanner should detect:
- 4+ CRITICAL vulnerabilities
- Multiple security issues with detailed remediation

## Bramka bezpieczeństwa w buildzie (Opcja A)

`build.gradle` zawiera task `confriskScan` (typu `Exec`) wpięty przed `compileJava`,
`build`, `assemble`, `check`, `install`/`publishToMavenLocal`. Gdy skaner znajdzie
zależność o ryzyku ≥ próg (`--fail-on high`), zwraca kod ≠ 0 i **Gradle przerywa build** —
np. `gradle build` lub `./gradlew clean install` kończy się błędem.

### Wymagania

```bash
# 1) confrisk-gradle na PATH
cargo install --path /ścieżka/do/confrisk   # instaluje do ~/.cargo/bin

# 2) konfiguracja widoczna dla skanera (jedno z poniższych)
export CONFRISK_CONFIG_DIR=/ścieżka/do/confrisk/config   # dziedziczone przez Gradle
# albo: cp -r config ~/.config/confrisk
# albo: umieść katalog `config/` w katalogu projektu
```

### Uruchomienie

```bash
# w tym katalogu (wymaga zainstalowanego gradle lub wrappera)
gradle build           # albo: ./gradlew clean install

# → build PRZERWANY: zadanie ':confriskScan' kończy się błędem
#   („❌ Security issues found! Failing build.")
```

> To demo nie ma jeszcze wrappera (`gradlew`). Wygeneruj go raz poleceniem
> `gradle wrapper`, albo uruchamiaj `gradle …` bezpośrednio. W realnym projekcie
> `./gradlew` zwykle już istnieje — wystarczy wkleić blok `confriskScan` do `build.gradle`.

### Jak to działa / ograniczenia

- Task odpala `confrisk-gradle --path <projekt> --fail-on high --exit-code`; `Exec`
  domyślnie przerywa build na kodzie ≠ 0.
- confrisk parsuje **build.gradle** (regex), więc łapie zależności **zadeklarowane wprost**,
  nie tranzytywne. Dla tranzytywnych użyj reguł `resolutionStrategy` (Opcja B).
- Po naprawie wersji (sekcja niżej) `confriskScan` przechodzi i build idzie dalej.

## Fixing Vulnerabilities

Update `build.gradle` with safe versions:

```gradle
dependencies {
    implementation 'org.apache.logging.log4j:log4j-core:2.17.1'
    implementation 'org.springframework:spring-core:5.3.18'
    implementation 'com.fasterxml.jackson.core:jackson-databind:2.13.4'
    implementation 'org.apache.commons:commons-collections4:4.4'
}
```
