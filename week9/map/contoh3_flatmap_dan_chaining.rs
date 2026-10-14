// =========================================
// Contoh 3: flat_map() dan map() Berantai (Chaining)
// flat_map = map + flatten (untuk nested koleksi)
// =========================================

fn main() {
    // --- Chaining map() berganda ---
    println!("=== Chaining map() ===");
    let angka: Vec<i32> = (1..=5).collect();

    // Kalikan 3, lalu tambah 1, lalu kuadratkan
    let hasil: Vec<i32> = angka.iter()
        .map(|&x| x * 3)        // [3, 6, 9, 12, 15]
        .map(|x| x + 1)         // [4, 7, 10, 13, 16]
        .map(|x| x * x)         // [16, 49, 100, 169, 256]
        .collect();

    println!("  Asal    : {:?}", angka);
    println!("  x*3+1)² : {:?}", hasil);

    // --- flat_map(): setiap elemen menghasilkan iterator ---
    println!("\n=== flat_map() ===");
    let kalimat = vec!["halo dunia", "belajar rust", "functional programming"];

    // Pecah setiap kalimat jadi kata-kata individual
    let semua_kata: Vec<&str> = kalimat.iter()
        .flat_map(|kalimat| kalimat.split_whitespace())
        .collect();

    println!("  Kalimat : {:?}", kalimat);
    println!("  Semua kata: {:?}", semua_kata);

    // --- flat_map() pada angka: hasilkan range ---
    println!("\n=== flat_map() menghasilkan range ===");
    let kelompok = vec![1, 2, 3];

    // Setiap angka n → hasilkan n kopi dari n
    let diulang: Vec<i32> = kelompok.iter()
        .flat_map(|&n| std::iter::repeat(n).take(n as usize))
        .collect();

    println!("  Input   : {:?}", kelompok);
    println!("  Diulang : {:?}", diulang); // [1, 2, 2, 3, 3, 3]

    // --- map() + filter() (preview kombinasi) ---
    println!("\n=== map() + filter(): angka genap dikuadratkan ===");
    let kombinasi: Vec<i32> = (1..=10)
        .filter(|x| x % 2 == 0)
        .map(|x| x * x)
        .collect();
    println!("  {:?}", kombinasi); // [4, 16, 36, 64, 100]
}
