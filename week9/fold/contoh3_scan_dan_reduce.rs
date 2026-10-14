// =========================================
// Contoh 3: scan() dan reduce() - Saudara fold()
// scan()   = seperti fold tapi menghasilkan
//            iterator dari setiap nilai antara
// reduce() = seperti fold tapi tanpa nilai awal
// =========================================

fn main() {
    let angka: Vec<i32> = (1..=5).collect();

    // ------- reduce() -------
    // reduce() menggunakan elemen pertama sebagai nilai awal
    println!("=== reduce() ===");

    // Jumlah semua elemen
    let jumlah = angka.iter().copied().reduce(|akum, x| akum + x);
    println!("  Sum    : {:?}", jumlah); // Some(15)

    // Cari nilai terbesar
    let maks = angka.iter().copied().reduce(|a, b| if a > b { a } else { b });
    println!("  Maks   : {:?}", maks); // Some(5)

    // reduce() pada Vec kosong → None (aman!)
    let kosong: Vec<i32> = vec![];
    let hasil = kosong.iter().copied().reduce(|a, b| a + b);
    println!("  Vec kosong reduce: {:?}", hasil); // None

    // ------- scan() -------
    // scan() seperti fold tapi menghasilkan nilai ANTARA di setiap langkah
    println!("\n=== scan() - running total ===");
    let running_total: Vec<i32> = angka.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state) // emit nilai sementara
        })
        .collect();

    println!("  Elemen       : {:?}", angka);
    println!("  Running total: {:?}", running_total); // [1, 3, 6, 10, 15]

    // scan(): running max
    println!("\n=== scan() - running max ===");
    let data = vec![3, 7, 2, 9, 4, 8, 1];
    let running_maks: Vec<i32> = data.iter()
        .scan(i32::MIN, |state, &x| {
            *state = (*state).max(x);
            Some(*state)
        })
        .collect();
    println!("  Data         : {:?}", data);
    println!("  Running max  : {:?}", running_maks); // [3,7,7,9,9,9,9]

    // --- fold vs reduce: perbandingan ---
    println!("\n=== Perbandingan fold vs reduce ===");
    let v = vec![10, 20, 30];

    let dengan_fold   = v.iter().fold(0, |a, &x| a + x);
    let dengan_reduce = v.iter().copied().reduce(|a, b| a + b).unwrap_or(0);

    println!("  fold(0, +)  = {}", dengan_fold);
    println!("  reduce(+)   = {}", dengan_reduce);
    println!("  Hasil sama? {}", dengan_fold == dengan_reduce);
}
