fn main() {
    println!("========================================");
    println!("  Halo dari Rust di Windows!");
    println!("========================================");

    let nama = "Programmer";
    println!("Selamat datang, {}! Lingkungan Rust sudah siap digunakan.\n", nama);

    // Contoh perhitungan Fibonacci sederhana
    println!("Deret Fibonacci (10 angka pertama):");
    let mut a = 0;
    let mut b = 1;
    for _i in 0..10 {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!("\n\nSemua berjalan dengan lancar!");
}
