use crate::chapters::ChapterData;
use crate::state::game_state::MathLevel;

pub fn chapter_01() -> ChapterData {
    ChapterData {
        number: 1,
        title: "Jam yang Berhenti",
        math_level: MathLevel::LOTS,
        narrative_intro: &[
            "Angin malam berhembus menembus celah jendela bangunan tua di Semarang.",
            "Ruang kerja mendiang Profesor Hartono terasa pengap. Polisi menyatakan ini murni bunuh diri. Namun, insting detektifku berkata lain.",
            "Aku menemukan sebuah brankas baja yang tertanam di dinding. Anehnya, brankas ini tidak menggunakan dial angka kombinasi standar, melainkan dial pengukur waktu dalam hitungan 'menit'.",
            "Satu-satunya petunjuk di ruangan ini adalah jam dinding antik yang pecah berantakan. Jarumnya berhenti paksa tepat pada pukul 03:15 pagi.",
        ],
        puzzle_context: "Dial brankas meminta input total menit yang telah berlalu sejak tengah malam (00:00) hingga waktu yang ditunjukkan jam tersebut (03:15).",
        puzzle_question: "Berapa menit total yang harus aku putar pada dial brankas tersebut?",
        puzzle_hint: "Tengah malam ke jam 3 pagi itu 3 jam. 1 jam ada 60 menit.",
        correct_answers: &["195", "195menit", "195 menit"],
        narrative_success: &[
            "Klik! Mekanisme roda gigi di dalam brankas berputar pelan.",
            "Ternyata dugaanku benar. Profesor sengaja merusak jamnya di waktu tersebut sebagai kode brankas.",
            "Pintu baja berat itu terbuka, menyingkap sebuah kompartemen rahasia.",
        ],
        narrative_failure: "Dial terkunci sesaat. Angka menit itu belum tepat membuka brankas.",
        variable_key: Some("alarm_time"),
        variable_value: Some("195"),
        items_gained: &["Catatan Profesor"],
        health_penalty: 5,
    }
}

pub fn chapter_02() -> ChapterData {
    ChapterData {
        number: 2,
        title: "Laci Terkunci",
        math_level: MathLevel::LOTS,
        narrative_intro: &[
            "Di dalam brankas tadi, aku menemukan sebuah kunci laci meja dengan nomor identifikasi '487' terukir di pangkalnya.",
            "Namun kunci itu bengkok dan tidak bisa dimasukkan ke laci meja utama Profesor.",
            "Di samping meja, ada alat pemotong kunci (key cutter) analog tua peninggalan zaman Belanda.",
            "Alat itu membutuhkan input 'kedalaman pemotongan' agar bisa menduplikasi kunci tersebut dari bahan mentah.",
            "Buku panduan alat menyebutkan: 'Untuk kunci model seri-A, kedalaman potong adalah jumlah dari semua digit nomor kunci, dikalikan tiga.'",
        ],
        puzzle_context: "Nomor identifikasi kunci adalah 487. Kedalaman potong = (jumlah semua digit) dikalikan tiga.",
        puzzle_question: "Pada angka berapa aku harus menyetel alat pemotong kunci ini?",
        puzzle_hint: "Tambahkan dulu 4 + 8 + 7, barulah dikalikan 3.",
        correct_answers: &["57"],
        narrative_success: &[
            "Mesin pemotong mendesing, memahat baja mentah menjadi anak kunci yang baru.",
            "Kumasukkan kunci duplikat itu ke laci meja utama. Pas sekali.",
            "Laci terbuka. Di dalamnya, tersimpan sebuah gulungan kertas yang tebal.",
        ],
        narrative_failure: "Kunci duplikatnya tidak presisi. Laci menolak bergeser. Aku harus menghitung ulang.",
        variable_key: Some("lock_code"),
        variable_value: Some("57"),
        items_gained: &["Kunci Duplikat Laci"],
        health_penalty: 5,
    }
}

pub fn chapter_03() -> ChapterData {
    ChapterData {
        number: 3,
        title: "Karpet Persia",
        math_level: MathLevel::LOTS,
        narrative_intro: &[
            "Gulungan di laci itu ternyata cetak biru lantai ruangan ini.",
            "Profesor menggambar ruang bawah tanah persis di bawah tempatku berpijak sekarang.",
            "Untuk membuka pintu jebakan, aku harus menekan satu ubin kayu khusus yang tersembunyi di bawah karpet Persia tebal ini.",
            "Pintu jebakan berada di titik tengah luasan ruangan. Tapi karpet ini terlalu berat untuk digeser seluruhnya tanpa bantuan.",
            "Aku harus mengetahui luas ruangan agar bisa mengukur titik tengahnya dengan akurat tanpa perlu menggulung seluruh karpet.",
        ],
        puzzle_context: "Cetak biru menunjukkan ruangan ini berbentuk persegi panjang sempurna dengan panjang dinding 6 meter dan lebar 4 meter.",
        puzzle_question: "Berapa meter persegi luas lantai ruangan ini agar aku bisa memetakan titik pusat ubinnya?",
        puzzle_hint: "Luas persegi panjang didapat dengan mengalikan panjang dan lebarnya.",
        correct_answers: &["24", "24m", "24 m2", "24m2", "24 meter"],
        narrative_success: &[
            "Dua puluh empat meter persegi. Berbekal ukuran itu, aku melangkah tepat ke titik pusat ruangan.",
            "Kusibak sebagian kecil karpet di area itu, dan benar saja, ada kenop kuningan tertanam di kayu.",
            "Saat kutarik, sebuah pintu jebakan (trapdoor) terbuka perlahan.",
        ],
        narrative_failure: "Luasnya meleset. Aku mencongkel lantai yang salah, membuang-buang tenagaku.",
        variable_key: Some("room_area"),
        variable_value: Some("24"),
        items_gained: &["Denah Lantai Rahasia"],
        health_penalty: 5,
    }
}

pub fn chapter_04() -> ChapterData {
    ChapterData {
        number: 4,
        title: "Pecahan Wasiat",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di bawah pintu jebakan, aku menemukan brankas kecil lain peninggalan Profesor beserta sebuah surat wasiat.",
            "Nahasnya, surat wasiat itu setengahnya hangus terbakar. Seseorang telah mencoba menghancurkan dokumen ini.",
            "Teks yang tersisa: 'Pembagian seluruh saham rekayasa ini: 1/3 untuk istriku, 1/4 untuk putraku. Sisa saham sepenuhnya dialihkan ke entitas Lingkaran Merah.'",
            "Brankas deposit di hadapanku menanyakan nomor rekening bank yayasan Lingkaran Merah. Nomor itu berakhiran dengan rasio pecahan saham mereka.",
        ],
        puzzle_context: "Dari 1 bagian utuh (100%), 1/3 diberikan ke istri dan 1/4 ke anak. Sisanya adalah milik yayasan.",
        puzzle_question: "Berapa sisa pecahan yang menjadi milik yayasan tersebut? (Ketik format: pembilang/penyebut)",
        puzzle_hint: "Samakan penyebutnya jadi 12. 1/3 = 4/12, 1/4 = 3/12. Sisanya dari 1 utuh (12/12).",
        correct_answers: &["5/12", "5 / 12"],
        narrative_success: &[
            "Lima per dua belas. Angka fraksional yang sangat spesifik.",
            "Aku memasukkan nomor tersebut ke sistem keypad mekanik brankas.",
            "Terdengar bunyi logam bergeser. Brankas berhasil diakses.",
        ],
        narrative_failure: "Keypad berbunyi nyaring. Sandi turunannya salah.",
        variable_key: Some("fraction_remainder"),
        variable_value: Some("5/12"),
        items_gained: &["Surat Wasiat Hangus"],
        health_penalty: 5,
    }
}

pub fn chapter_05() -> ChapterData {
    ChapterData {
        number: 5,
        title: "Buku Besar Konspirasi",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di dalam brankas, aku menemukan Buku Besar yang mencatat transaksi ilegal Lingkaran Merah.",
            "Sayangnya, buku ini dikunci dengan sebuah gembok silinder putar yang sangat rumit.",
            "Catatan kecil di sampul buku menyebutkan bahwa kodenya adalah 'akumulasi dari setiap jejak yang kutinggalkan'.",
            "Ini menguji ingatanku sebagai seorang detektif mengenai semua data numerik yang telah kukumpulkan di ruangan ini.",
        ],
        puzzle_context: "Kombinasi gembok adalah penjumlahan *digit terakhir* dari data-data investigasi sebelumnya:\nJam (195), Kunci (57), Luas Ruang (24), dan Pembilang dari rasio saham (5/12).",
        puzzle_question: "Berapa total hasil penjumlahannya?",
        puzzle_hint: "Digit terakhir dari 195 adalah 5. Digit terakhir dari 57 adalah 7. Tambahkan digit-digit terakhir tersebut.",
        correct_answers: &["21"],
        narrative_success: &[
            "Krak! Gembok silinder itu terlepas.",
            "Aku membuka Buku Besar tersebut. Di dalamnya tertulis sebuah titik pertemuan rahasia.",
            "Mereka sering berkumpul di gereja tua di ujung kota.",
        ],
        narrative_failure: "Gemboknya macet. Perhitunganku pasti ada yang meleset.",
        variable_key: Some("safe_code"),
        variable_value: Some("21"),
        items_gained: &["Buku Besar Lingkaran Merah"],
        health_penalty: 5,
    }
}

pub fn chapter_06() -> ChapterData {
    ChapterData {
        number: 6,
        title: "Menyelinap ke Gereja",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Buku Besar itu mengarahkanku pada gereja tua yang telah lama ditinggalkan.",
            "Setibanya di sana, aku bersembunyi di balik semak belukar. Tempat ini dijaga ketat oleh dua preman bersenjata.",
            "Preman pertama (Si Jangkung) berpatroli mengelilingi pekarangan setiap 12 menit sekali sebelum kembali ke pos depan.",
            "Preman kedua (Si Pendek) berpatroli dengan rute lebih panjang, kembali ke pos setiap 18 menit sekali.",
            "Saat ini mereka berdua sedang berada di pos depan. Untuk menyelinap masuk lewat pintu belakang yang macet, aku butuh setidaknya 5 menit tanpa ada yang berpatroli di area luar.",
            "Artinya, aku harus mulai bergerak tepat setelah mereka berdua bertemu di pos secara bersamaan untuk kedua kalinya.",
        ],
        puzzle_context: "Mereka baru saja berpapasan di pos sekarang. Kapan mereka akan berpapasan kembali di pos depan?",
        puzzle_question: "Berapa menit lagi mereka berdua akan bertemu kembali di pos secara bersamaan?",
        puzzle_hint: "Cari waktu di mana kedua siklus patroli tumpang tindih. Gunakan Kelipatan Persekutuan Terkecil (KPK) dari 12 dan 18.",
        correct_answers: &["36", "36 menit", "36menit"],
        narrative_success: &[
            "Tiga puluh enam menit. Aku menyetel alarm senyap di jam tanganku.",
            "Aku menunggu dengan sabar. Tepat di menit ke-36, keduanya bertemu kembali di pos untuk merokok.",
            "Kesempatan emas! Aku bergegas menuju pintu belakang, mendobrak masuk, dan menyelinap ke ruang bawah tanah gereja.",
        ],
        narrative_failure: "Tebakanku meleset. Aku hampir ketahuan oleh salah satu preman yang tiba-tiba melintas. Aku harus mundur dan menghitung lagi.",
        variable_key: Some("bell_sync"),
        variable_value: Some("36"),
        items_gained: &["Peta Patroli"],
        health_penalty: 8,
    }
}

pub fn chapter_07() -> ChapterData {
    ChapterData {
        number: 7,
        title: "Kerek Rantai Besi",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di ruang bawah tanah gereja, aku menemukan mesin derek kuno berbahan bakar uap.",
            "Mesin ini tersambung ke pintu jeruji baja berat yang menghalangi lorong.",
            "Sistem mekaniknya manual: aku harus memutar engkol yang terhubung ke Roda Gigi A, yang akan memutar Roda Gigi B untuk menarik rantai baja.",
            "Jeruji itu cukup berat. Untuk mengangkatnya sepenuhnya setinggi kepalaku, Roda Gigi B harus berputar sebanyak 9 putaran penuh.",
            "Masalahnya, engkolku terhubung di Roda Gigi A. Aku harus tahu berapa kali aku harus memutar engkol secara manual.",
        ],
        puzzle_context: "Roda Gigi A (tempat engkol) memiliki 32 gigi. Roda Gigi B (penarik rantai) memiliki 24 gigi. Roda Gigi B harus berputar 12 kali (Wait, cerita butuh diubah. Roda Gigi B perlu berputar 12 kali agar Roda A diputar 9 kali? Tidak, soal awal: A 24, B 32, A memutar B. Kita ubah sesuai hitungan fisika murni.)\nWait, mari kita ikuti puzzle aslinya: Roda Gigi A (engkol) punya 24 gigi, B punya 32 gigi. A memutar 12 kali, berapakah putaran B? (Hasil: 9).",
        puzzle_question: "Roda Gigi A (24 gigi) memutar Roda Gigi B (32 gigi). Jika aku memutar engkol di Roda Gigi A sebanyak 12 kali penuh, berapa kali Roda Gigi B akan berputar menarik rantai?",
        puzzle_hint: "Semakin besar giginya, semakin lambat putarannya. (24 / 32) * 12 putaran.",
        correct_answers: &["9", "9 kali", "9 putaran"],
        narrative_success: &[
            "Sembilan putaran. Perhitungan mekanika dasar.",
            "Aku memutar engkol Roda A dengan kuat sebanyak 12 kali, dan tepat seperti dugaanku, Roda B menarik rantainya sebanyak 9 kali putaran.",
            "Jeruji baja itu terangkat sempurna hingga mengunci di posisinya.",
        ],
        narrative_failure: "Jeruji belum terangkat sepenuhnya atau malah tersangkut karena aku salah mengestimasi tenaga tarikannya.",
        variable_key: Some("gear_turns"),
        variable_value: Some("9"),
        items_gained: &["Engkol Besi"],
        health_penalty: 8,
    }
}

pub fn chapter_08() -> ChapterData {
    ChapterData {
        number: 8,
        title: "Gas Beracun",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di balik jeruji, aku mendapati sebuah ruang bawah tanah berukuran kecil yang tiba-tiba tertutup otomatis.",
            "Alarm meraung, dan semburan gas amonia mematikan mulai mengisi ruangan. Ini jebakan!",
            "Di dinding terdapat tabung oksigen darurat dan ampul cairan penetral gas.",
            "Buku panduan darurat menyatakan cairan penetral harus dilarutkan ke sistem ventilasi sesuai dengan volume udara ruangan agar tidak meledak.",
        ],
        puzzle_context: "Ruangan jebakan ini berukuran panjang 4 meter, lebar 3 meter, dan tinggi atap 2 meter. Dosis cairan penetral adalah 2 ml untuk setiap 6 meter kubik volume ruangan.",
        puzzle_question: "Berapa ml dosis cairan penetral yang harus segera kutuang ke dalam filter ventilasi ruangan ini?",
        puzzle_hint: "Cari volume ruang jebakan dulu (panjang x lebar x tinggi). Kemudian hitung dosis penawarnya berdasarkan volume tersebut.",
        correct_answers: &["8", "8 ml", "8ml"],
        narrative_success: &[
            "Delapan mililiter! Tanganku gemetar menuangkan cairan kuning itu ke filter ventilasi.",
            "Bunyi desisan keras terdengar. Asap beracun itu bereaksi dan mengendap menjadi uap air.",
            "Pintu otomatis terbuka kembali. Aku selamat, meski dadaku masih sesak.",
        ],
        narrative_failure: "Dosisnya tidak pas. Gas beracun itu membakar tenggorokanku, aku batuk darah.",
        variable_key: Some("poison_dose"),
        variable_value: Some("8"),
        items_gained: &["Ampul Kosong"],
        health_penalty: 8,
    }
}

pub fn chapter_09() -> ChapterData {
    ChapterData {
        number: 9,
        title: "Kabel Listrik Cadangan",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Lorong selanjutnya digenangi air, dan ada kabel listrik putus yang menyentuh genangan, membuatnya mematikan untuk dilewati.",
            "Di sebelahku ada saklar utama, tapi kabelnya telah diputus oleh preman-preman itu.",
            "Satu-satunya cara adalah menyambung kabel cadangan dari panel A ke panel B dengan mengitari lorong secara menempel ke dinding lorong agar tidak menyentuh air.",
            "Lorong ini berkelok-kelok tajam membentuk huruf L tak beraturan.",
            "Aku harus memotong kabel yang cukup panjang dari gulungan di tas, tapi jika terlalu pendek aku akan tersengat, jika terlalu panjang kabel akan menjuntai ke air.",
        ],
        puzzle_context: "Untuk menyusuri tepi dinding kering dari panel A ke panel B, kabel harus melewati 6 sisi tikungan dinding yang panjangnya: 10m, 3m, 7m, 5m, 3m, dan 8m.",
        puzzle_question: "Berapa meter total panjang kabel yang harus kupotong agar pas mengitari seluruh sisi tepi dinding (keliling) tersebut?",
        puzzle_hint: "Jumlahkan saja panjang seluruh sisi dinding yang harus dilewati kabel.",
        correct_answers: &["36", "36 meter", "36m"],
        narrative_success: &[
            "Tiga puluh enam meter kabel ku potong dan kupasang perlahan menempel ke dinding, memutar air genangan.",
            "Klik. Saklar kuaktifkan. Arus berpindah ke kabel cadangan dan genangan air kembali aman untuk diseberangi.",
            "Aku melangkah melewati genangan dengan hati-hati.",
        ],
        narrative_failure: "Kabelnya tidak pas! Ujungnya menyentuh air, menciptakan percikan api besar yang membakarku.",
        variable_key: Some("maze_length"),
        variable_value: Some("36"),
        items_gained: &["Sisa Kabel"],
        health_penalty: 8,
    }
}

pub fn chapter_10() -> ChapterData {
    ChapterData {
        number: 10,
        title: "Pintu Markas",
        math_level: MathLevel::MOTS,
        narrative_intro: &[
            "Di ujung lorong genangan air, aku berhadapan dengan sebuah brankas kubah raksasa.",
            "Ini adalah akses masuk ke operasi bawah tanah Lingkaran Merah.",
            "Panel pintu baja ini terhubung ke empat modul keamanan yang sebelumnya kulewati: Mekanisme Lonceng, Derek Roda Gigi, Katup Gas, dan Panel Listrik Air.",
            "Terminal meminta 'Master Over-ride Code' yang merupakan checksum diagnostik dari keempat operasi mekanik yang kulakukan tadi.",
        ],
        puzzle_context: "Kode checksum adalah hasil penjumlahan dari metrik operasional tadi: Menit Patroli Lonceng (36), Putaran Gigi (9), Dosis Gas (8), dan Panjang Kabel (36).",
        puzzle_question: "Berapa angka sandi untuk menembus ke dalam markas rahasia ini?",
        puzzle_hint: "Jumlahkan 36 + 9 + 8 + 36.",
        correct_answers: &["89"],
        narrative_success: &[
            "Delapan puluh sembilan. Aku memasukkan digit terakhir ke terminal layar sentuh.",
            "Hydrolik berdesis, dan kubah baja seberat tiga ton bergeser lambat.",
            "Markas Lingkaran Merah terbentang di hadapanku. Saatnya membongkar konspirasi ini.",
        ],
        narrative_failure: "Sandi ditolak. Alarm intrusi tingkat dua mulai menyala.",
        variable_key: Some("caesar_code"),
        variable_value: Some("89"),
        items_gained: &["Akses Markas"],
        health_penalty: 8,
    }
}
