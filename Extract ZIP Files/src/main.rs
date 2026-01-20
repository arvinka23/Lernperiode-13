use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::read::ZipArchive;
use zip::ZipWriter;
use zip::write::FileOptions;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== ZIP-Dateien entpacken und Dateisystem verwalten ===\n");

    // Beispiel 1: ZIP-Datei erstellen
    println!("Beispiel 1: ZIP-Datei erstellen");
    println!("--------------------------------");
    erstelle_beispiel_zip()?;
    
    println!("\n");

    // Beispiel 2: ZIP-Datei entpacken
    println!("Beispiel 2: ZIP-Datei entpacken");
    println!("-------------------------------");
    entpacke_zip("data/beispiel.zip", "data/entpackt")?;

    println!("\n");

    // Beispiel 3: Dateisystem-Operationen
    println!("Beispiel 3: Dateisystem-Operationen");
    println!("-----------------------------------");
    dateisystem_operationen()?;

    Ok(())
}

/// Erstellt eine Beispiel-ZIP-Datei mit mehreren Dateien
fn erstelle_beispiel_zip() -> Result<(), Box<dyn std::error::Error>> {
    // Stelle sicher, dass das data-Verzeichnis existiert
    fs::create_dir_all("data")?;

    // Erstelle einige Testdateien
    fs::write("data/test1.txt", "Dies ist die erste Testdatei.\nZeile 2 der Datei.")?;
    fs::write("data/test2.txt", "Dies ist die zweite Testdatei.\nMit mehreren Zeilen.\nZeile 3.")?;
    fs::write("data/info.txt", "Informationen:\n- Projekt: Extract ZIP Files\n- Sprache: Rust")?;

    // Erstelle Unterverzeichnis
    fs::create_dir_all("data/unterordner")?;
    fs::write("data/unterordner/datei.txt", "Datei im Unterordner")?;

    // Erstelle ZIP-Datei
    let zip_path = "data/beispiel.zip";
    let file = File::create(zip_path)?;
    let mut zip = ZipWriter::new(file);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Füge Dateien zur ZIP hinzu
    println!("Erstelle ZIP-Datei: {}", zip_path);
    
    // Datei 1
    zip.start_file("test1.txt", options)?;
    zip.write_all(b"Dies ist die erste Testdatei.\nZeile 2 der Datei.")?;
    println!("  - test1.txt hinzugefügt");

    // Datei 2
    zip.start_file("test2.txt", options)?;
    zip.write_all(b"Dies ist die zweite Testdatei.\nMit mehreren Zeilen.\nZeile 3.")?;
    println!("  - test2.txt hinzugefügt");

    // Datei 3
    zip.start_file("info.txt", options)?;
    zip.write_all(b"Informationen:\n- Projekt: Extract ZIP Files\n- Sprache: Rust")?;
    println!("  - info.txt hinzugefügt");

    // Datei im Unterordner
    zip.start_file("unterordner/datei.txt", options)?;
    zip.write_all(b"Datei im Unterordner")?;
    println!("  - unterordner/datei.txt hinzugefügt");

    zip.finish()?;
    println!("\nZIP-Datei erfolgreich erstellt!");

    Ok(())
}

/// Entpackt eine ZIP-Datei in ein Zielverzeichnis
fn entpacke_zip<P: AsRef<Path>>(zip_path: P, ziel_verzeichnis: P) -> Result<(), Box<dyn std::error::Error>> {
    let zip_path = zip_path.as_ref();
    let ziel_verzeichnis = ziel_verzeichnis.as_ref();

    // Öffne ZIP-Datei
    let file = File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;

    // Erstelle Zielverzeichnis falls nicht vorhanden
    fs::create_dir_all(ziel_verzeichnis)?;

    println!("Entpacke ZIP-Datei: {}", zip_path.display());
    println!("Zielverzeichnis: {}", ziel_verzeichnis.display());
    println!("\nDateien:");

    // Entpacke alle Dateien
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => ziel_verzeichnis.join(path),
            None => continue,
        };

        // Erstelle Verzeichnisse falls nötig
        if let Some(p) = outpath.parent() {
            if !p.exists() {
                fs::create_dir_all(p)?;
            }
        }

        // Entpacke Datei
        let mut outfile = File::create(&outpath)?;
        std::io::copy(&mut file, &mut outfile)?;

        // Zeige Dateiinformationen
        println!("  - {} ({} Bytes)", 
                 outpath.display(), 
                 file.size());
    }

    println!("\nZIP-Datei erfolgreich entpackt!");
    println!("Anzahl Dateien: {}", archive.len());

    Ok(())
}

/// Demonstriert verschiedene Dateisystem-Operationen
fn dateisystem_operationen() -> Result<(), Box<dyn std::error::Error>> {
    let beispiel_verzeichnis = "data/dateisystem_beispiel";
    
    // 1. Verzeichnis erstellen
    println!("1. Verzeichnis erstellen:");
    fs::create_dir_all(beispiel_verzeichnis)?;
    println!("   Verzeichnis erstellt: {}", beispiel_verzeichnis);

    // 2. Dateien erstellen
    println!("\n2. Dateien erstellen:");
    let dateien = vec![
        "datei1.txt",
        "datei2.txt",
        "unterordner/datei3.txt",
    ];

    for datei in &dateien {
        let pfad = PathBuf::from(beispiel_verzeichnis).join(datei);
        
        // Erstelle Unterverzeichnisse falls nötig
        if let Some(parent) = pfad.parent() {
            fs::create_dir_all(parent)?;
        }

        let inhalt = format!("Inhalt von {}\nErstellt am: {}", 
                           datei, 
                           chrono::Local::now().format("%Y-%m-%d %H:%M:%S"));
        fs::write(&pfad, inhalt)?;
        println!("   Datei erstellt: {}", pfad.display());
    }

    // 3. Verzeichnisinhalt auflisten
    println!("\n3. Verzeichnisinhalt auflisten:");
    if let Ok(entries) = fs::read_dir(beispiel_verzeichnis) {
        for entry in entries {
            if let Ok(entry) = entry {
                let pfad = entry.path();
                let metadata = entry.metadata()?;
                
                if metadata.is_file() {
                    println!("   📄 Datei: {} ({} Bytes)", 
                           pfad.file_name().unwrap().to_string_lossy(),
                           metadata.len());
                } else if metadata.is_dir() {
                    println!("   📁 Verzeichnis: {}", 
                           pfad.file_name().unwrap().to_string_lossy());
                }
            }
        }
    }

    // 4. Pfad-Operationen
    println!("\n4. Pfad-Operationen:");
    let beispiel_pfad = PathBuf::from(beispiel_verzeichnis).join("datei1.txt");
    println!("   Vollständiger Pfad: {}", beispiel_pfad.display());
    println!("   Dateiname: {:?}", beispiel_pfad.file_name());
    println!("   Verzeichnis: {:?}", beispiel_pfad.parent());
    println!("   Erweiterung: {:?}", beispiel_pfad.extension());
    println!("   Existiert: {}", beispiel_pfad.exists());

    // 5. Datei lesen
    println!("\n5. Datei lesen:");
    if beispiel_pfad.exists() {
        let inhalt = fs::read_to_string(&beispiel_pfad)?;
        println!("   Inhalt von {}:", beispiel_pfad.file_name().unwrap().to_string_lossy());
        for zeile in inhalt.lines() {
            println!("     {}", zeile);
        }
    }

    // 6. Datei kopieren
    println!("\n6. Datei kopieren:");
    let ziel_pfad = PathBuf::from(beispiel_verzeichnis).join("datei1_kopie.txt");
    fs::copy(&beispiel_pfad, &ziel_pfad)?;
    println!("   Datei kopiert: {} -> {}", 
           beispiel_pfad.file_name().unwrap().to_string_lossy(),
           ziel_pfad.file_name().unwrap().to_string_lossy());

    // 7. Datei umbenennen/verschieben
    println!("\n7. Datei umbenennen:");
    let neuer_name = PathBuf::from(beispiel_verzeichnis).join("umbenannt.txt");
    fs::rename(&ziel_pfad, &neuer_name)?;
    println!("   Datei umbenannt: {} -> {}", 
           ziel_pfad.file_name().unwrap().to_string_lossy(),
           neuer_name.file_name().unwrap().to_string_lossy());

    Ok(())
}
