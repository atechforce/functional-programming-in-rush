// =========================================
// Contoh 1: filter() Dasar
// filter() menyaring elemen berdasarkan
// kondisi dari closure (harus return bool)
// =========================================

fn main() {
    let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // --- Filter angka genap ---
    println!("=== Filter angka GENAP ===");
    let genap: Vec<&i32> = angka.iter()
        .filter(|&&x| x % 2 == 0)
        .collect();
    println!("  Semua  : {:?}", angka);
    println!("  Genap  : {:?}", genap);

    // --- Filter angka ganjil ---
    println!("\n=== Filter angka GANJIL ===");
    let ganjil: Vec<&i32> = angka.iter()
        .filter(|&&x| x % 2 != 0)
        .collect();
    println!("  Ganjil : {:?}", ganjil);

    // --- Filter angka lebih besar dari threshold ---
    println!("\n=== Filter angka > 5 ===");
    let threshold = 5;
    let besar: Vec<&i32> = angka.iter()
        .filter(|&&x| x > threshold)
        .collect();
    println!("  > {} : {:?}", threshold, besar);

    // --- Filter string berdasarkan panjang karakter ---
    println!("\n=== Filter kata dengan panjang > 4 karakter ===");
    let kata = vec!["hi", "halo", "dunia", "rust", "pemrograman", "oke"];
    let kata_panjang: Vec<&&str> = kata.iter()
        .filter(|s| s.len() > 4)
        .collect();
    println!("  Semua    : {:?}", kata);
    println!("  Panjang  : {:?}", kata_panjang);

    // --- Menghitung jumlah elemen yang lolos filter ---
    println!("\n=== Hitung angka yang habis dibagi 3 ===");
    let hitung = angka.iter().filter(|&&x| x % 3 == 0).count();
    println!("  Jumlah habis dibagi 3: {}", hitung);
}
