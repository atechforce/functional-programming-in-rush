// =========================================
// Contoh 3: collect() dengan Result dan Option
// Mengumpulkan Result<T,E> dari iterator
// Jika ada satu Err/None, seluruh collect() gagal
// =========================================

fn main() {
    // --- collect::<Result<Vec<_>, _>>() ---
    // Jika SEMUA Ok → hasilkan Ok(Vec)
    // Jika ada satu Err → hasilkan Err segera

    println!("=== collect() dengan Result: semua berhasil ===");
    let angka_str = vec!["10", "20", "30", "40"];
    let hasil: Result<Vec<i32>, _> = angka_str.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("  Input : {:?}", angka_str);
    println!("  Output: {:?}", hasil); // Ok([10, 20, 30, 40])

    println!("\n=== collect() dengan Result: ada yang gagal ===");
    let campuran = vec!["10", "dua", "30"];
    let hasil2: Result<Vec<i32>, _> = campuran.iter()
        .map(|s| s.parse::<i32>())
        .collect();
    println!("  Input : {:?}", campuran);
    println!("  Output: {:?}", hasil2); // Err(ParseIntError)

    // --- Menangani Err dengan pola match ---
    println!("\n=== Menangani Result dengan match ===");
    match hasil2 {
        Ok(v)  => println!("  Berhasil: {:?}", v),
        Err(e) => println!("  ❌ Error: {}", e),
    }

    // --- collect::<Option<Vec<_>>>() ---
    // Jika SEMUA Some → hasilkan Some(Vec)
    // Jika ada None → hasilkan None

    println!("\n=== collect() dengan Option: semua Some ===");
    let opsi: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    let terkumpul: Option<Vec<i32>> = opsi.into_iter().collect();
    println!("  Output: {:?}", terkumpul); // Some([1, 2, 3])

    println!("\n=== collect() dengan Option: ada None ===");
    let opsi2: Vec<Option<i32>> = vec![Some(1), None, Some(3)];
    let terkumpul2: Option<Vec<i32>> = opsi2.into_iter().collect();
    println!("  Output: {:?}", terkumpul2); // None

    // --- Praktis: konversi dan validasi sekaligus ---
    println!("\n=== Pipeline lengkap: filter + parse + collect ===");
    let data = vec!["5", "-3", "dua", "8", "-1", "abc", "12"];

    let positif: Vec<i32> = data.iter()
        .filter_map(|s| s.parse::<i32>().ok()) // skip yang gagal parse
        .filter(|&n| n > 0)                    // hanya yang positif
        .collect();

    println!("  Input    : {:?}", data);
    println!("  Positif  : {:?}", positif); // [5, 8, 12]
}
