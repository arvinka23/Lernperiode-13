# Read CSV File

## Beschreibung
Einlesen und Verarbeiten von CSV-Dateien.

## Lernziele
- Arbeiten mit CSV-Daten
- Parsen von Zeilen und Spalten
- Abbildung der Daten in Rust-Strukturen

## Verwendung

### Projekt ausführen
```bash
cargo run
```

### Projekt kompilieren
```bash
cargo build
```

## Projektstruktur
- `src/main.rs` - Hauptprogramm mit CSV-Einlese-Funktionen
- `data/personen.csv` - Beispiel-CSV mit Personendaten
- `data/produkte.csv` - Beispiel-CSV mit Produktdaten

## Funktionen
- `lese_personen_csv()` - Liest Personendaten aus CSV und mappt sie in `Person`-Strukturen
- `lese_produkte_csv()` - Liest Produktdaten aus CSV und mappt sie in `Produkt`-Strukturen

## Abhängigkeiten
- `csv` - CSV-Parsing-Bibliothek
- `serde` - Serialisierung/Deserialisierung für Rust-Strukturen
