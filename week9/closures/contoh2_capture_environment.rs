// =========================================
// Contoh 2: Closure Menangkap (Capture) Variabel
// dari Lingkungan Sekitarnya (Environment)
// =========================================

fn main() {
    // --- Capture by reference (immutable borrow) ---
    let angka = 10;
    let tambah_angka = |x| x + angka; // closure "meminjam" angka
    println!("5 + angka({}) = {}", angka, tambah_angka(5));

    // --- Capture by mutable reference ---
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Counter sekarang: {}", counter);
    };
    increment();
    increment();
    increment();
    // counter tidak bisa dipakai di sini selama `increment` masih hidup
    drop(increment); // lepaskan borrow
    println!("Counter akhir: {}", counter);

    // --- Capture by move (memindahkan ownership) ---
    let pesan = String::from("Halo Rust!");
    let cetak_pesan = move || println!("Pesan: {}", pesan);
    cetak_pesan();
    // println!("{}", pesan); // ERROR: pesan sudah di-move ke dalam closure
}
