# Extract ZIP Files

## Beschreibung
Arbeiten mit dem Dateisystem, Entpacken komprimierter Archive und Pfad- und Dateiverwaltung.

## Lernziele
- Arbeiten mit dem Dateisystem
- Entpacken komprimierter Archive
- Pfad- und Dateiverwaltung

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
- `src/main.rs` - Hauptprogramm mit ZIP- und Dateisystem-Funktionen
- `data/` - Verzeichnis für ZIP-Dateien und entpackte Inhalte

## Funktionen

### ZIP-Operationen
- `erstelle_beispiel_zip()` - Erstellt eine Beispiel-ZIP-Datei mit mehreren Dateien
- `entpacke_zip()` - Entpackt eine ZIP-Datei in ein Zielverzeichnis

### Dateisystem-Operationen
- `dateisystem_operationen()` - Demonstriert verschiedene Dateisystem-Operationen:
  - Verzeichnisse erstellen
  - Dateien erstellen und schreiben
  - Verzeichnisinhalt auflisten
  - Pfad-Operationen (Dateiname, Verzeichnis, Erweiterung)
  - Dateien lesen
  - Dateien kopieren
  - Dateien umbenennen/verschieben

## Abhängigkeiten
- `zip` - ZIP-Archiv-Bibliothek zum Erstellen und Entpacken
- `walkdir` - Rekursive Verzeichnisdurchsuchung
- `chrono` - Datum- und Zeit-Bibliothek

## Beispiel-Ausgabe
Das Programm:
1. Erstellt eine Beispiel-ZIP-Datei mit mehreren Dateien
2. Entpackt die ZIP-Datei in ein Zielverzeichnis
3. Demonstriert verschiedene Dateisystem-Operationen
