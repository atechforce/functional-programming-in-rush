// =========================================
// Contoh 2: map() dengan Struct dan Transformasi Data
// =========================================

#[derive(Debug)]
struct Mahasiswa {
    nama: String,
    nilai: f64,
}

#[derive(Debug)]
struct HasilNilai {
    nama: String,
    nilai: f64,
    predikat: String,
}

fn tentukan_predikat(nilai: f64) -> String {
    match nilai as u32 {
        90..=100 => String::from("A"),
        80..=89  => String::from("B"),
        70..=79  => String::from("C"),
        60..=69  => String::from("D"),
        _        => String::from("E"),
    }
}

fn main() {
    let mahasiswa = vec![
        Mahasiswa { nama: String::from("Andi"),  nilai: 92.5 },
        Mahasiswa { nama: String::from("Budi"),  nilai: 78.0 },
        Mahasiswa { nama: String::from("Cinta"), nilai: 85.3 },
        Mahasiswa { nama: String::from("Dino"),  nilai: 61.0 },
        Mahasiswa { nama: String::from("Elsa"),  nilai: 55.0 },
    ];

    // Transformasi Mahasiswa → HasilNilai
    let hasil: Vec<HasilNilai> = mahasiswa.iter()
        .map(|m| HasilNilai {
            nama: m.nama.clone(),
            nilai: m.nilai,
            predikat: tentukan_predikat(m.nilai),
        })
        .collect();

    println!("{:<10} {:>8} {:>10}", "Nama", "Nilai", "Predikat");
    println!("{}", "-".repeat(30));
    for h in &hasil {
        println!("{:<10} {:>8.1} {:>10}", h.nama, h.nilai, h.predikat);
    }

    // Hanya ambil nama mahasiswa saja (map ke String)
    println!("\n=== Daftar nama saja ===");
    let nama_saja: Vec<&str> = mahasiswa.iter()
        .map(|m| m.nama.as_str())
        .collect();
    println!("{:?}", nama_saja);

    // Naikkan semua nilai sebesar 5 poin (bonus)
    println!("\n=== Setelah bonus +5 ===");
    let setelah_bonus: Vec<f64> = mahasiswa.iter()
        .map(|m| (m.nilai + 5.0).min(100.0))
        .collect();
    println!("{:?}", setelah_bonus);
}
