# confrisk — plan projektu

**Framework: scan → score → prioritize**
Łączy wyniki skanowania konfiguracji systemu Linux z oceną ryzyka i priorytetyzacją remediacji.

Narzędzie CLI w Ruście, zero zależności zewnętrznych (cały kod na `std`), generuje raport HTML w stylu "security console".

Projekt spełnia trzy wymagane elementy:
- **Model oceny** → sekcja [2](#2-model-oceny-sedno-projektu)
- **Implementacja prototypu** → sekcje [3–6](#3-pipeline)
- **Demonstracja działania na przykładzie** → sekcja [9](#9-demo-podatny-kontener-docker) (podatny kontener Docker)

---

## 1. Setup

```bash
cargo new confrisk --bin
cd confrisk
```

Zero zależności zewnętrznych — wszystko na stdlib (`std::fs`, `std::os::unix::fs::PermissionsExt`, `std::process::Command`). Kompiluje się offline, brak zabawy z serde czy clap.

Struktura projektu:

```
confrisk/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI, spina pipeline
    ├── model.rs     # typy + silnik scoringu
    ├── checks.rs    # sondy konfiguracji Linuksa
    └── report.rs    # generator HTML
```

`Cargo.toml` — nic ponad standard:

```toml
[package]
name = "confrisk"
version = "0.1.0"
edition = "2021"

[profile.release]
opt-level = 2
```

---

## 2. Model oceny (sedno projektu)

Model jest **hybrydowy** — bo sam CVSS opisuje podatność w próżni, a nie ryzyko w *twoim* środowisku. Realne ryzyko zależy od tego, na jakim zasobie problem występuje i czy jest osiągalny z zewnątrz.

```
risk     = severity × asset_criticality × exposure × confidence
priority = risk ÷ effort
```

Czynniki i sugerowane wagi:

| Czynnik | Wartości → mnożnik | Co opisuje |
|---|---|---|
| `severity` | Info 1.0 / Low 3.0 / Med 5.5 / High 8.0 / Crit 10.0 | bazowa dotkliwość (CIS level / CVSS) |
| `asset_criticality` | dev 0.5 / internal 0.8 / prod 1.1 / crown-jewel 1.3 | jak krytyczny jest zasób |
| `exposure` | local 0.7 / adjacent 0.95 / internet-facing 1.25 | osiągalność / powierzchnia ataku |
| `confidence` | 0.0 – 1.0 | pewność, że to NIE false-positive |
| `effort` | 1.0 (trywialny) – 5.0 (przepisanie architektury) | nakład remediacji |

Sortowanie po `priority` wypycha na górę **quick wins** — wysokie ryzyko przy niskim nakładzie. To właśnie odróżnia "listę findingów" od "co naprawić w poniedziałek rano".

**Kluczowe dla wyróżnienia projektu:** zachowaj rozbicie score na czynniki i pokaż je w raporcie (*explainability*). W security audytowalność decyzji liczy się tak samo jak sama decyzja. Każdy finding powinien nieść uzasadnienie typu:

```
8.0 (sev) × 1.30 (asset:crown-jewel) × 1.25 (expo:internet-facing) × 0.95 (conf) = 12.35
```

### Pasma ryzyka (do kolorowania raportu)

| Pasmo | Próg risk |
|---|---|
| critical | ≥ 9.0 |
| high | ≥ 6.0 |
| medium | ≥ 3.5 |
| low | ≥ 1.5 |
| info | < 1.5 |

---

## 3. Pipeline

Klasyczny **collect → normalize → score → prioritize → report**:

1. **collect** — sondy czytają realny stan systemu
2. **normalize** — każda sonda zwraca znormalizowany `Finding`
3. **score** — silnik liczy `risk` i `priority`
4. **prioritize** — sortowanie malejąco po `priority`
5. **report** — render HTML

Znormalizowany model (warstwa wewnętrzna) odcina framework od konkretnego skanera. Jeśli kiedyś zechcesz wczytywać OpenSCAP/Trivy, dopisujesz tylko cienki parser → `Finding`, a reszta pipeline'u się nie zmienia.

---

## 4. `model.rs` — typy i silnik

Definicje do zaimplementowania:

```rust
enum Severity { Info, Low, Medium, High, Critical }   // + fn weight() -> f64, fn label()
enum AssetCriticality { Dev, Internal, Production, CrownJewel }  // + fn multiplier(), fn label()
enum Exposure { Local, AdjacentNetwork, InternetFacing }        // + fn multiplier(), fn label()

struct Finding {
    id: String,            // np. "SSH-001"
    title: String,
    description: String,
    severity: Severity,
    exposure: Exposure,
    confidence: f64,       // 0.0–1.0
    effort: f64,           // 1.0–5.0
    remediation: String,
    evidence: String,      // co realnie wykryto na maszynie
    passed: bool,          // true = kontrola OK, brak problemu
}

struct ScoredFinding {
    finding: Finding,
    risk: f64,
    priority: f64,
}
```

Silnik scoringu:

```rust
pub fn score_all(findings: Vec<Finding>, ctx: AssetCriticality) -> Vec<ScoredFinding> {
    // dla każdego findingu:
    //   risk = if passed { 0.0 }
    //          else { severity.weight() * ctx.multiplier()
    //                 * exposure.multiplier() * confidence }
    //   priority = if effort > 0 { risk / effort } else { risk }
    // sortuj malejąco po priority, remisy rozstrzyga risk
}
```

Dodatkowo metoda `ScoredFinding::explanation(ctx)` zwracająca string z rozbiciem score (patrz sekcja 2) oraz `risk_band()` mapujący na pasmo.

---

## 5. `checks.rs` — sondy konfiguracji (8 realnych kontroli)

Wszystkie odporne na brak pliku/uprawnień — wtedy zwracają `passed=true` z niską `confidence` zamiast paniki. Wzorzec: **read → sprawdź warunek → zwróć `Finding` z `evidence`**.

| ID | Co sprawdza | Severity | Exposure | Źródło |
|---|---|---|---|---|
| SSH-001 | `PermitRootLogin no` | High | internet-facing | `/etc/ssh/sshd_config` |
| SSH-002 | `PasswordAuthentication no` | Medium | internet-facing | `/etc/ssh/sshd_config` |
| FILE-001 | uprawnienia ≤ 644 | Medium | local | `/etc/passwd` |
| FILE-002 | nieczytelny dla "other" | Critical | local | `/etc/shadow` |
| FILE-003 | brak plików world-writable | High | local | skan `/etc` |
| KRNL-001 | ASLR == 2 | Medium | local | `/proc/sys/kernel/randomize_va_space` |
| NET-001 | IP forwarding == 0 | Low | adjacent | `/proc/sys/net/ipv4/ip_forward` |
| CRON-001 | bit `0o077 == 0` | High | local | `/etc/crontab` |

Pomocniki do napisania raz:
- `read(path) -> Option<String>` — czyta plik, `None` gdy nieosiągalny
- `directive_value(content, key) -> Option<String>` — pobiera aktywną (niezakomentowaną) dyrektywę z pliku typu sshd_config
- `perms_check(path, max_mode, ...)` — generyczna kontrola uprawnień przez `metadata().permissions().mode() & 0o777`

`run_all() -> Vec<Finding>` po prostu zbiera wszystkie sondy do wektora.

---

## 6. `report.rs` — generator HTML

Estetyka **"security console"**: ciemny motyw, monospace (JetBrains Mono / Fira Code), kolorowe pasy ryzyka po lewej (`border-left`), findingi jako natywne `<details>/<summary>` (zero JS).

Układ raportu od góry:
1. **header** — host, profil zasobu, data skanu, liczba kontroli
2. **posture banner** — ogólny stan (ZAGROŻONY / WYMAGA UWAGI / DO POPRAWY / STABILNY) + skumulowane ryzyko
3. **karty liczników** — critical / high / medium / low / passed
4. **blok modelu** — wzór scoringu, żeby raport był samoobjaśniający
5. **lista findingów** — posortowana po priorytecie, top 3 otwarte domyślnie

W każdym rozwiniętym findingu: opis, **evidence** (co wykryto), **uzasadnienie score** (rozbicie na czynniki), nakład remediacji, rekomendacja.

Implementacja: pojedynczy string przez `format!`, zero crate'ów. Pamiętaj o `esc()` na danych wstrzykiwanych do HTML (`& < > "`).

---

## 7. `main.rs` — CLI

```bash
confrisk --asset production --out report.html
```

- Parsuj `std::env::args` ręcznie (flagi `--asset`, `--out`); bez clap
- Zmapuj `--asset` na `AssetCriticality` (dev / internal / production / crown-jewel)
- Datę pobierz z `Command::new("date")` albo hardcode formatu
- Hostname z `Command::new("hostname")` lub `/etc/hostname`
- Pipeline: `checks::run_all()` → `model::score_all(findings, ctx)` → `report::render(...)` → zapis pliku → krótkie podsumowanie na stdout

Przykładowy stdout:

```
confrisk v0.1 — skan zakończony
host: web-prod-01 | profil: production
findingi: 8 (critical: 2, high: 3, medium: 1, passed: 2)
skumulowane ryzyko: 41.7
raport: report.html
```

---

## 8. Kolejność roboty

1. `model.rs` — enumy, struktury, `score_all()`. Skompiluj sam ten moduł (`cargo build`).
2. `checks.rs` — najpierw JEDNA kontrola (SSH-001), `cargo build`, dopiero potem reszta.
3. `report.rs` — najpierw goły HTML bez stylów, potem dołóż CSS.
4. `main.rs` — CLI i sklejka.
5. `cargo run -- --asset dev` na własnej maszynie → otwórz `report.html`.

---

## 9. Demo: podatny kontener Docker

Najlepsza "demonstracja działania na przykładzie" — celowo zepsuty obraz, na którym wszystkie kontrole pokazują FAIL, plus porównanie dwóch profili zasobu.

### 9.1 Dockerfile — maszyna z wieloma podatnościami

Utwórz `demo/Dockerfile.vulnerable`:

```dockerfile
FROM ubuntu:22.04

# --- Celowe podatności konfiguracyjne (do wykrycia przez confrisk) ---

RUN apt-get update && apt-get install -y openssh-server cron && rm -rf /var/lib/apt/lists/*

# SSH-001 + SSH-002: root login po haśle, hasła włączone
RUN mkdir -p /etc/ssh && cat > /etc/ssh/sshd_config <<'EOF'
PermitRootLogin yes
PasswordAuthentication yes
EOF

# FILE-002: shadow czytelny dla wszystkich (krytyczne!)
RUN chmod 644 /etc/shadow

# FILE-003: world-writable plik w /etc
RUN echo "junk" > /etc/badconfig && chmod 666 /etc/badconfig

# CRON-001: zapisywalny crontab
RUN touch /etc/crontab && chmod 666 /etc/crontab

# NET-001: włączony IP forwarding (zapis w obrazie nie utrzyma /proc,
#          ale demonstruje regułę; realnie ustawisz przez --sysctl przy uruchomieniu)

# Skopiuj zbudowane, statyczne confrisk (patrz sekcja 10)
COPY confrisk /usr/local/bin/confrisk

ENTRYPOINT ["/usr/local/bin/confrisk"]
```

> Uwaga o `/proc`: ASLR i IP forwarding czytane są z `/proc/sys/...`, którego nie ustawisz w warstwie obrazu. Aby zademonstrować NET-001, uruchom kontener z `--sysctl net.ipv4.ip_forward=1`.

### 9.2 Build i uruchomienie

```bash
# 1. Zbuduj statyczne confrisk (musl — patrz sekcja 10), połóż obok Dockerfile
cp target/x86_64-unknown-linux-musl/release/confrisk demo/confrisk

# 2. Zbuduj obraz
cd demo
docker build -f Dockerfile.vulnerable -t confrisk-demo .

# 3. Uruchom — profil crown-jewel, IP forwarding włączony, raport na host
docker run --rm \
  --sysctl net.ipv4.ip_forward=1 \
  -v "$PWD/out:/out" \
  confrisk-demo --asset crown-jewel --out /out/report.html

# 4. Otwórz raport
xdg-open out/report.html
```

### 9.3 Porównanie profili — clou demonstracji

Uruchom DWA RAZY ten sam obraz, różne profile zasobu:

```bash
docker run --rm -v "$PWD/out:/out" confrisk-demo --asset dev        --out /out/dev.html
docker run --rm -v "$PWD/out:/out" confrisk-demo --asset crown-jewel --out /out/crown.html
```

Ten sam finding (np. SSH-001) dostaje **różny priorytet** zależnie od krytyczności zasobu:

| Finding | dev (×0.5) | crown-jewel (×1.3) |
|---|---|---|
| SSH-001 PermitRootLogin | risk ≈ 4.75 | risk ≈ 12.35 |

To najmocniejszy dowód, że model liczy **ryzyko kontekstowe**, a nie samą dotkliwość — idealny slajd na prezentację.

### 9.4 (opcjonalnie) docker-compose dla powtarzalności

`demo/docker-compose.yml`:

```yaml
services:
  confrisk-demo:
    build:
      context: .
      dockerfile: Dockerfile.vulnerable
    sysctls:
      - net.ipv4.ip_forward=1
    volumes:
      - ./out:/out
    command: ["--asset", "crown-jewel", "--out", "/out/report.html"]
```

```bash
docker compose run --rm confrisk-demo
```

---

## 10. Deployment jako systemowe CLI

### 10.1 Build statyczny (musl) — jeden plik, zero zależności

Najlepszy format dystrybucji: jeden statyczny binarny plik, działa na każdym Linuksie bez instalowania runtime.

```bash
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
# wynik: target/x86_64-unknown-linux-musl/release/confrisk
```

Sprawdź, że jest statyczny:

```bash
file target/x86_64-unknown-linux-musl/release/confrisk
# -> "statically linked"
ldd  target/x86_64-unknown-linux-musl/release/confrisk
# -> "not a dynamic executable"
```

### 10.2 Instalacja systemowa

```bash
# pojedynczy host
sudo install -m 755 \
  target/x86_64-unknown-linux-musl/release/confrisk \
  /usr/local/bin/confrisk

# weryfikacja
confrisk --asset production --out /tmp/r.html
```

`cargo install` dla wygody developera:

```bash
cargo install --path .   # ląduje w ~/.cargo/bin/confrisk
```

### 10.3 Uruchamianie cykliczne (systemd timer)

Skan np. raz dziennie z zapisem datowanego raportu.

`/etc/systemd/system/confrisk.service`:

```ini
[Unit]
Description=confrisk — skan ryzyka konfiguracji

[Service]
Type=oneshot
ExecStart=/usr/local/bin/confrisk --asset production --out /var/log/confrisk/report-%i.html
# uruchom jako root, bo czyta /etc/shadow itd.
User=root
```

`/etc/systemd/system/confrisk.timer`:

```ini
[Unit]
Description=Codzienny skan confrisk

[Timer]
OnCalendar=daily
Persistent=true

[Install]
WantedBy=timers.target
```

```bash
sudo mkdir -p /var/log/confrisk
sudo systemctl enable --now confrisk.timer
sudo systemctl list-timers | grep confrisk
```

### 10.4 Uwaga o uprawnieniach

Część kontroli wymaga roota (odczyt `/etc/shadow`, metadanych chronionych plików). Bez roota narzędzie nie panikuje — zwraca `passed=true` z niską `confidence`, ale dla pełnego skanu uruchamiaj przez `sudo` lub jako usługa systemowa z `User=root`.

### 10.5 Pakowanie do dystrybucji (skrót)

- **tar.gz** — najprościej: `tar czf confrisk-0.1-x86_64.tar.gz -C target/.../release confrisk`
- **.deb** — przez `cargo-deb` (`cargo install cargo-deb && cargo deb`)
- **kontener narzędziowy** — `FROM scratch` + skopiowany statyczny binarny plik (obraz ~kilka MB)

---

## 11. Co dopisać, gdy rdzeń działa (roadmap)

- adaptery wejścia: parser OpenSCAP / Trivy JSON → `Finding` (framework już to przewiduje)
- eksport JSON obok HTML (do integracji z CI/CD, np. fail build gdy skumulowane ryzyko > próg)
- kod wyjścia zależny od najwyższego pasma (0 = czysto, 1 = high, 2 = critical) — przydatne w pipeline CI
- więcej kontroli (sysctl hardening, sudoers, umask, montowania `nodev/nosuid`)
- konfigurowalne wagi modelu z pliku `confrisk.toml` zamiast hardcode

---

## 12. Mapowanie na wymagania zadania

| Wymaganie | Gdzie w projekcie |
|---|---|
| Model oceny ryzyka | sekcja 2 — hybryda risk/priority z 5 czynnikami + explainability |
| Łączenie skanu z oceną i priorytetyzacją | pipeline collect→score→prioritize, sekcje 3–5 |
| Implementacja prototypu | CLI w Ruście, sekcje 4–7 |
| Demonstracja na przykładzie | podatny kontener Docker + porównanie profili, sekcja 9 |
| Deployment | systemowe CLI, musl, systemd timer, sekcja 10 |