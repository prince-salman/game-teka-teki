use crate::chapters::ChapterData;
use crate::state::game_state::MathLevel;

pub fn chapter_21() -> ChapterData {
    ChapterData {
        number: 21,
        title: "Pemotongan Kabel Keamanan",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Lift mendesing berhenti. Aku telah mencapai ruang bawah benteng puncak gunung, markas operasional terakhir Lingkaran Merah.",
            "Di ujung aula baja, pintu masuk utama terkunci mati. Untuk meretas masuk, aku telah membongkar panel kontrol elektronik di dinding.",
            "Panel ini merupakan sistem keamanan buatan Jerman dengan aturan kabel yang saling bergantung satu sama lain untuk mencegah sabotase.",
            "Di dalam manual operasi yang tertempel, tertulis peringatan keras terkait sirkuit lampu detektor.",
            "Jika aku mengambil tindakan gegabah, benteng ini akan langsung memasuki status *Lockdown* dan gas beracun akan dilepaskan.",
        ],
        puzzle_context: "Manual instruksi logika sirkuit berbunyi: 'Premis 1: Jika Relay Merah diaktifkan, maka Alarm Sistem akan berbunyi.' Saat kulacak arus voltasenya, kulihat dengan jelas: 'Premis 2: Relay Merah saat ini sedang AKTIF (menyala)'.",
        puzzle_question: "Berdasarkan prinsip Modus Ponens, apa kesimpulan logis mutlak yang akan langsung terjadi sekarang dari sistem ini?",
        puzzle_hint: "Modus Ponens: Jika P maka Q. P terjadi. Maka kesimpulannya adalah Q.",
        correct_answers: &["alarm berbunyi", "alarmberbunyi", "maka alarm berbunyi", "alarm sistem berbunyi"],
        narrative_success: &[
            "Alarm berbunyi! Sistem ini sengaja menjebakku, alarm sudah dirancang untuk berbunyi apapun yang terjadi.",
            "Aku memutus kabel sirine utama tepat detik berikutnya sehingga peringatan ke pos jaga berhasil dicegah.",
            "Pintu baja bergeser terbuka, dan aku lolos dari deteksi awal.",
        ],
        narrative_failure: "Bukan itu hasilnya. Kesalahan analisaku membuat panel menolak input *bypass*, malah mengunci satu lapis engsel tambahan.",
        variable_key: Some("gate_logic"),
        variable_value: Some("alarm"),
        items_gained: &["Akses Panel Kunci"],
        health_penalty: 10,
    }
}

pub fn chapter_22() -> ChapterData {
    ChapterData {
        number: 22,
        title: "Kait Jembatan Putus",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Masuk lebih dalam, aku dihadapkan pada jembatan hidrolik yang posisinya terbuka tegak ke atas, menghalangi jalan menuju paviliun utara.",
            "Ujung jembatan itu memiliki tuas manual yang sayangnya berada di puncak struktur menara kecil di sisi jembatan tersebut.",
            "Satu-satunya cara untuk mencapainya adalah dengan menembakkan panah tali *grappling hook* tepat mengenai tuas pemicu di atas menara itu.",
            "Masalahnya, tabung pelontar panahku memiliki indikator tekanan hidrolik yang sangat bergantung pada tinggi lintasan terbang.",
            "Jika tekanannya salah, panahku hanya akan menabrak baja jembatan dan melenting kembali.",
        ],
        puzzle_context: "Aku berdiri sejauh 10 meter dari dasar menara pemicu. Kuukur dengan *clinometer* di jam tanganku, sudut elevasi tepat dari posisiku ke puncak tuas adalah 45 derajat.",
        puzzle_question: "Berapa meter ketinggian tuas menara tersebut dari permukaan tanah tempatku berdiri?",
        puzzle_hint: "Gunakan Trigonometri. tan(45 derajat) = tinggi / jarak. Ingat bahwa nilai tan(45°) = 1.",
        correct_answers: &["10", "10 meter", "10m"],
        narrative_success: &[
            "Sepuluh meter. Aku memompa tekanan gas pelontar sesuai spesifikasi tersebut.",
            "Zwoosh! Klank! Panah baja kait mencengkeram kuat tuas di atas sana.",
            "Kutarik tali dengan sekuat tenaga, dan jembatan hidrolik itu akhirnya turun berdentang ke lantai dasar.",
        ],
        narrative_failure: "Kaitnya meleset! Tali itu mengenai baja penyeimbang dan terlempar menghajar helm proyekku dengan keras.",
        variable_key: Some("tower_height"),
        variable_value: Some("10"),
        items_gained: &["Panah Kait Tertanam"],
        health_penalty: 10,
    }
}

pub fn chapter_23() -> ChapterData {
    ChapterData {
        number: 23,
        title: "Kebohongan Evakuasi",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Aku menemukan ruang CCTV keamanan. Di layarnya, terlihat Komisaris Surya berbicara lewat radio dengan pasukan helikopter di atap.",
            "Aku berhasil meretas frekuensi komunikasi mereka menggunakan radio polisi yang kudapat di stasiun kargo sebelumnya.",
            "Surya berbohong pada pilot: 'Aku berada di dalam rumah persembunyian (Bunker Darat). Bersiaplah mengudara tanpa aku.'",
            "Pilot membalas: 'Tapi Pak, sesuai protokol standar, jika Anda sedang berada di rumah persembunyian, maka mobil taktis Anda seharusnya terparkir di landasan pelataran!'",
            "Aku menoleh ke layar monitor CCTV yang menyorot pelataran darat... dan benar saja, mobil taktis miliknya sama sekali TIDAK terparkir di sana.",
        ],
        puzzle_context: "Kita evaluasi fakta deduktif ini: 'Jika Surya ada di rumah, maka mobilnya terparkir. Fakta di lapangan: mobilnya TIDAK terparkir.'",
        puzzle_question: "Berdasarkan prinsip Modus Tollens, apa kesimpulan logis nyata mengenai keberadaan Surya saat ini?",
        puzzle_hint: "Modus Tollens: Jika P maka Q. Ternyata Tidak Q. Kesimpulannya adalah Tidak P.",
        correct_answers: &["dia tidak ada di rumah", "tidakadadirumah", "tidak di rumah", "surya tidak di rumah", "dia tidak di rumah"],
        narrative_success: &[
            "Tentu saja, Surya TIDAK ADA DI RUMAH bunker! Dia sengaja menipu helikopternya agar dijadikan umpan pengalih perhatianku.",
            "Dia masih bersembunyi di suatu tempat di dalam paviliun utara benteng ini.",
            "Licik sekali pria tua itu, mengorbankan pasukannya demi keselamatannya.",
        ],
        narrative_failure: "Asumsiku keliru. Jika aku terkecoh oleh logikanya, aku akan membuang waktu mengejar target ke lokasi yang salah.",
        variable_key: Some("alibi_broken"),
        variable_value: Some("true"),
        items_gained: &["Radio Frekuensi Heli"],
        health_penalty: 10,
    }
}

pub fn chapter_24() -> ChapterData {
    ChapterData {
        number: 24,
        title: "Lantai Laser Berpola",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Aku mendobrak pintu paviliun utara, dan mendapati diriku berada di lorong jebakan laser pendeteksi gerak.",
            "Lantai aula besar ini terbagi ke dalam kotak-kotak ubin logam bernomor seri berurutan dari 1 hingga 100.",
            "Beberapa ubin memiliki pelat penyeimbang yang tidak akan memicu sensor laser. Jejak kaki samar Surya menunjukkan pola pijakannya.",
            "Surya telah memijak ubin nomor 3, lalu 8, lalu 13, 18, 23, dan terus begitu membentuk barisan aritmatika yang sistematis.",
            "Langkahku berikutnya harus tepat meniru pola ini, karena ubin lainnya terhubung langsung ke menara senapan mesin otomatis di ujung aula.",
            "Aku harus melompat sejauh sepuluh langkah untuk bisa mencapai sisi seberang tanpa terdeteksi.",
        ],
        puzzle_context: "Ubin pijakan yang aman mengikuti barisan aritmatika: 3, 8, 13, 18, 23, ...",
        puzzle_question: "Berapa nomor ubin yang harus kuinjak persis pada langkah ke-10 (U_10)?",
        puzzle_hint: "Gunakan rumus suku ke-n aritmatika: U(n) = a + (n-1)b. Di sini a=3, dan selisih b=5.",
        correct_answers: &["48", "ubin 48"],
        narrative_success: &[
            "Ubin nomor 48! Aku mengambil ancang-ancang dan melompat memutar, menghindari jaringan laser di udara.",
            "Kaki bot-ku mendarat keras tepat di kotak bernomor 48.",
            "Berhasil! Lampu sensor senapan mesin tak merespons. Sistem senjatanya dapat kuraih dan kumatikan secara manual.",
        ],
        narrative_failure: "KRAK! Aku menginjak pinggiran ubin yang salah. Laser merah menyorotku tajam, dan sebutir peluru senapan mesin nyaris menyerempet kepalaku.",
        variable_key: Some("safe_tile"),
        variable_value: Some("48"),
        items_gained: &["Sensor Penonaktif Laser"],
        health_penalty: 10,
    }
}

pub fn chapter_25() -> ChapterData {
    ChapterData {
        number: 25,
        title: "Protokol Peledak Diri",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Menyadari aku telah lolos dari sistem lasernya, Surya mengaktifkan protokol bunuh diri (Self-Destruct) seluruh sektor benteng.",
            "Di ujung lorong, panel bom termit terhubung langsung ke pintu baja komando. Aku tak bisa lari keluar.",
            "Sistem pemicu komputernya sangat aneh. Bom ini tidak menggunakan hitung mundur linier 1 detik biasa.",
            "Indikator timer-nya memproses pelepasan enkripsi lapis demi lapis, setiap lapis memakan waktu separuh dari lapis sebelumnya (Deret Geometri).",
            "Untuk meretas inti detonator, aku harus tahu persis berapa *total* waktu riil di dunia nyata yang kupunya sebelum proses komputasi ini mencapai lapis terakhir dan meledak.",
        ],
        puzzle_context: "Proses dekripsi bom lapis 1 makan waktu 32 detik. Lapis kedua 16 detik. Ketiga 8 detik. Keempat 4 detik, kelima 2 detik, dan lapis keenam (terakhir) 1 detik.",
        puzzle_question: "Berapa detik total waktu riil yang kumiliki dari awal hingga akhir (jumlah 6 lapis) sebelum bom termit meledak?",
        puzzle_hint: "Jumlahkan semuanya: 32 + 16 + 8 + 4 + 2 + 1.",
        correct_answers: &["63", "63 detik", "63s"],
        narrative_success: &[
            "Enam puluh tiga detik. Sebentar sekali!",
            "Tanganku gemetar. Di detik ke-61, kutarik kabel busi komputasinya secara paksa.",
            "Angka merah di layar mati, bertepatan dengan padamnya lampu strobo bahaya.",
        ],
        narrative_failure: "Hitunganku salah! Salah satu tabung termit meletup kecil, menyemburkan api putih yang membakar lengan bajuku.",
        variable_key: Some("bomb_time"),
        variable_value: Some("63"),
        items_gained: &["Inti Busi Bom"],
        health_penalty: 10,
    }
}

pub fn chapter_26() -> ChapterData {
    ChapterData {
        number: 26,
        title: "Pemutar Balik Arus Matriks",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Pintu baja terbuka, menampilkan Ruang Pengendali Utama.",
            "Di baliknya, Surya mengunci dirinya dalam kubah kaca Vault transparan berlapis baja.",
            "Sistem kaca vault ini memiliki panel tenaga elektrik bertenaga koil vektor.",
            "Catatan mekanis di sirkuit luar mengindikasikan bahwa arus tenaga dikontrol oleh operasi perkalian matriks untuk membalik arah voltase.",
            "Aku telah mempelajari susunan relai matriks koil dari cetak biru yang kutemukan di ruang kerjanya dulu. Kini saatnya mempraktikkannya secara nyata.",
        ],
        puzzle_context: "Matriks daya koil (A) bernilai [[1, 2], [3, 4]] dan arus suplai (vektor B) adalah [5, 6]. Hasil pembalikan arus adalah sebuah vektor daya akhir [x, y].",
        puzzle_question: "Berapa kapasitas total daya pembalikan tegangan (x + y) agar kumparan magnet vault tersebut membalikkan arah kuncinya?",
        puzzle_hint: "Kalikan matriks dengan vektor: x = (1x5) + (2x6) = 17. y = (3x5) + (4x6) = 39. Berapa total x + y?",
        correct_answers: &["56"],
        narrative_success: &[
            "Lima puluh enam kilovolt pembalikan! Aku menancapkan inti busi bom sebelumnya ke panel untuk menyalurkan listrik.",
            "Percikan listrik menyambar hebat, menggeser seluruh magnet polarisasi kubah vault.",
            "Kubah kaca berdengung dan terbuka secara paksa dengan asap putih menyembur ke segala arah.",
        ],
        narrative_failure: "Kelebihan beban! Arus tidak membalik melainkan menolak mentah-mentah kabel jumper-ku, menyetrum tanganku dengan menyakitkan.",
        variable_key: Some("matrix_code"),
        variable_value: Some("56"),
        items_gained: &["Komponen Relai Matriks"],
        health_penalty: 12,
    }
}

pub fn chapter_27() -> ChapterData {
    ChapterData {
        number: 27,
        title: "Reaksi Asam Pembakar",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Surya terpojok. Tanpa pikir panjang, ia melempar tabung gas pelumpuh syaraf berwarna hijau pekat ke arahku.",
            "Gas itu langsung membakar saluran pernapasanku. Aku jatuh terduduk, terbatuk darah.",
            "Di meja lab komandonya, terdapat tabung reaksi dan jurnal kimia mengenai penetralisir agen syaraf tersebut.",
            "Aku harus mencampur senyawa larutan (X dan Y) secara cepat sebelum saraf otakku hancur.",
            "Sesuai jurnal, konsentrasi yang bisa menyelamatkanku harus memenuhi dua reaksi linear kesetimbangan asam secara simultan.",
        ],
        puzzle_context: "Reaksi stabil jika dan hanya jika: 2X + 3Y = 21, dan X + Y = 8.",
        puzzle_question: "Berapa mililiter (ml) takaran senyawa X yang harus segera kusuntikkan ke tubuhku?",
        puzzle_hint: "Gunakan substitusi. Dari persamaan kedua: Y = 8 - X. Masukkan ke persamaan pertama: 2X + 3(8-X) = 21.",
        correct_answers: &["3", "3 ml", "3ml"],
        narrative_success: &[
            "Tiga mililiter! Aku menarik cairannya dengan jarum darurat, menekan panik, dan menyuntikkannya ke paha kananku.",
            "Denyut nadiku berdegup kencang, pembuluh darahku terasa membara, tapi efek gas pelumpuh itu sirna seketika.",
            "Aku bangkit, menyeka darah di mulutku, menatap tajam Surya yang mulai bergetar.",
        ],
        narrative_failure: "Kesadaranku mengabur! Takaran yang salah membuatku mual hebat dan organ dalamku melilit kesakitan.",
        variable_key: Some("chemical_x"),
        variable_value: Some("3"),
        items_gained: &["Alat Suntik Bekas"],
        health_penalty: 12,
    }
}

pub fn chapter_28() -> ChapterData {
    ChapterData {
        number: 28,
        title: "Pengejaran Tali Udara",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Ketakutan melihatku kebal terhadap gasnya, Surya berlari keluar, melompat ke balkon terbuka yang menghadap tebing berbatu karang.",
            "Ia meraih seutas tali katrol gantung panjang (zipline) dan meluncur deras ke stasiun pendaratan perahu rahasia di bawah jurang.",
            "Ia memotong tali mekanis sehingga aku tidak bisa menyusul. Namun, aku masih membawa pelontar panah kait bajaku.",
            "Aku bisa menembakkan tali baru ke tiang penyangga stasiun perahu itu, membentuk jalur luncurku sendiri.",
            "Aku hanya punya satu *winch* sisa, dan jika talinya terlalu tegang atau terlalu panjang, aku akan menabrak batu karang saat meluncur turun.",
        ],
        puzzle_context: "Ketinggian vertikal (tebing) ke titik target adalah 12 meter. Tali akan meluncur membentuk sudut kemiringan 30 derajat dari dataran atas menuju tiang pendaratan bawah.",
        puzzle_question: "Berapa meter ukuran panjang peregangan tali yang harus ku set pada pelontar kait baja ini? (Gunakan sin 30° = 0.5)",
        puzzle_hint: "sin(sudut) = tinggi vertikal / sisi miring panjang tali. Panjang tali = 12 / 0.5.",
        correct_answers: &["24", "24 meter", "24m"],
        narrative_success: &[
            "Dua puluh empat meter. Aku memasang kunci tuas dan menembakkan *grappling hook*-ku membelah angin malam.",
            "Kait baja berdenting sempurna melilit tiang. Tanpa ragu, kulempar diriku menuruni garis diagonal itu.",
            "Aku meluncur deras mengejar bayangannya menembus kabut laut.",
        ],
        narrative_failure: "Panjangnya meleset! Talinya menggelendot kendur, punggungku menghantam karang bergerigi sebelum aku menahan diri.",
        variable_key: Some("rope_length"),
        variable_value: Some("24"),
        items_gained: &["Sisa Tali Kait"],
        health_penalty: 12,
    }
}

pub fn chapter_29() -> ChapterData {
    ChapterData {
        number: 29,
        title: "Perintah Penghentian Total",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Kakiku menendang dada Surya tepat saat ia mencoba menyalakan perahunya.",
            "Kami berguling di lantai baja pendaratan karang. Pistolnya terlempar, ia tak lagi bisa melawan.",
            "Meski kalah, Surya tertawa gila dan menekan detonator pemusnah masal. Ini bukan hanya meledakkan markas ini, tapi sistem bendungan kota yang dikendalikan Lingkaran Merah.",
            "'Kalian tidak akan pernah menghancurkan Lingkaran Merah!' teriaknya memamerkan layar detonator yang terus menghitung mundur.",
            "Aku langsung merampas detonator pemancar sinyal itu. Antarmukanya meminta Kode Penghentian darurat yang terkalibrasi dari modul utama server markas di atas tebing tadi.",
        ],
        puzzle_context: "Kode pembatalan darurat adalah gabungan langsung dari variabel daya Matriks Vault (Chapter 26) ditambah variabel Tipe Dekripsi Waktu (Chapter 25).",
        puzzle_question: "Berapa digit angka Master Code pembatalan yang harus kusisipkan untuk menghentikan sabotase bendungan? (Matriks 56, Waktu Dekripsi 63).",
        puzzle_hint: "Cukup jumlahkan kedua angka tersebut secara langsung.",
        correct_answers: &["119"],
        narrative_success: &[
            "Seratus sembilan belas! Tanganku memencet tombol terminal pemancar dengan putus asa.",
            "Lampu merah yang berkedip ganas di detonator tiba-tiba mati perlahan.",
            "Sinyal pemusnah ke bendungan terputus sepenuhnya. Jutaan nyawa di kota aman malam ini.",
        ],
        narrative_failure: "Kode yang salah membuat detonator memancarkan suara bip melengking peringatan akhir. Tinggal sekian detik menuju kehancuran total!",
        variable_key: Some("final_code"),
        variable_value: Some("119"),
        items_gained: &["Detonator Yang Mati"],
        health_penalty: 12,
    }
}

pub fn chapter_30() -> ChapterData {
    ChapterData {
        number: 30,
        title: "Akhir Cerita: Sang Detektif",
        math_level: MathLevel::HOTS,
        narrative_intro: &[
            "Surya terkapar tak berdaya. Sirene polisi terdengar dari kejauhan — tim backup independen yang kuhubungi di stasiun mulai mengepung lokasi perairan.",
            "Cetak Biru Berdarah, sebuah konspirasi gila untuk mengontrol pasokan sumber daya kota melalui manipulasi bendungan dan infrastruktur kereta, akhirnya terbongkar.",
            "Namun sebelum diborgol, Surya yang masih tersungkur melontarkan argumen logika yang menjengkelkan dari mulut berdarahnya.",
            "'Pikirkan ini, Detektif. Jika kau adalah pelindung hukum AND kau menghancurkan markasku, maka kau sama kriminalnya denganku. Kenyataannya, kau pelindung hukum dan kau menghancurkan milikku...'",
            "Aku menodongkan moncong pistol yang kosong ke arahnya, dan menjawabnya dengan kesimpulan silogisme buatanku sendiri.",
            "'Kau salah, Surya. Premisku adalah: Jika hukum butuh pedang AND konspirasi membahayakan kota, maka Lingkaran Merah telah runtuh malam ini. Hukum sedang ditegakkan dan konspirasimu berbahaya.'",
        ],
        puzzle_context: "Silogisme sang detektif: Jika A (hukum butuh pedang) dan B (konspirasi membahayakan kota) terjadi, maka Kesimpulan C berlaku.",
        puzzle_question: "Berdasarkan premis tegas sang detektif tersebut, apa kesimpulan utamanya yang mematahkan argumen si Komisaris?",
        puzzle_hint: "Masukkan persis kesimpulan dari premis detektif tadi (lima kata yang menyatakan nasib organisasi itu malam ini).",
        correct_answers: &["lingkaran merah telah runtuh malam ini", "lingkaran merahtelahruntuhmalamini", "lingkaran merah telah runtuh", "lingkaran merah runtuh"],
        narrative_success: &[
            "'Lingkaran Merah telah runtuh malam ini,' bisikku dingin, membuang pistol ke tanah.",
            "Surya hanya bisa menatap nanar seiring pasukan polisi menyerbu masuk ke pelataran darat.",
            "Aku berjalan menjauh menyusuri tepi pantai, memandangi fajar yang merekah di ufuk timur.",
            "Profesor Hartono, di mana pun Anda berada, keadilan bagi karya cetak biru Anda telah tuntas ditegakkan.",
            "Petualangan usai.",
        ],
        narrative_failure: "Bukan itu kesimpulanku. Jawablah dengan tajam untuk mematahkan keangkuhan Komisaris.",
        variable_key: Some("final_verdict"),
        variable_value: Some("guilty"),
        items_gained: &["Lencana Kehormatan Sejati"],
        health_penalty: 15,
    }
}
