use csv::Reader;
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::path::Path;

// Struktur für Personendaten
#[derive(Debug, Deserialize)]
struct Person {
    name: String,
    alter: u32,
    stadt: String,
    beruf: String,
}

// Struktur für Produktdaten (Beispiel)
#[derive(Debug, Deserialize)]
struct Produkt {
    id: u32,
    name: String,
    preis: f64,
    kategorie: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("=== CSV-Datei einlesen und verarbeiten ===\n");

    // Beispiel 1: Personen-CSV einlesen
    println!("Beispiel 1: Personen-Daten einlesen");
    println!("----------------------------------------");
    lese_personen_csv("data/personen.csv")?;
    
    println!("\n");

    // Beispiel 2: Produkte-CSV einlesen
    println!("Beispiel 2: Produkt-Daten einlesen");
    println!("-----------------------------------");
    lese_produkte_csv("data/produkte.csv")?;

    Ok(())
}

/// Liest eine CSV-Datei mit Personendaten ein und gibt sie aus
fn lese_personen_csv<P: AsRef<Path>>(pfad: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(&pfad)?;
    let mut reader = Reader::from_reader(file);

    // Header ausgeben
    if let Some(header) = reader.headers().ok() {
        println!("Spalten: {:?}", header);
        println!();
    }

    // Zeilen einlesen und in Strukturen mappen
    let mut personen: Vec<Person> = Vec::new();
    
    for result in reader.deserialize() {
        let person: Person = result?;
        personen.push(person);
    }

    // Daten ausgeben
    println!("Anzahl eingelesener Personen: {}", personen.len());
    println!("\nPersonendaten:");
    for person in &personen {
        println!("  - {} ({}) aus {}, Beruf: {}", 
                 person.name, person.alter, person.stadt, person.beruf);
    }

    // Datenverarbeitung: Durchschnittsalter berechnen
    if !personen.is_empty() {
        let durchschnittsalter: f64 = personen.iter()
            .map(|p| p.alter as f64)
            .sum::<f64>() / personen.len() as f64;
        println!("\nDurchschnittsalter: {:.2} Jahre", durchschnittsalter);
    }

    Ok(())
}

/// Liest eine CSV-Datei mit Produktdaten ein und gibt sie aus
fn lese_produkte_csv<P: AsRef<Path>>(pfad: P) -> Result<(), Box<dyn Error>> {
    let file = File::open(&pfad)?;
    let mut reader = Reader::from_reader(file);

    // Header ausgeben
    if let Some(header) = reader.headers().ok() {
        println!("Spalten: {:?}", header);
        println!();
    }

    // Zeilen einlesen und in Strukturen mappen
    let mut produkte: Vec<Produkt> = Vec::new();
    
    for result in reader.deserialize() {
        let produkt: Produkt = result?;
        produkte.push(produkt);
    }

    // Daten ausgeben
    println!("Anzahl eingelesener Produkte: {}", produkte.len());
    println!("\nProduktdaten:");
    for produkt in &produkte {
        println!("  - ID: {}, Name: {}, Preis: {:.2}€, Kategorie: {}", 
                 produkt.id, produkt.name, produkt.preis, produkt.kategorie);
    }

    // Datenverarbeitung: Gesamtwert berechnen
    if !produkte.is_empty() {
        let gesamtpreis: f64 = produkte.iter()
            .map(|p| p.preis)
            .sum();
        println!("\nGesamtpreis aller Produkte: {:.2}€", gesamtpreis);
    }

    Ok(())
}
