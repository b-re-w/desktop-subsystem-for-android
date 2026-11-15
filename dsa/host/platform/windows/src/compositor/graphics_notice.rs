//! What the user is told about the Vulkan the graphics renderer runs on.
//!
//! Two cases need the user: no Vulkan at all (no GPU has a Vulkan driver and the Compatibility
//! Pack is not installed), where Android cannot draw and is kept stopped until they install
//! the pack; and Dozen on the software Direct3D 12 device, where everything works but the CPU
//! draws, so games are slow. The first is said each time the window manager finds it, since
//! nothing works until it is fixed. The second is said once: a file in the user's DSA folder
//! remembers it, so the user is not told again at every sign-in, and is told again only after
//! the renderer ran on a GPU in between.
//!
//! The notices are DSA's own toasts under the recovery prefix, so a click on one asks nothing
//! of Android.

use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use dsa_host_core::compositor::frame_stall::NOTICE_PREFIX;
use dsa_host_core::graphics_state::GraphicsState;
use dsa_host_core::notifications::Notification;
use dsa_host_core::ui_language::UiLanguage;

use crate::vulkan_driver::{PACK_WEB_LINK, PACK_WINGET_ID};

/// The keys of the two notices.
pub fn unavailable_key() -> String {
    format!("{NOTICE_PREFIX}vulkan:unavailable")
}

pub fn cpu_key() -> String {
    format!("{NOTICE_PREFIX}vulkan:cpu")
}

/// The title and text of the notice that no GPU has a Vulkan driver DSA can use, in
/// `language`. `{link}` is the Compatibility Pack's Store link and `{winget}` its winget ID.
pub fn unavailable_words(language: UiLanguage) -> (&'static str, &'static str) {
    match language {
        UiLanguage::English => (
            "Android needs Vulkan to draw",
            "No GPU of this PC has a Vulkan driver DSA can use, so Android is not started. \
             Install Microsoft's \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" from the \
             Microsoft Store ({link}) or with \"winget install {winget}\", then sign out and in \
             again. \"dsa gpus\" shows what DSA finds.",
        ),
        UiLanguage::Korean => (
            "Android를 그리려면 Vulkan이 필요합니다",
            "이 PC의 GPU에는 DSA가 쓸 수 있는 Vulkan 드라이버가 없어 Android를 시작하지 \
             않습니다. Microsoft Store({link})나 \"winget install {winget}\"로 Microsoft의 \
             \"OpenCL, OpenGL, and Vulkan Compatibility Pack\"을 설치한 뒤 로그아웃했다가 \
             다시 로그인하십시오. \"dsa gpus\"로 DSA가 찾은 것을 볼 수 있습니다.",
        ),
        UiLanguage::Japanese => (
            "Android の描画には Vulkan が必要です",
            "この PC の GPU には DSA が使用できる Vulkan ドライバーがないため、Android は\
             起動されません。Microsoft Store ({link}) または \"winget install {winget}\" で \
             Microsoft の \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" をインストール\
             してから、サインアウトしてもう一度サインインしてください。\"dsa gpus\" で DSA \
             が検出したものを確認できます。",
        ),
        UiLanguage::ChineseSimplified => (
            "Android 需要 Vulkan 才能绘制",
            "此电脑的 GPU 没有 DSA 可用的 Vulkan 驱动程序，因此不会启动 Android。请从 \
             Microsoft Store ({link}) 或使用 \"winget install {winget}\" 安装 Microsoft 的 \
             \"OpenCL, OpenGL, and Vulkan Compatibility Pack\"，然后注销并重新登录。\
             \"dsa gpus\" 会显示 DSA 找到的内容。",
        ),
        UiLanguage::ChineseTraditional => (
            "Android 需要 Vulkan 才能繪製",
            "這部電腦的 GPU 沒有 DSA 可用的 Vulkan 驅動程式，因此不會啟動 Android。請從 \
             Microsoft Store ({link}) 或使用 \"winget install {winget}\" 安裝 Microsoft 的 \
             \"OpenCL, OpenGL, and Vulkan Compatibility Pack\"，然後登出再重新登入。\
             \"dsa gpus\" 會顯示 DSA 找到的內容。",
        ),
        UiLanguage::Spanish => (
            "Android necesita Vulkan para dibujar",
            "Ninguna GPU de este equipo tiene un controlador Vulkan que DSA pueda usar, así que \
             Android no se inicia. Instale \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" de \
             Microsoft desde Microsoft Store ({link}) o con \"winget install {winget}\" y, a \
             continuación, cierre la sesión y vuelva a iniciarla. \"dsa gpus\" muestra lo que \
             encuentra DSA.",
        ),
        UiLanguage::French => (
            "Android a besoin de Vulkan pour l'affichage",
            "Aucun GPU de ce PC ne dispose d'un pilote Vulkan utilisable par DSA, Android n'est \
             donc pas démarré. Installez le \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" de \
             Microsoft depuis le Microsoft Store ({link}) ou avec \"winget install {winget}\", \
             puis déconnectez-vous et reconnectez-vous. \"dsa gpus\" affiche ce que DSA trouve.",
        ),
        UiLanguage::German => (
            "Android benötigt Vulkan zum Zeichnen",
            "Keine GPU dieses PCs hat einen Vulkan-Treiber, den DSA verwenden kann, daher wird \
             Android nicht gestartet. Installieren Sie das \"OpenCL, OpenGL, and Vulkan \
             Compatibility Pack\" von Microsoft aus dem Microsoft Store ({link}) oder mit \
             \"winget install {winget}\", und melden Sie sich dann ab und wieder an. \
             \"dsa gpus\" zeigt, was DSA findet.",
        ),
        UiLanguage::PortugueseBrazil => (
            "O Android precisa do Vulkan para desenhar",
            "Nenhuma GPU deste PC tem um driver Vulkan que o DSA possa usar, então o Android não \
             é iniciado. Instale o \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" da \
             Microsoft pela Microsoft Store ({link}) ou com \"winget install {winget}\" e depois \
             saia e entre novamente. \"dsa gpus\" mostra o que o DSA encontra.",
        ),
        UiLanguage::Russian => (
            "Для отрисовки Android нужен Vulkan",
            "Ни у одного GPU этого компьютера нет драйвера Vulkan, который может использовать \
             DSA, поэтому Android не запускается. Установите \"OpenCL, OpenGL, and Vulkan \
             Compatibility Pack\" от Microsoft из Microsoft Store ({link}) или командой \
             \"winget install {winget}\", затем выйдите из системы и войдите снова. Команда \
             \"dsa gpus\" показывает, что находит DSA.",
        ),
        UiLanguage::Italian => (
            "Android richiede Vulkan per disegnare",
            "Nessuna GPU di questo PC ha un driver Vulkan utilizzabile da DSA, quindi Android \
             non viene avviato. Installa \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" di \
             Microsoft da Microsoft Store ({link}) o con \"winget install {winget}\", quindi \
             esci e accedi di nuovo. \"dsa gpus\" mostra cosa trova DSA.",
        ),
        UiLanguage::Vietnamese => (
            "Android cần Vulkan để hiển thị",
            "Không GPU nào của PC này có trình điều khiển Vulkan mà DSA dùng được, nên Android \
             không được khởi động. Hãy cài đặt \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" \
             của Microsoft từ Microsoft Store ({link}) hoặc bằng \"winget install {winget}\", \
             rồi đăng xuất và đăng nhập lại. \"dsa gpus\" cho biết DSA tìm thấy gì.",
        ),
        UiLanguage::Indonesian => (
            "Android memerlukan Vulkan untuk menggambar",
            "Tidak ada GPU di PC ini yang memiliki driver Vulkan yang dapat digunakan DSA, jadi \
             Android tidak dimulai. Instal \"OpenCL, OpenGL, and Vulkan Compatibility Pack\" \
             dari Microsoft melalui Microsoft Store ({link}) atau dengan \
             \"winget install {winget}\", lalu keluar dan masuk kembali. \"dsa gpus\" \
             menampilkan apa yang ditemukan DSA.",
        ),
        UiLanguage::Turkish => (
            "Android'in çizim yapması için Vulkan gerekiyor",
            "Bu bilgisayarın hiçbir GPU'sunda DSA'nın kullanabileceği bir Vulkan sürücüsü yok, \
             bu nedenle Android başlatılmıyor. Microsoft'un \"OpenCL, OpenGL, and Vulkan \
             Compatibility Pack\" paketini Microsoft Store'dan ({link}) veya \
             \"winget install {winget}\" ile yükleyin, ardından oturumu kapatıp yeniden açın. \
             \"dsa gpus\", DSA'nın bulduklarını gösterir.",
        ),
    }
}

/// The title and text of the notice that Android is drawn through Dozen on Windows' software
/// renderer, in `language`.
pub fn cpu_words(language: UiLanguage) -> (&'static str, &'static str) {
    match language {
        UiLanguage::English => (
            "Android draws on the CPU",
            "This PC's GPU has no Direct3D 12 driver, so DSA renders Android through Dozen on \
             Windows' software renderer. Apps work, but games and animations will be slow.",
        ),
        UiLanguage::Korean => (
            "Android를 CPU로 그립니다",
            "이 PC의 GPU에는 Direct3D 12 드라이버가 없어 DSA가 Windows의 소프트웨어 렌더러 위의 \
             Dozen으로 Android를 그립니다. 앱은 동작하지만 게임과 애니메이션은 느립니다.",
        ),
        UiLanguage::Japanese => (
            "Android は CPU で描画しています",
            "この PC の GPU には Direct3D 12 ドライバーがないため、DSA は Windows のソフトウェア \
             レンダラー上の Dozen で Android を描画します。アプリは動作しますが、ゲームや\
             アニメーションは遅くなります。",
        ),
        UiLanguage::ChineseSimplified => (
            "Android 正在使用 CPU 绘制",
            "此电脑的 GPU 没有 Direct3D 12 驱动程序，因此 DSA 通过 Windows 软件渲染器上的 Dozen \
             渲染 Android。应用可以运行，但游戏和动画会很慢。",
        ),
        UiLanguage::ChineseTraditional => (
            "Android 正在使用 CPU 繪製",
            "這部電腦的 GPU 沒有 Direct3D 12 驅動程式，因此 DSA 透過 Windows 軟體轉譯器上的 \
             Dozen 轉譯 Android。應用程式可以運作，但遊戲和動畫會很慢。",
        ),
        UiLanguage::Spanish => (
            "Android dibuja con la CPU",
            "La GPU de este equipo no tiene un controlador de Direct3D 12, así que DSA representa \
             Android mediante Dozen sobre el representador de software de Windows. Las \
             aplicaciones funcionan, pero los juegos y las animaciones serán lentos.",
        ),
        UiLanguage::French => (
            "Android est affiché par le processeur",
            "Le GPU de ce PC n'a pas de pilote Direct3D 12 : DSA affiche donc Android via Dozen \
             sur le moteur de rendu logiciel de Windows. Les applications fonctionnent, mais les \
             jeux et les animations seront lents.",
        ),
        UiLanguage::German => (
            "Android wird von der CPU gezeichnet",
            "Die GPU dieses PCs hat keinen Direct3D 12-Treiber, daher rendert DSA Android über \
             Dozen mit dem Softwarerenderer von Windows. Apps funktionieren, aber Spiele und \
             Animationen sind langsam.",
        ),
        UiLanguage::PortugueseBrazil => (
            "O Android desenha com a CPU",
            "A GPU deste PC não tem um driver Direct3D 12, então o DSA renderiza o Android pelo \
             Dozen no renderizador de software do Windows. Os apps funcionam, mas jogos e \
             animações ficarão lentos.",
        ),
        UiLanguage::Russian => (
            "Android отрисовывается процессором",
            "У GPU этого компьютера нет драйвера Direct3D 12, поэтому DSA отрисовывает Android \
             через Dozen на программном отрисовщике Windows. Приложения работают, но игры и \
             анимации будут медленными.",
        ),
        UiLanguage::Italian => (
            "Android disegna con la CPU",
            "La GPU di questo PC non ha un driver Direct3D 12, quindi DSA esegue il rendering di \
             Android tramite Dozen sul renderer software di Windows. Le app funzionano, ma giochi \
             e animazioni saranno lenti.",
        ),
        UiLanguage::Vietnamese => (
            "Android đang hiển thị bằng CPU",
            "GPU của PC này không có trình điều khiển Direct3D 12, nên DSA kết xuất Android qua \
             Dozen trên trình kết xuất phần mềm của Windows. Ứng dụng vẫn hoạt động, nhưng trò \
             chơi và hoạt ảnh sẽ chậm.",
        ),
        UiLanguage::Indonesian => (
            "Android menggambar dengan CPU",
            "GPU PC ini tidak memiliki driver Direct3D 12, jadi DSA merender Android melalui \
             Dozen pada perender perangkat lunak Windows. Aplikasi tetap berfungsi, tetapi game \
             dan animasi akan lambat.",
        ),
        UiLanguage::Turkish => (
            "Android, CPU ile çiziliyor",
            "Bu bilgisayarın GPU'sunda Direct3D 12 sürücüsü yok, bu nedenle DSA, Android'i \
             Windows'un yazılım işleyicisi üzerinde Dozen ile işliyor. Uygulamalar çalışır ancak \
             oyunlar ve animasyonlar yavaş olur.",
        ),
    }
}

/// The title and text of the notice for `state`, in `language`; `None` for a state the user
/// need not hear of.
pub fn notice_text(state: &GraphicsState, language: UiLanguage) -> Option<(String, String)> {
    match state {
        GraphicsState::Unavailable(_) => {
            let (title, text) = unavailable_words(language);
            let text = text
                .replace("{link}", PACK_WEB_LINK)
                .replace("{winget}", PACK_WINGET_ID);
            Some((title.into(), text))
        }
        GraphicsState::LayeredOnCpu(_) => {
            let (title, text) = cpu_words(language);
            Some((title.into(), text.into()))
        }
        _ => None,
    }
}

/// The notice for `state` as one of DSA's own notifications, in `language`.
pub fn notification(state: &GraphicsState, language: UiLanguage) -> Option<Notification> {
    let (title, text) = notice_text(state, language)?;
    let key = match state {
        GraphicsState::Unavailable(_) => unavailable_key(),
        _ => cpu_key(),
    };
    Some(Notification {
        key,
        package_name: String::new(),
        app_label: dsa_host_core::update::alerts::SHOWN_AS.into(),
        title,
        text,
        icon_png: Vec::new(),
        importance: 4,
        high_priority: false,
        clearable: true,
        buttons: Vec::new(),
        when_millis: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|since| since.as_millis() as i64)
            .unwrap_or(0),
    })
}

/// Decides which renderer starts the user hears of.
#[derive(Debug, Default)]
pub struct GraphicsNotices {
    /// The state this process last told the user of, as its wire name.
    told: Option<&'static str>,
    /// The file that remembers the CPU notice was shown; `None` keeps it in memory only.
    marker: Option<PathBuf>,
}

impl GraphicsNotices {
    /// Remembers the CPU notice in `marker`.
    pub fn new(marker: Option<PathBuf>) -> GraphicsNotices {
        GraphicsNotices { told: None, marker }
    }

    /// The marker beside the window manager's settings: `<DSA folder>\graphics-cpu-notice`.
    pub fn marker_in(dir: &Path) -> PathBuf {
        dir.join("graphics-cpu-notice")
    }

    /// Whether the renderer's new `state` is one to tell the user of now, and records it.
    pub fn should_tell(&mut self, state: &GraphicsState) -> bool {
        let name = state.as_str();
        let again = self.told == Some(name);
        match state {
            GraphicsState::Unavailable(_) => {
                self.told = Some(name);
                !again
            }
            GraphicsState::LayeredOnCpu(_) => {
                self.told = Some(name);
                let shown_before = self.marker.as_ref().is_some_and(|marker| marker.is_file());
                if again || shown_before {
                    return false;
                }
                if let Some(marker) = &self.marker {
                    if let Some(dir) = marker.parent() {
                        let _ = std::fs::create_dir_all(dir);
                    }
                    let _ = std::fs::write(marker, b"shown\n");
                }
                true
            }
            GraphicsState::Native(_) | GraphicsState::Layered(_) => {
                // On a GPU again: the CPU notice may be said again if it comes back.
                self.told = Some(name);
                if let Some(marker) = &self.marker {
                    let _ = std::fs::remove_file(marker);
                }
                false
            }
            GraphicsState::Unknown => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui_language::assert_translation;
    use dsa_host_core::compositor::frame_stall::is_recovery_key;

    fn scratch(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("dsa-graphics-notice-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        dir
    }

    #[test]
    fn pr3_no_vulkan_is_said_with_the_store_link_and_the_winget_id() {
        for language in UiLanguage::ALL {
            let state = GraphicsState::Unavailable("why".into());
            let notice = notification(&state, language).unwrap();
            assert!(notice.text.contains(PACK_WEB_LINK), "{}", notice.text);
            assert!(notice.text.contains(PACK_WINGET_ID), "{}", notice.text);
            assert!(notice.text.contains("dsa gpus"), "{}", notice.text);
            assert!(
                is_recovery_key(&notice.key),
                "a click asks nothing of Android"
            );
        }
        let native = GraphicsState::Native("x".into());
        assert!(notification(&native, UiLanguage::English).is_none());
        let layered = GraphicsState::Layered("x".into());
        assert!(notification(&layered, UiLanguage::Korean).is_none());
        let cpu = GraphicsState::LayeredOnCpu("x".into());
        let cpu = notification(&cpu, UiLanguage::English).unwrap();
        assert!(cpu.text.contains("slow"), "{}", cpu.text);
        assert_ne!(cpu.key, unavailable_key());
    }

    #[test]
    fn fr17_the_graphics_notices_speak_every_language() {
        let (english_title, english_text) = unavailable_words(UiLanguage::English);
        let (cpu_title, cpu_text) = cpu_words(UiLanguage::English);
        for language in UiLanguage::ALL {
            let (title, text) = unavailable_words(language);
            assert_translation(language, "the Vulkan title", english_title, title);
            assert_translation(language, "the Vulkan text", english_text, text);
            let (title, text) = cpu_words(language);
            assert_translation(language, "the CPU title", cpu_title, title);
            assert_translation(language, "the CPU text", cpu_text, text);
            for (words, what) in [
                (unavailable_words(language), "Vulkan"),
                (cpu_words(language), "CPU"),
            ] {
                assert!(words.0.contains("Android"), "{what} title in {language:?}");
                assert!(words.1.contains("DSA"), "{what} text in {language:?}");
            }
            if language != UiLanguage::English {
                assert_ne!(title, cpu_title, "{language:?} has no translation");
            }
            let cpu = notification(&GraphicsState::LayeredOnCpu("x".into()), language).unwrap();
            assert!(cpu.text.contains("Dozen"), "{}", cpu.text);
        }
    }

    #[test]
    fn pr3_the_cpu_notice_is_said_once_and_again_only_after_a_gpu() {
        let dir = scratch("once");
        let marker = GraphicsNotices::marker_in(&dir);
        let cpu = GraphicsState::LayeredOnCpu("warp".into());
        let mut first = GraphicsNotices::new(Some(marker.clone()));
        assert!(first.should_tell(&cpu));
        assert!(
            !first.should_tell(&cpu),
            "a restarted renderer says nothing new"
        );
        // The next sign-in remembers it.
        let mut next = GraphicsNotices::new(Some(marker.clone()));
        assert!(!next.should_tell(&cpu));
        // A GPU in between, and the CPU again: said again.
        assert!(!next.should_tell(&GraphicsState::Native("arc".into())));
        assert!(next.should_tell(&cpu));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn pr3_no_vulkan_is_said_at_each_start_of_the_window_manager() {
        let mut notices = GraphicsNotices::new(None);
        let none = GraphicsState::Unavailable("why".into());
        assert!(notices.should_tell(&none));
        assert!(
            !notices.should_tell(&none),
            "not again for a restarted renderer"
        );
        assert!(GraphicsNotices::new(None).should_tell(&none));
        assert!(!notices.should_tell(&GraphicsState::Unknown));
    }
}
