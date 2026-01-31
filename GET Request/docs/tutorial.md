# Tutorial: HTTP GET-Request mit Rust

In diesem Tutorial lernst du, wie du mit Rust einen einfachen HTTP GET-Request ausführst und Status, Headers sowie den Response-Body ausgibst.

---

## Vorwissen

Wir setzen voraus, dass du bereits:

- die Grundlagen von Rust kennst (Variablen, Funktionen, `Result`),
- mit Cargo und `Cargo.toml` umgehen kannst,
- weißt, was ein HTTP GET-Request ist.

---

## Zu erklärende Konzepte

Im Mittelpunkt stehen:

1. **`reqwest`** – eine Rust-Bibliothek für HTTP-Requests (hier im „blocking“-Modus).
2. **`anyhow::Result`** – eine einfache Art, Fehler durchzureichen, ohne eigene Fehlertypen zu bauen.
3. **`.text()`** – liest den Response-Body als String.

---

## Ausformulieren mit Code-Beispielen

### Abhängigkeiten in `Cargo.toml`

```toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking", "json"] }
anyhow = "1.0"
```

### Hauptprogramm in `src/main.rs`

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let res = reqwest::blocking::get("http://httpbin.org/get")?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", res.text()?);

    Ok(())
}
```

- `reqwest::blocking::get(...)` sendet den GET-Request und blockiert, bis die Antwort da ist.
- `res.status()` und `res.headers()` liefern Statuscode und Header.
- `res.text()?` liest den Body einmal komplett als String; das `?` gibt Fehler an den Aufrufer weiter.

---

## Erwartetes Resultat

Nach `cargo run` solltest du in etwa Folgendes sehen:

- **Status:** z. B. `200 OK`
- **Headers:** z. B. `content-type`, `date`, …
- **Body:** JSON von httpbin.org (z. B. mit `"url": "http://httpbin.org/get"`, …)

*(Optional: Screenshot oder .gif der Konsolenausgabe hier einfügen.)*

---

## Was schiefgehen kann

**Keine Internetverbindung oder falsche URL**

- Wenn dein Rechner offline ist oder die URL nicht erreichbar ist, schlägt `get(...)` fehl.
- Du bekommst dann einen Fehler von `reqwest` (z. B. Timeout oder „connection failed“), der durch `?` in eine `anyhow`-Fehlermeldung umgewandelt wird und das Programm beendet.
- **Tipp:** Prüfe deine Internetverbindung und ob `http://httpbin.org/get` im Browser erreichbar ist.
