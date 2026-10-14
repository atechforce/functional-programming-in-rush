// =========================================
// Contoh 2: Membuat Custom Iterator Sendiri
// dengan mengimplementasikan trait Iterator
// =========================================

// Struct yang akan menjadi iterator
struct HitungMundur {
    nilai: i32,
}

impl HitungMundur {
    fn baru(mulai: i32) -> HitungMundur {
        HitungMundur { nilai: mulai }
    }
}

// Implementasi trait Iterator
impl Iterator for HitungMundur {
    type Item = i32; // tipe data yang dikembalikan setiap next()

    fn next(&mut self) -> Option<Self::Item> {
        if self.nilai > 0 {
            let sekarang = self.nilai;
            self.nilai -= 1;
            Some(sekarang)
        } else {
            None // iterator selesai
        }
    }
}

// ----------------------------------------
// Contoh kedua: Iterator bilangan genap
// ----------------------------------------
struct BilanganGenap {
    sekarang: u32,
    batas: u32,
}

impl BilanganGenap {
    fn baru(batas: u32) -> BilanganGenap {
        BilanganGenap { sekarang: 0, batas }
    }
}

impl Iterator for BilanganGenap {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.sekarang += 2;
        if self.sekarang <= self.batas {
            Some(self.sekarang)
        } else {
            None
        }
    }
}

fn main() {
    // Hitung mundur dari 5
    println!("=== Hitung Mundur dari 5 ===");
    let hm = HitungMundur::baru(5);
    for n in hm {
        println!("  {}", n);
    }

    // Bilangan genap sampai 10
    println!("\n=== Bilangan Genap hingga 10 ===");
    let genap = BilanganGenap::baru(10);
    for n in genap {
        println!("  {}", n);
    }

    // Karena Iterator, semua metode bawaan bisa dipakai!
    println!("\n=== Menggunakan .sum() pada custom iterator ===");
    let total: i32 = HitungMundur::baru(10).sum();
    println!("  Jumlah 1..=10 = {}", total);
}
