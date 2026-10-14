// =========================================
// Contoh 1: Iterator Dasar
// Iterator adalah trait yang memungkinkan
// kita melintasi sebuah koleksi elemen
// =========================================

fn main() {
    // --- iter() pada Vec: meminjam elemen ---
    let angka = vec![10, 20, 30, 40, 50];

    println!("=== Iterasi dengan for loop ===");
    for &n in angka.iter() {
        println!("  nilai: {}", n);
    }

    // --- iter() secara eksplisit menggunakan .next() ---
    println!("\n=== Iterasi manual dengan .next() ===");
    let mut it = angka.iter();
    println!("next() #1 = {:?}", it.next()); // Some(10)
    println!("next() #2 = {:?}", it.next()); // Some(20)
    println!("next() #3 = {:?}", it.next()); // Some(30)
    println!("next() #4 = {:?}", it.next()); // Some(40)
    println!("next() #5 = {:?}", it.next()); // Some(50)
    println!("next() #6 = {:?}", it.next()); // None (habis)

    // --- into_iter(): memindahkan ownership ---
    println!("\n=== into_iter() (consume vec) ===");
    let buah = vec!["apel", "mangga", "jeruk"];
    for b in buah.into_iter() {
        println!("  buah: {}", b);
    }
    // `buah` tidak bisa dipakai lagi di sini

    // --- iter pada range ---
    println!("\n=== Iterasi Range 1..=5 ===");
    for i in 1..=5 {
        println!("  i = {}", i);
    }
}
