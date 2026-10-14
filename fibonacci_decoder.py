v bdef decode_fibonacci(code: str) -> int:
    """
    Mendecode string biner Fibonacci tunggal (misal: '001011') menjadi bilangan bulat.
    """
    code = code.strip()
    if not code.endswith("11"):
        raise ValueError(f"Kode '{code}' tidak valid (harus diakhiri dengan '11')")
    
    bits = code[:-1]  # buang stop bit terakhir
    
    # Generate deret Fibonacci: 1, 2, 3, 5, 8, 13, ...
    fib = [1, 2]
    while len(fib) < len(bits):
        fib.append(fib[-1] + fib[-2])
    
    # Jumlahkan nilai Fibonacci yang bit-nya bernilai '1'
    total = sum(fib[i] for i, bit in enumerate(bits) if bit == "1")
    return total


def decode_stream(stream: str) -> list[int]:
    """
    Mendecode rentetan bit biner yang berisi satu atau beberapa bilangan Fibonacci.
    Contoh: '00101111' -> [11, 1]
    """
    stream = stream.strip()
    numbers = []
    current = ""
    
    i = 0
    while i < len(stream):
        current += stream[i]
        # Jika menemukan '11', berarti satu bilangan selesai di-encode
        if len(current) >= 2 and current.endswith("11"):
            numbers.append(decode_fibonacci(current))
            current = ""
        i += 1
        
    if current:
        print(f"[Peringatan] Sisa bit '{current}' tidak lengkap / tidak berakhiran '11'.")
        
    return numbers


if __name__ == "__main__":
    print("=== FIBONACCI CODE DECODER ===")
    
    # Contoh demonstrasi
    contoh_data = {
        "11": 1,
        "011": 2,
        "0011": 3,
        "1011": 4,
        "00011": 5,
        "100011": 9,
        "001011": 11,
    }
    
    print("\n--- Contoh Dekode Tunggal ---")
    for kode, expected in contoh_data.items():
        hasil = decode_fibonacci(kode)
        print(f"Kode: {kode:<8} -> Hasil: {hasil:<3} (Sesuai: {hasil == expected})")
    
    print("\n--- Contoh Dekode Stream Gabungan ---")
    stream_contoh = "00101111100011"  # 11, 1, 9
    print(f"Stream: {stream_contoh}")
    print(f"Hasil : {decode_stream(stream_contoh)}")
    
    print("\n------------------------------")
    user_input = input("Masukkan kode biner Fibonacci untuk didecode (atau tekan Enter untuk keluar): ").strip()
    if user_input:
        try:
            if " " in user_input:
                for k in user_input.split():
                    print(f"Hasil '{k}': {decode_fibonacci(k)}")
            elif user_input.count("11") > 1:
                print(f"Hasil Stream: {decode_stream(user_input)}")
            else:
                print(f"Hasil: {decode_fibonacci(user_input)}")
        except Exception as e:
            print(f"Error: {e}")
