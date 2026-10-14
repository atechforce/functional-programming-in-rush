// =========================================
// Contoh 3: filter_map() dan Kombinasi Filter Lanjutan
// filter_map = filter + map dalam satu langkah
// Berguna ketika proses bisa gagal (Option/Result)
// =========================================

fn main() {
    // --- filter_map(): parsing string ke angka, skip yang gagal ---
    println!("=== filter_map(): parse angka dari string ===");
    let data = vec!["10", "dua", "30", "empat", "50", "abc", "70"];

    let angka_valid: Vec<i32> = data.iter()
        .filter_map(|s| s.parse::<i32>().ok()) // None jika gagal parse
        .collect();

    println!("  Input : {:?}", data);
    println!("  Valid : {:?}", angka_valid); // [10, 30, 50, 70]

    // --- filter_map(): ambil hanya Some() dari Vec<Option<T>> ---
    println!("\n=== filter_map() pada Vec<Option<i32>> ===");
    let mungkin: Vec<Option<i32>> = vec![Some(1), None, Some(3), None, Some(5)];
    let ada: Vec<i32> = mungkin.iter()
        .filter_map(|&x| x)
        .collect();
    println!("  Input : {:?}", mungkin);
    println!("  Ada   : {:?}", ada); // [1, 3, 5]

    // --- filter() dengan pattern matching ---
    println!("\n=== filter() dengan kondisi kompleks ===");
    let nilai = vec![45, 78, 91, 55, 88, 62, 73, 95, 40, 80];

    // Ambil nilai yang di range 70–89 (predikat B atau C)
    let menengah: Vec<&i32> = nilai.iter()
        .filter(|&&n| n >= 70 && n <= 89)
        .collect();
    println!("  Semua  : {:?}", nilai);
    println!("  70–89  : {:?}", menengah);

    // --- filter().map() vs filter_map() (efek setara) ---
    println!("\n=== Perbandingan filter+map vs filter_map ===");
    let teks = vec!["12", "abc", "34", "!", "56"];

    // Cara 1: filter dulu, map kemudian
    let cara1: Vec<i32> = teks.iter()
        .filter(|s| s.parse::<i32>().is_ok())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    // Cara 2: filter_map (lebih ringkas)
    let cara2: Vec<i32> = teks.iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("  Cara 1 (filter+map)  : {:?}", cara1);
    println!("  Cara 2 (filter_map)  : {:?}", cara2);
    println!("  Sama? {}", cara1 == cara2);
}
