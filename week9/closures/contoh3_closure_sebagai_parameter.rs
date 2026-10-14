// =========================================
// Contoh 3: Closure sebagai Parameter Fungsi
// Menggunakan trait: Fn, FnMut, FnOnce
// =========================================

// Fn  → closure bisa dipanggil berkali-kali, hanya borrow immutable
fn terapkan<F: Fn(i32) -> i32>(nilai: i32, fungsi: F) -> i32 {
    fungsi(nilai)
}

// FnMut → closure bisa dipanggil berkali-kali, boleh ubah state internal
fn ulangi<F: FnMut()>(mut aksi: F, kali: u32) {
    for _ in 0..kali {
        aksi();
    }
}

// FnOnce → closure hanya dipanggil sekali (biasanya karena move)
fn jalankan_sekali<F: FnOnce() -> String>(pembuat: F) -> String {
    pembuat()
}

fn main() {
    // Fn: kirim closure ke fungsi
    let hasil = terapkan(8, |x| x * x);
    println!("Kuadrat 8 = {}", hasil);

    let hasil2 = terapkan(10, |x| x + 5);
    println!("10 + 5 = {}", hasil2);

    // FnMut: closure dengan state
    let mut jumlah = 0;
    ulangi(|| {
        jumlah += 1;
        println!("  iterasi ke-{}", jumlah);
    }, 3);

    // FnOnce: closure yang mengambil ownership
    let nama = String::from("Rust");
    let pesan = jalankan_sekali(move || format!("Selamat belajar {}!", nama));
    println!("{}", pesan);
}
