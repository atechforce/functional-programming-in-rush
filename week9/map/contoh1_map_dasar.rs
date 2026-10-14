// =========================================
// Contoh 1: map() Dasar
// map() mengubah setiap elemen iterator
// menjadi elemen baru menggunakan closure
// =========================================

fn main() {
    let angka = vec![1, 2, 3, 4, 5];

    // --- Mengkuadratkan setiap elemen ---
    println!("=== Kuadrat setiap elemen ===");
    let kuadrat: Vec<i32> = angka.iter()
        .map(|&x| x * x)
        .collect();
    println!("  Input : {:?}", angka);
    println!("  Output: {:?}", kuadrat);

    // --- Mengubah tipe: i32 → f64 ---
    println!("\n=== Konversi i32 ke f64 ===");
    let float_angka: Vec<f64> = angka.iter()
        .map(|&x| x as f64 * 1.5)
        .collect();
    println!("  Output: {:?}", float_angka);

    // --- map() pada String ---
    println!("\n=== Huruf kapital semua kata ===");
    let kata = vec!["halo", "dunia", "rust", "itu", "keren"];
    let kapital: Vec<String> = kata.iter()
        .map(|s| s.to_uppercase())
        .collect();
    println!("  Input : {:?}", kata);
    println!("  Output: {:?}", kapital);

    // --- map() tambah prefix ---
    println!("\n=== Tambah prefix ke setiap kata ===");
    let dengan_prefix: Vec<String> = kata.iter()
        .map(|s| format!("📌 {}", s))
        .collect();
    for item in &dengan_prefix {
        println!("  {}", item);
    }
}
