// =========================================
// Contoh 2: collect() ke HashMap dan HashSet
// =========================================

use std::collections::{HashMap, HashSet};

fn main() {
    // --- collect() → HashMap dari tuple (key, value) ---
    println!("=== collect() → HashMap ===");
    let pasangan = vec![
        ("nama",  "Andi"),
        ("kota",  "Bandung"),
        ("hobi",  "Coding"),
        ("bahasa","Rust"),
    ];

    let peta: HashMap<&str, &str> = pasangan.into_iter().collect();
    println!("  nama   : {}", peta["nama"]);
    println!("  kota   : {}", peta["kota"]);
    println!("  bahasa : {}", peta["bahasa"]);

    // --- HashMap dari dua Vec menggunakan zip ---
    println!("\n=== HashMap dari zip dua Vec ===");
    let kunci = vec!["satu", "dua", "tiga", "empat", "lima"];
    let nilai = vec![1, 2, 3, 4, 5];

    let map_angka: HashMap<&str, i32> = kunci.iter()
        .copied()
        .zip(nilai.iter().copied())
        .collect();

    let mut urut: Vec<_> = map_angka.iter().collect();
    urut.sort_by_key(|&(k, _)| *k);
    for (k, v) in urut {
        println!("  {:5} → {}", k, v);
    }

    // --- collect() → HashSet (menghapus duplikat) ---
    println!("\n=== collect() → HashSet (hapus duplikat) ===");
    let dengan_duplikat = vec![1, 2, 3, 2, 4, 3, 5, 1, 6];
    let unik: HashSet<i32> = dengan_duplikat.iter().copied().collect();

    let mut unik_urut: Vec<i32> = unik.into_iter().collect();
    unik_urut.sort();
    println!("  Asli   : {:?}", dengan_duplikat);
    println!("  Unik   : {:?}", unik_urut);

    // --- collect() → HashMap frekuensi ---
    println!("\n=== HashMap frekuensi huruf ===");
    let teks = "hello world rust";
    let frekuensi: HashMap<char, usize> = teks.chars()
        .filter(|c| *c != ' ')
        .fold(HashMap::new(), |mut map, c| {
            *map.entry(c).or_insert(0) += 1;
            map
        });

    let mut huruf_urut: Vec<_> = frekuensi.iter().collect();
    huruf_urut.sort_by_key(|&(k, _)| k);
    println!("  Teks: \"{}\"", teks);
    for (h, n) in huruf_urut {
        println!("    '{}' muncul {} kali", h, n);
    }
}
