# Przykładowe raporty HTML

Gotowe raporty do pokazania na prezentacji / w nagraniu. Otwórz dowolny plik w przeglądarce
(dwuklik) — są samodzielne (CSS inline, bez zależności).

| Plik | Skaner | Pochodzenie | Co pokazuje |
|------|--------|-------------|-------------|
| `raport-gradle.html` | `confrisk-gradle` | **realny skan** `examples/gradle-project-demo` | 5 zablokowanych zależności (krytyczne); brak auto-fix (naprawa = edycja `build.gradle`) |
| `raport-npm.html` | `confrisk-npm` | reprezentatywny (oznaczony „przykład") | findingi `npm audit` + blocklista; **przyciski „Napraw automatycznie"** (`npm audit fix`, `npm update`) + `fix.sh` |
| `raport-os.html` | `confrisk` (system) | reprezentatywny (oznaczony „przykład") | kontrole systemu Linux; przyciski auto-fix (`chmod`, `sysctl`) |

> **Dlaczego część jest „reprezentatywna":** raport gradle to prawdziwy skan (działa offline,
> parsuje `build.gradle`). Raporty npm i OS oznaczone „(przykład)" przedstawiają to, co użytkownik
> widzi na **realnym serwerze Linux / przy `npm audit` online** — w środowisku macOS bez dostępu do
> rejestru `npm audit` zwraca 0 podatności, a kontrole systemu Linux nie mają zastosowania. Mechanizm
> (kod) jest produkcyjny; dane w tych dwóch plikach są dobrane tak, by pokazać pełnię funkcji.

Aby wygenerować własne raporty z realnego środowiska, zobacz `docs/TUTORIAL_NAGRANIE.md`.
