// =========================================
// Contoh 2: fold() untuk Membangun Koleksi
// fold() bisa dipakai untuk membangun Vec, String,
// HashMap, atau tipe apapun sebagai akumulator
// =========================================

use std::collections::HashMap;

fn main() {
    // --- fold() membangun Vec baru (seperti filter+map manual) ---
    println!("=== fold() membangun Vec ===");
    let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // Kumpulkan hanya angka genap yang dikuadratkan
    let genap_kuadrat: Vec<i32> = angka.iter().fold(Vec::new(), |mut akum, &x| {
        if x % 2 == 0 {
            akum.push(x * x);
        }
        akum
    });
    println!("  Input          : {:?}", angka);
    println!("  Genap kuadrat  : {:?}", genap_kuadrat); // [4, 16, 36, 64, 100]

    // --- fold() membangun String ---
    println!("\n=== fold() membangun String ===");
    let kata = vec!["Functional", "Programming", "di", "Rust"];
    let kalimat = kata.iter().fold(String::new(), |mut akum, &s| {
        if !akum.is_empty() {
            akum.push(' ');
        }
        akum.push_str(s);
        akum
    });
    println!("  Kata    : {:?}", kata);
    println!("  Kalimat : \"{}\"", kalimat);

    // --- fold() membangun HashMap: frekuensi kemunculan ---
    println!("\n=== fold() menghitung frekuensi kata ===");
    let teks = vec!["apel", "mangga", "apel", "jeruk", "mangga", "apel"];
    let frekuensi: HashMap<&str, u32> = teks.iter().fold(HashMap::new(), |mut map, &kata| {
        *map.entry(kata).or_insert(0) += 1;
        map
    });
    let mut urut: Vec<(&&str, &u32)> = frekuensi.iter().collect();
    urut.sort_by_key(|&(k, _)| *k);
    for (kata, jumlah) in urut {
        println!("  {:10} → {} kali", kata, jumlah);
    }

    // --- fold() flatten nested Vec ---
    println!("\n=== fold() flatten Vec<Vec<i32>> ===");
    let nested = vec![vec![1, 2], vec![3, 4, 5], vec![6]];
    let flat: Vec<i32> = nested.iter().fold(Vec::new(), |mut akum, v| {
        akum.extend(v.iter());
        akum
    });
    println!("  Nested : {:?}", nested);
    println!("  Flat   : {:?}", flat);
}
