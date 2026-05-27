use crate::chapters::ChapterData;
use crate::state::game_state::MathLevel;

pub fn chapter_11() -> ChapterData {
    ChapterData {
        number: 11,
        title: "Pemotongan Rel Sempit",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Pintu kubah terbuka. Aku memasuki sistem terowongan bawah tanah peninggalan Belanda yang kini disulap jadi rute penyelundupan.",
            "Sebuah lori tambang bertenaga listrik menantiku, tapi rodanya rusak. Satu-satunya jalan adalah berlari di atas rel besi yang licin.",
            "Sayangnya, di perempatan terowongan, jalur rel utama dibarikade besi las.",
            "Ada lorong sempit berbentuk diagonal yang memotong lurus dari posisiku ke rel cabang di sisi lain.",
            "Udara di sini sangat tipis. Aku harus tahu persis jarak lari sprint ini agar bisa mengatur napasku dengan baik.",
        ],
        puzzle_context: "Jika melihat dari perempatan, terowongan potong kompas ini membentuk sisi miring dari segitiga siku-siku. Jarak ke depan (sisi tegak) adalah 3 meter, dan jarak ke samping (alas) adalah 4 meter.",
        puzzle_question: "Berapa meter jarak lurus (sisi miring) yang harus ku lalui saat berlari sprint lewat lorong potong kompas ini?",
        puzzle_hint: "Gunakan Teorema Pythagoras: a² + b² = c².",
        correct_answers: &["5", "5 meter", "5m"],
        narrative_success: &[
            "Lima meter. Jarak yang sangat pendek, tapi dengan udara minim, ini mematikan.",
            "Aku berlari sekuat tenaga, paru-paruku serasa terbakar, dan tiba di sisi seberang barikade.",
            "Di sini rel kereta tambang terhubung langsung ke permukaan.",
        ],
        narrative_failure: "Napas pendekku membuatku tersandung di tengah kegelapan terowongan.",
        variable_key: Some("tunnel_length"),
        variable_value: Some("5"),
        items_gained: &["Peta Terowongan"],
        health_penalty: 8,
    }
}

pub fn chapter_12() -> ChapterData {
    ChapterData {
        number: 12,
        title: "Lompatan di Atap Gerbong",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Aku merangkak naik melalui ventilasi terowongan dan muncul di halaman stasiun kargo terbengkalai.",
            "Seorang agen Lingkaran Merah melihatku. Ia melompat naik ke atap gerbong kereta kargo.",
            "Aku mengejarnya ke atas atap. Gerbong ini atapnya rata tapi licin karena lumut.",
            "Ia menodongkan pistol. Aku harus melempar pisau lempar milikku ke arahnya untuk melucuti senjatanya.",
            "Dia berdiri di sudut kanan depan atap, dan aku bersembunyi di sudut kiri belakang. Aku butuh mengukur jarak lemparanku.",
        ],
        puzzle_context: "Atap gerbong kargo berbentuk persegi panjang berukuran panjang 8 meter dan lebar 6 meter. Pisau harus dilempar memotong menyilang (diagonal).",
        puzzle_question: "Berapa meter jarak tempuh lurus lemparan pisaumu dari sudut ke sudut atap?",
        puzzle_hint: "Masih menggunakan Pythagoras untuk mencari sisi miring alas balok.",
        correct_answers: &["10", "10 meter", "10m"],
        narrative_success: &[
            "Sepuluh meter. Aku mengambil napas, dan melempar pisau itu sekuat tenaga.",
            "Jleb! Pisau menancap di lengan kanannya. Pistolnya terjatuh ke bawah gerbong.",
            "Ia mengerang dan segera melompat kabur ke dalam kegelapan stasiun, meninggalkan sebuah alat di atas atap.",
        ],
        narrative_failure: "Lemparanku meleset jauh! Pistol menyalak dan pelurunya menyerempet bahuku.",
        variable_key: Some("roof_diagonal"),
        variable_value: Some("10"),
        items_gained: &["Alat Kalibrasi Roda Gigi"],
        health_penalty: 8,
    }
}

pub fn chapter_13() -> ChapterData {
    ChapterData {
        number: 13,
        title: "Overload Generator",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Alat kalibrasi itu tertinggal di atap. Aku membawanya turun ke ruang generator stasiun.",
            "Mereka sedang mencoba membangkitkan generator tua untuk menyalakan kereta kargo.",
            "Aku harus melakukan sabotase (overload) agar mesin generator hancur.",
            "Sistem transmisi generator memakai tiga roda gigi utama yang saling mengunci. Untuk membuat kumparan akhir (Roda C) mencapai batas kritis (putaran tinggi), aku harus memutar engkol manual (Roda A).",
        ],
        puzzle_context: "Roda A terhubung dengan Roda B (rasio putaran 2:3, artinya A putar 2 kali, B putar 3 kali). Roda B terhubung dengan Roda C (rasio 3:5). Roda A harus diputar tepat 10 kali agar C mencapai batas overload tanpa memicu pengaman.",
        puzzle_question: "Jika aku memutar Roda Gigi A sebanyak 10 kali, berapa kali Roda C akan berputar dan menghancurkan dinamonya?",
        puzzle_hint: "Hitung putaran B dulu (10 x 3/2). Lalu kalikan hasil B untuk mencari C (hasil x 5/3).",
        correct_answers: &["25", "25 kali", "25 putaran"],
        narrative_success: &[
            "Dua puluh lima putaran. Aku mengeset regulator keamanan batas akhir ke angka itu.",
            "BUM! Kumparan dinamo terbakar hebat, menyebarkan bau hangus.",
            "Rencana mereka memindahkan kargo ilegal malam ini telah kugagalkan.",
        ],
        narrative_failure: "Rasio salah. Roda gigi macet dan gagang engkol menghantam rahangku.",
        variable_key: Some("clock_turns"),
        variable_value: Some("25"),
        items_gained: &["Logistik Hancur"],
        health_penalty: 8,
    }
}

pub fn chapter_14() -> ChapterData {
    ChapterData {
        number: 14,
        title: "Usia Sang Komisaris",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di ruang pengawas stasiun, aku menemukan dokumen kepegawaian polisi yang telah dienkripsi.",
            "Dokumen ini mengungkap identitas pemimpin rahasia operasi di stasiun ini: Komisaris Surya.",
            "Untuk meretas akses ke radio komunikasinya, aku butuh nomor identifikasi (PIN) yang ternyata adalah umurnya sendiri.",
            "Dokumen ini sengaja disandikan: 'Umur subjek ditambah dua kali lipat umur asistennya sama dengan 90.'",
            "Aku mengenal asistennya. Dia polisi korup yang kutembak di gudang kemarin. Umurnya baru 25 tahun.",
        ],
        puzzle_context: "Persamaan dari dokumen: Umur Surya + 2 * Umur Asisten(25) = 90.",
        puzzle_question: "Berapa umur Komisaris Surya yang harus kugunakan sebagai PIN radio?",
        puzzle_hint: "Bentuk persamaannya: x + 2(25) = 90. Cari nilai x.",
        correct_answers: &["40", "40 tahun"],
        narrative_success: &[
            "Empat puluh tahun. Aku menekan angka itu di panel radio polisi yang disadap.",
            "Saluran terbuka. Terdengar suara parau Komisaris Surya memberi perintah penarikan pasukan ke 'Fasilitas Atas'.",
            "Dia tahu aku mulai mendekatinya.",
        ],
        narrative_failure: "Akses ditolak. Radio memancarkan gelombang suara yang memekakkan telinga.",
        variable_key: Some("sender_age"),
        variable_value: Some("40"),
        items_gained: &["Frekuensi Radio"],
        health_penalty: 8,
    }
}

pub fn chapter_15() -> ChapterData {
    ChapterData {
        number: 15,
        title: "Papan Penyangga Runtuh",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Bermaksud menyusul mereka, aku masuk ke gudang logistik tua yang jembatan papannya sudah lapuk.",
            "Komisaris Surya telah memutus tali baja penahan lantai gudang. Lantainya ambles perlahan ke jurang tambang.",
            "Sebagian besar ubin mulai runtuh. Yang masih kokoh hanya satu area yang ditopang pilar beton tebal di bawahnya.",
            "Aku harus melompat dan berdiri sejajar dengan luas area aman ini, sambil menarik brankas dokumen dari jurang.",
        ],
        puzzle_context: "Area yang masih ditopang pilar adalah kombinasi bentuk persegi (panjang sisi 4m) yang bersambung dengan segitiga siku-siku (alas 4m, tinggi 3m). Keduanya menyatu.",
        puzzle_question: "Berapa meter persegi total luasan lantai aman yang bisa kutempati untuk menarik brankas dokumen?",
        puzzle_hint: "Luas persegi (4x4) ditambah luas segitiga (1/2 x alas x tinggi).",
        correct_answers: &["22", "22 meter persegi", "22m2", "22 m2"],
        narrative_success: &[
            "Dua puluh dua meter persegi. Aku melempar tubuhku ke area tersebut.",
            "Kayu di sekitarnya patah dan jatuh ke dalam jurang gelap berdebu.",
            "Aku berhasil menyelamatkan brankas dokumen itu tepat waktu.",
        ],
        narrative_failure: "Salah perhitungan. Ubin pijakanku patah dan aku tergantung nyaris jatuh.",
        variable_key: Some("safe_area"),
        variable_value: Some("22"),
        items_gained: &["Brankas Dokumen"],
        health_penalty: 8,
    }
}

pub fn chapter_16() -> ChapterData {
    ChapterData {
        number: 16,
        title: "Volume Tangki Vakum",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Brankas dokumen itu disegel vakum secara hidrolik. Tidak bisa dibuka dengan paksa.",
            "Satu-satunya cara membukanya adalah dengan memompa tekanan air ke dalam tabung silinder pendorong di samping brankas, persis hingga tabung itu penuh tanpa sisa.",
            "Jika aku mengisi air kurang atau lebih dari volumenya, tekanan tidak akan setimbang dan mekanisme vakum akan merusak isi brankas secara permanen.",
            "Aku harus menghitung volume tabung silinder ini untuk menakar airnya.",
        ],
        puzzle_context: "Tabung silinder hidrolik ini memiliki jari-jari penampang 7 cm dan tinggi 20 cm.",
        puzzle_question: "Berapa sentimeter kubik (cm³) air yang harus kuinjeksikan ke dalam tabung tersebut? (Gunakan pi = 22/7)",
        puzzle_hint: "Volume tabung = pi x r x r x t.",
        correct_answers: &["3080", "3080 cm3", "3080cm3"],
        narrative_success: &[
            "Tiga ribu delapan puluh sentimeter kubik. Aku menggunakan suntikan pengukur dan menyuntikkannya perlahan.",
            "Tekanan naik. Terdengar bunyi desisan udara keluar dari sela-sela baja brankas.",
            "Kait hidrolik terbuka. Di dalamnya, terdapat peta rahasia kota.",
        ],
        narrative_failure: "Airnya luber. Segel mekanis menekan kuat tanganku.",
        variable_key: Some("tank_volume"),
        variable_value: Some("3080"),
        items_gained: &["Peta Markas"],
        health_penalty: 10,
    }
}

pub fn chapter_17() -> ChapterData {
    ChapterData {
        number: 17,
        title: "Jarak Evakuasi",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Peta itu menunjukkan lokasi 'Fasilitas Atas'—sebuah benteng pertahanan di luar kota.",
            "Tapi peta militer tua ini menggunakan skala kecil, dan tidak menyebutkan jarak kilometernya.",
            "Helikopter polisi yang kuhubungi butuh radius koordinat yang presisi dari stasiun ini.",
            "Aku harus mengonversi jarak di peta ke jarak sebenarnya agar pilot bisa menentukan rute evakuasi di ujung perbukitan.",
        ],
        puzzle_context: "Skala pada peta adalah 1 : 25.000. Jarak dari stasiun menuju fasilitas di gambar peta adalah persis 8 cm.",
        puzzle_question: "Berapa meter jarak nyata (sebenarnya) yang harus kuberikan ke pilot helikopter?",
        puzzle_hint: "Kalikan 8 dengan 25.000 untuk dapat sentimeter (cm), lalu ubah ke meter dengan dibagi 100.",
        correct_answers: &["2000", "2000 meter", "2000m", "2 km"],
        narrative_success: &[
            "Dua ribu meter. Tepat dua kilometer ke arah perbukitan.",
            "Aku menyalakan suar transmisi dan mengirimkan titik temu tersebut.",
            "Tugas selanjutnya: menyusul Komisaris Surya ke sana.",
        ],
        narrative_failure: "Jaraknya salah. Helikopter berputar tanpa arah di perbukitan gelap.",
        variable_key: Some("real_distance"),
        variable_value: Some("2000"),
        items_gained: &["Koordinat Fasilitas"],
        health_penalty: 10,
    }
}

pub fn chapter_18() -> ChapterData {
    ChapterData {
        number: 18,
        title: "Tembakan Jitu Sang Komisaris",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Aku berlari membelah hujan malam, tiba di pelataran dasar benteng di perbukitan.",
            "Tiba-tiba, suara tembakan memecah kesunyian. Tembakan sniper dari atas tembok benteng!",
            "Peluru itu menghancurkan lampu sorot persis di atasku.",
            "Lampu yang hancur berada di tinggi 3 meter dari tanah.",
            "Aku bersembunyi di balik barikade karung pasir. Untuk menembak balik tepat ke sarangnya, aku harus mengestimasi jarak lintasan lurus laras senapannya.",
            "Cahaya moncong senapan tadi berasal dari titik di tanah yang berjarak horisontal 4 meter dari lampu sorot.",
        ],
        puzzle_context: "Posisi penembak berjarak horisontal 4m dari titik jatuhnya peluru, dan peluru bersarang di dinding setinggi 3m.",
        puzzle_question: "Berapa meter jarak lintasan lurus peluru yang melesat dari penembak ke lampu sorot?",
        puzzle_hint: "Gunakan Theorema Pythagoras (alas 4m, tinggi 3m).",
        correct_answers: &["5", "5 meter", "5m"],
        narrative_success: &[
            "Lima meter! Dia berada sangat dekat, bersembunyi di parit pertahanan luar, bukan di menara!",
            "Aku menembak membabi-buta ke arah sudut lima meter itu.",
            "Terdengar pekikan kesakitan. Aku berhasil mengenainya, namun ia membalas tembakan sambil mundur ke dalam benteng.",
        ],
        narrative_failure: "Bidikanku meleset ke udara kosong. Sniper itu malah hampir menembak tembus pelindung barikadeku.",
        variable_key: Some("bullet_distance"),
        variable_value: Some("5"),
        items_gained: &["Parit Terbuka"],
        health_penalty: 10,
    }
}

pub fn chapter_19() -> ChapterData {
    ChapterData {
        number: 19,
        title: "Daya Angkat Pelampung",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Melompat ke parit pertahanan yang menggenang air hujan, aku menemukan sebuah gerbang besi bermotor.",
            "Pintu gerbang basah ini dihubungkan dengan mekanisme pelampung mekanik berat.",
            "Untuk menaikkan gerbangnya, pelampung kotak besi pengontrol pompa harus dicelupkan seluruhnya ke air genangan parit.",
            "Buku panduan pompa menyebutkan, gerbang hanya akan terangkat jika air dipindahkan (displacement) sejumlah liter yang setara dengan volume pelampung baja.",
        ],
        puzzle_context: "Pelampung baja itu berbentuk kubus tebal dengan sisi 30 cm. Ingat bahwa 1000 cm³ sama dengan 1 Liter.",
        puzzle_question: "Berapa liter jumlah air yang akan dipindahkan saat pelampung kubus ini ditenggelamkan sepenuhnya (sama dengan volume kotak)?",
        puzzle_hint: "Volume kubus = s x s x s (dalam cm). Konversi ke liter.",
        correct_answers: &["27", "27 liter", "27L", "27 l"],
        narrative_success: &[
            "Dua puluh tujuh liter. Pompa mulai berdengung keras dan menyedot airnya.",
            "Sistem hidrolik bekerja berat akibat daya angkat pelampung baja tersebut.",
            "Pintu baja berkarat itu mulai terbuka, mengizinkan aku masuk ke koridor utama benteng.",
        ],
        narrative_failure: "Air tidak terpompa cukup karena tuas mekanik tersumbat lumpur akibat perhitunganku yang salah.",
        variable_key: Some("box_volume"),
        variable_value: Some("27"),
        items_gained: &["Akses Benteng"],
        health_penalty: 10,
    }
}

pub fn chapter_20() -> ChapterData {
    ChapterData {
        number: 20,
        title: "Kabel Rem Lift Menuju Puncak",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Koridor ini berujung di sebuah poros lift vertikal untuk suplai senjata.",
            "Komisaris Surya telah naik ke atas dan menyabotase sistem rem kabel lift ini.",
            "Terdapat empat kabel baja (A, B, C, D) penyangga lift. Surya telah memotong sebagian agar lift ini jatuh jika kabel terakhir tidak ditegangkan dengan batas maksimal.",
            "Sensor beban menyatakan kapasitas total tegangan keempat kabel harus tepat menahan 30 metrik ton.",
            "Aku telah melihat data tegangan batas kabel-kabel lainnya di panel maintenance sepanjang perjalananku.",
        ],
        puzzle_context: "Tegangan kabel A adalah 5 ton (Ch11), kabel B 10 ton (Ch12), kabel C 5 ton (Ch18). Total yang dibutuhkan adalah 30 ton.",
        puzzle_question: "Berapa ton sisa tegangan yang harus kutarik secara manual pada kabel D agar lift tidak meluncur jatuh?",
        puzzle_hint: "Kurangi total 30 dengan jumlah tegangan ketiga kabel lainnya (5 + 10 + 5).",
        correct_answers: &["10", "10 ton", "10 t"],
        narrative_success: &[
            "Sepuluh ton! Aku memutar tuas katrol darurat dengan mengerahkan seluruh tenagaku.",
            "Bunyi kertakan logam yang ngilu memekakkan telinga, tapi rem mekanis mengunci dengan sempurna.",
            "Lift mulai naik perlahan, membawaku langsung ke sarang sang Komisaris.",
            "Tunggu pembalasanku, Surya.",
        ],
        narrative_failure: "Tali katrol terlepas, lift sempat anjlok satu lantai membuat tulang rusukku seperti retak.",
        variable_key: Some("final_tunnel"),
        variable_value: Some("10"),
        items_gained: &["Kabel Rem Katrol"],
        health_penalty: 10,
    }
}
