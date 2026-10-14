// =========================================
// Contoh 3: Iterator Adapter dan Consumer
// Adapter  → mengubah iterator jadi iterator baru (lazy)
// Consumer → menghabiskan iterator, menghasilkan nilai akhir
// =========================================

fn main() {
    let angka = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // ---- ADAPTER (lazy, belum dievaluasi) ----

    // .skip(n)  → lewati n elemen pertama
    println!("=== skip(3) ===");
    let setelah_skip: Vec<_> = angka.iter().skip(3).collect();
    println!("  {:?}", setelah_skip); // [4,5,6,7,8,9,10]

    // .take(n)  → ambil hanya n elemen pertama
    println!("\n=== take(4) ===");
    let empat_pertama: Vec<_> = angka.iter().take(4).collect();
    println!("  {:?}", empat_pertama); // [1,2,3,4]

    // .enumerate() → pasangkan (indeks, nilai)
    println!("\n=== enumerate() ===");
    for (i, val) in angka.iter().enumerate() {
        if i < 3 {
            println!("  indeks {} → nilai {}", i, val);
        }
    }

    // .zip()  → gabungkan dua iterator menjadi pasangan tuple
    println!("\n=== zip() ===");
    let huruf = vec!['a', 'b', 'c', 'd', 'e'];
    let pasangan: Vec<_> = angka.iter().zip(huruf.iter()).collect();
    for (n, h) in &pasangan {
        println!("  {} ↔ {}", n, h);
    }

    // ---- CONSUMER ----

    // .count()   → hitung jumlah elemen
    let jumlah = angka.iter().count();
    println!("\n=== count() ===");
    println!("  Jumlah elemen: {}", jumlah);

    // .sum()     → jumlahkan semua elemen
    let total: i32 = angka.iter().sum();
    println!("\n=== sum() ===");
    println!("  Total: {}", total); // 55

    // .max() / .min()
    println!("\n=== max() dan min() ===");
    println!("  Maks: {:?}", angka.iter().max());
    println!("  Min : {:?}", angka.iter().min());

    // .any() / .all()
    println!("\n=== any() dan all() ===");
    println!("  Ada yg > 8? {}", angka.iter().any(|&x| x > 8));
    println!("  Semua > 0?  {}", angka.iter().all(|&x| x > 0));
}
