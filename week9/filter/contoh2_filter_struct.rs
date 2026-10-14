// =========================================
// Contoh 2: filter() pada Struct dan Data Kompleks
// =========================================

#[derive(Debug)]
struct Produk {
    nama: String,
    harga: f64,
    stok: u32,
    kategori: String,
}

impl Produk {
    fn baru(nama: &str, harga: f64, stok: u32, kategori: &str) -> Produk {
        Produk {
            nama: nama.to_string(),
            harga,
            stok,
            kategori: kategori.to_string(),
        }
    }
}

fn main() {
    let produk = vec![
        Produk::baru("Laptop",   12_000_000.0, 5,  "Elektronik"),
        Produk::baru("Mouse",       250_000.0, 20, "Elektronik"),
        Produk::baru("Baju",        150_000.0, 0,  "Fashion"),
        Produk::baru("Celana",      200_000.0, 10, "Fashion"),
        Produk::baru("Headphone", 1_500_000.0, 3,  "Elektronik"),
        Produk::baru("Sepatu",      500_000.0, 0,  "Fashion"),
        Produk::baru("Keyboard",    450_000.0, 8,  "Elektronik"),
    ];

    // --- Filter produk yang masih ada stok ---
    println!("=== Produk TERSEDIA (stok > 0) ===");
    let tersedia: Vec<&Produk> = produk.iter()
        .filter(|p| p.stok > 0)
        .collect();
    for p in &tersedia {
        println!("  {:15} | stok: {:2} | Rp {:>12.0}", p.nama, p.stok, p.harga);
    }

    // --- Filter berdasarkan kategori ---
    println!("\n=== Produk Kategori ELEKTRONIK ===");
    let elektronik: Vec<&Produk> = produk.iter()
        .filter(|p| p.kategori == "Elektronik")
        .collect();
    for p in &elektronik {
        println!("  {} (Rp {:.0})", p.nama, p.harga);
    }

    // --- Filter harga di bawah budget ---
    let budget = 500_000.0_f64;
    println!("\n=== Produk di bawah Budget Rp {:.0} ===", budget);
    let terjangkau: Vec<&Produk> = produk.iter()
        .filter(|p| p.harga <= budget && p.stok > 0)
        .collect();
    for p in &terjangkau {
        println!("  {} → Rp {:.0}", p.nama, p.harga);
    }

    // --- Kombinasi filter ganda ---
    println!("\n=== Elektronik TERSEDIA dengan harga < 1 juta ===");
    let rekomendasi: Vec<&Produk> = produk.iter()
        .filter(|p| p.kategori == "Elektronik")
        .filter(|p| p.harga < 1_000_000.0)
        .filter(|p| p.stok > 0)
        .collect();
    for p in &rekomendasi {
        println!("  ✅ {} (Rp {:.0})", p.nama, p.harga);
    }
}
