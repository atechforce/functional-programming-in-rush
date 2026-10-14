// =========================================
// Contoh 1: Closure Dasar
// Closure adalah fungsi anonim yang bisa
// menangkap variabel dari lingkungan sekitarnya
// =========================================

fn main() {
    // Closure sederhana: penjumlahan dua angka
    let tambah = |a, b| a + b;
    println!("5 + 3 = {}", tambah(5, 3));

    // Closure dengan tipe data eksplisit
    let kali = |a: i32, b: i32| -> i32 { a * b };
    println!("4 * 7 = {}", kali(4, 7));

    // Closure tanpa parameter, tanpa return (unit)
    let sapa = || println!("Halo dari closure!");
    sapa();

    // Closure dengan blok ekspresi multi-baris
    let hitung = |x: i32| {
        let kuadrat = x * x;
        let hasil = kuadrat + 10;
        hasil // nilai terakhir = nilai return
    };
    println!("hitung(5) = {}", hitung(5)); // (5*5) + 10 = 35
}
