// =========================================
// Contoh 1: collect() Dasar
// collect() mengubah iterator menjadi koleksi
// Rust menyimpulkan tipe tujuan secara otomatis
// =========================================

fn main() {
    // --- collect() ke Vec<T> ---
    println!("=== collect() → Vec ===");

    // Dari range
    let dari_range: Vec<i32> = (1..=5).collect();
    println!("  Range 1..=5 : {:?}", dari_range);

    // Dari iterator dengan transformasi
    let kuadrat: Vec<i32> = (1..=5).map(|x| x * x).collect();
    println!("  Kuadrat     : {:?}", kuadrat);

    // Dari iterator string
    let huruf: Vec<char> = "halo".chars().collect();
    println!("  Chars 'halo': {:?}", huruf);

    // --- collect() ke String ---
    println!("\n=== collect() → String ===");
    let karakter = vec!['R', 'u', 's', 't'];
    let kata: String = karakter.iter().collect();
    println!("  Vec<char> → String: \"{}\"", kata);

    // Gabung kata dengan pemisah (menggunakan join)
    let kata_kata = vec!["Halo", "Dunia", "Rust"];
    let kalimat: String = kata_kata.join(" ");
    println!("  Join dengan spasi : \"{}\"", kalimat);

    // --- collect() ke Vec dengan filter ---
    println!("\n=== collect() setelah filter ===");
    let semua: Vec<i32> = (1..=10).collect();
    let genap: Vec<i32> = semua.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("  Semua : {:?}", semua);
    println!("  Genap : {:?}", genap);

    // --- collect() ke Vec<String> dari &str ---
    println!("\n=== collect() &str → Vec<String> ===");
    let bahan = vec!["beras", "gula", "garam"];
    let milik: Vec<String> = bahan.iter().map(|&s| s.to_string()).collect();
    println!("  {:?}", milik);
}
