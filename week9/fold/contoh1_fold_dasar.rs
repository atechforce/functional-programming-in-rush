// =========================================
// Contoh 1: fold() Dasar
// fold(init, |akumulasi, elemen| ...)
// Mengakumulasi semua elemen menjadi 1 nilai
// =========================================

fn main() {
    let angka = vec![1, 2, 3, 4, 5];

    // --- Penjumlahan manual dengan fold ---
    println!("=== fold() sebagai SUM ===");
    let jumlah = angka.iter().fold(0, |akum, &x| akum + x);
    println!("  {:?}", angka);
    println!("  Jumlah = {}", jumlah); // 15

    // --- Perkalian semua elemen (faktorial gaya fold) ---
    println!("\n=== fold() sebagai PRODUCT ===");
    let hasil_kali = angka.iter().fold(1, |akum, &x| akum * x);
    println!("  Perkalian semua = {}", hasil_kali); // 120

    // --- Mencari nilai maksimum dengan fold ---
    println!("\n=== fold() sebagai MAX ===");
    let data = vec![3, 7, 1, 9, 4, 6, 2];
    let maks = data.iter().fold(i32::MIN, |akum, &x| {
        if x > akum { x } else { akum }
    });
    println!("  {:?}", data);
    println!("  Maksimum = {}", maks); // 9

    // --- Mencari nilai minimum dengan fold ---
    println!("\n=== fold() sebagai MIN ===");
    let min = data.iter().fold(i32::MAX, |akum, &x| {
        if x < akum { x } else { akum }
    });
    println!("  Minimum = {}", min); // 1

    // --- fold() menghitung rata-rata ---
    println!("\n=== fold() sebagai RATA-RATA ===");
    let n = angka.len() as f64;
    let total = angka.iter().fold(0.0, |akum, &x| akum + x as f64);
    println!("  Rata-rata = {}", total / n); // 3.0
}
