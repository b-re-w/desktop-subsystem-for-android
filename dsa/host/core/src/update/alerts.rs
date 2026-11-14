//! Telling the user about updates on the desktop: an update that is downloaded and waits to be
//! applied, the question whether Android restarts into a downloaded image now (with the
//! [Restart now] and [Later] buttons of [`buttons_of`]), one that was applied, and one that
//! failed.
//!
//! The service raises each of these once ([`UpdateAlerts::raise`]), and every user's desktop
//! shows it as one of the host's own notifications, beside the notifications of the user's
//! Android: the watch of a user's notifications carries the alerts that user has not been told
//! yet ([`UpdateAlerts::untold`]), so a user whose desktop was not connected when an update
//! happened is told when it connects. Each alert is told to each user once, and an alert is
//! raised once for each version, so the same news never pops up twice.
//!
//! An alert's notification key ([`UpdateAlert::key`]) says what it is about, which the hosts use
//! three ways: to show it in the user's language ([`UpdateAlert::text`]), to let a newer alert
//! about the same update replace the one shown ([`thread_of`]), and to open the settings app's
//! update page when it is clicked rather than passing the click to Android
//! ([`is_update_key`]).

use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use crate::notifications::{Notification, MAX_TEXT_CHARS};
use crate::ui_language::UiLanguage;

/// What the keys of the update alerts start with. It is under the host's own prefix, which
/// no guest notification can have.
pub const KEY_PREFIX: &str = "dsa:update:";

/// The name an alert is shown under, as its app.
pub const SHOWN_AS: &str = "DSA";

/// How many alerts the service keeps. Only the latest of each [`Subject`] is told, so a few
/// are plenty; the rest are kept so that an alert raised again is still known.
const KEPT: usize = 32;

/// What an update is of.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Subject {
    /// The Android image.
    Android,
    /// DSA itself.
    Program,
}

impl Subject {
    fn word(self) -> &'static str {
        match self {
            Subject::Android => "android",
            Subject::Program => "program",
        }
    }

    fn of_word(word: &str) -> Option<Subject> {
        [Subject::Android, Subject::Program]
            .into_iter()
            .find(|subject| subject.word() == word)
    }
}

/// What happened to an update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AlertKind {
    /// It is downloaded and is applied later: an Android image the next time Android starts,
    /// DSA once no Android is running.
    Ready,
    /// An Android image is downloaded while app windows are open, and the user is asked
    /// whether Android restarts into it now ([`RESTART_NOW`]) or later ([`LATER`]), which is
    /// once every window is closed and Android dozes.
    Restart,
    /// It was applied and is in use.
    Applied,
    /// It was not applied; the previous version is still in use.
    Failed,
}

impl AlertKind {
    fn word(self) -> &'static str {
        match self {
            AlertKind::Ready => "ready",
            AlertKind::Restart => "restart",
            AlertKind::Applied => "applied",
            AlertKind::Failed => "failed",
        }
    }

    fn of_word(word: &str) -> Option<AlertKind> {
        [
            AlertKind::Ready,
            AlertKind::Restart,
            AlertKind::Applied,
            AlertKind::Failed,
        ]
        .into_iter()
        .find(|kind| kind.word() == word)
    }
}

/// The action of the button that restarts Android into the downloaded image now. A host that
/// gets it asks the service to restart ([`crate::host_api::HostApiClient::restart_for_update`]).
pub const RESTART_NOW: &str = "restart-now";

/// The action of the button that leaves the downloaded image for later: the service applies it
/// once every app window is closed and Android dozes. A host only closes the notification.
pub const LATER: &str = "later";

/// A button on an alert's notification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlertButton {
    /// What it does: [`RESTART_NOW`] or [`LATER`].
    pub action: &'static str,
    /// Its label, in the user's language.
    pub label: String,
}

/// The buttons the notification with `key` shows: [Restart now] and [Later] on the question
/// whether Android restarts into a downloaded image, none on any other notification.
pub fn buttons_of(key: &str, lang: UiLanguage) -> Vec<AlertButton> {
    match UpdateAlert::parse_key(key) {
        Some((Subject::Android, AlertKind::Restart, _)) => {
            let words = alert_words(lang);
            vec![
                AlertButton {
                    action: RESTART_NOW,
                    label: words.restart_now.into(),
                },
                AlertButton {
                    action: LATER,
                    label: words.later.into(),
                },
            ]
        }
        _ => Vec::new(),
    }
}

/// One piece of news about an update.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateAlert {
    pub subject: Subject,
    pub kind: AlertKind,
    /// The version the news is about: the one downloaded, applied, or that failed.
    pub version: String,
    /// Why it failed, in words a person can act on. Empty for the other kinds.
    pub detail: String,
}

impl UpdateAlert {
    pub fn new(subject: Subject, kind: AlertKind, version: impl Into<String>) -> UpdateAlert {
        UpdateAlert {
            subject,
            kind,
            version: version.into(),
            detail: String::new(),
        }
    }

    /// An alert that the update to `version` failed, because of `detail`.
    pub fn failed(subject: Subject, version: impl Into<String>, detail: impl Into<String>) -> Self {
        UpdateAlert {
            detail: detail.into(),
            ..UpdateAlert::new(subject, AlertKind::Failed, version)
        }
    }

    /// The key of its notification, which names it: `dsa:update:android:ready:16.0.3`.
    pub fn key(&self) -> String {
        format!(
            "{KEY_PREFIX}{}:{}:{}",
            self.subject.word(),
            self.kind.word(),
            self.version
        )
    }

    /// What the key of an alert's notification names, or `None` for any other key.
    pub fn parse_key(key: &str) -> Option<(Subject, AlertKind, &str)> {
        let rest = key.strip_prefix(KEY_PREFIX)?;
        let mut parts = rest.splitn(3, ':');
        let subject = Subject::of_word(parts.next()?)?;
        let kind = AlertKind::of_word(parts.next()?)?;
        let version = parts.next().filter(|version| !version.is_empty())?;
        Some((subject, kind, version))
    }

    /// The alert an update notification stands for. Its text is the alert's detail.
    pub fn of_notification(notification: &Notification) -> Option<UpdateAlert> {
        let (subject, kind, version) = UpdateAlert::parse_key(&notification.key)?;
        Some(UpdateAlert {
            subject,
            kind,
            version: version.to_string(),
            detail: notification.text.clone(),
        })
    }

    /// The notification that shows it. Its title is in English and its text is the detail; a
    /// host shows it through [`in_user_language`](crate::native_abi::in_user_language), which
    /// writes both in the user's language.
    pub fn notification(&self) -> Notification {
        let (title, _) = self.text(UiLanguage::English);
        Notification {
            key: self.key(),
            package_name: String::new(),
            app_label: SHOWN_AS.into(),
            title,
            text: self.detail.chars().take(MAX_TEXT_CHARS).collect(),
            icon_png: Vec::new(),
            // It pops up: the user is meant to see it, but it is not urgent.
            importance: 3,
            high_priority: false,
            clearable: true,
            when_millis: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|since| since.as_millis() as i64)
                .unwrap_or(0),
            buttons: Vec::new(),
        }
    }

    /// The title and text, in `lang`.
    pub fn text(&self, lang: UiLanguage) -> (String, String) {
        let words = alert_words(lang);
        let (title, text) = match (self.subject, self.kind) {
            (Subject::Android, AlertKind::Ready) => words.android_ready,
            (Subject::Android, AlertKind::Restart) => words.android_restart,
            (Subject::Android, AlertKind::Applied) => words.android_applied,
            (Subject::Android, AlertKind::Failed) => words.android_failed,
            (Subject::Program, AlertKind::Ready) => words.program_ready,
            (Subject::Program, AlertKind::Restart) => words.program_restart,
            (Subject::Program, AlertKind::Applied) => words.program_applied,
            (Subject::Program, AlertKind::Failed) => words.program_failed,
        };
        let sentence = text.replace("{v}", &self.version);
        let text = if self.kind == AlertKind::Failed {
            with_detail(sentence, self.detail.trim(), lang)
        } else {
            sentence
        };
        (title.to_string(), text)
    }
}

/// `sentence`, then why, when there is a why. The why comes from the service in English.
fn with_detail(sentence: String, detail: &str, lang: UiLanguage) -> String {
    if detail.is_empty() {
        sentence
    } else {
        format!("{sentence}{}{detail}", alert_words(lang).why)
    }
}

/// The words of the update alerts in one language. Each alert is its title and its text, in
/// which `{v}` stands for the version.
struct AlertWords {
    android_ready: (&'static str, &'static str),
    android_restart: (&'static str, &'static str),
    android_applied: (&'static str, &'static str),
    android_failed: (&'static str, &'static str),
    program_ready: (&'static str, &'static str),
    program_restart: (&'static str, &'static str),
    program_applied: (&'static str, &'static str),
    program_failed: (&'static str, &'static str),
    /// What comes between a failure's sentence and why it failed.
    why: &'static str,
    restart_now: &'static str,
    later: &'static str,
}

fn alert_words(lang: UiLanguage) -> AlertWords {
    match lang {
        UiLanguage::English => AlertWords {
            android_ready: (
                "Android update ready",
                "Android {v} is downloaded. It is used the next time Android starts.",
            ),
            android_restart: (
                "Restart Android to update",
                "Android {v} is downloaded. Restart Android now to use it, or it is used once \
                 every Android app window is closed.",
            ),
            android_applied: ("Android updated", "Android {v} started and is now in use."),
            android_failed: ("Android update failed", "Android {v} was not applied."),
            program_ready: (
                "DSA update ready",
                "DSA {v} is downloaded. It installs when Android is not running; DSA Settings \
                 can install it now.",
            ),
            program_restart: ("DSA update ready", "DSA {v} is downloaded."),
            program_applied: ("DSA updated", "DSA {v} is installed."),
            program_failed: (
                "DSA update failed",
                "DSA {v} was not installed; the previous version is still in use.",
            ),
            why: " Why: ",
            restart_now: "Restart now",
            later: "Later",
        },
        UiLanguage::Korean => AlertWords {
            android_ready: (
                "Android 업데이트가 준비되었습니다",
                "Android {v}을(를) 내려받았습니다. 다음에 Android를 시작할 때 적용됩니다.",
            ),
            android_restart: (
                "Android를 다시 시작해 업데이트하세요",
                "Android {v}을(를) 내려받았습니다. 지금 다시 시작해 적용하거나, Android 앱 \
                 창을 모두 닫으면 적용됩니다.",
            ),
            android_applied: (
                "Android를 업데이트했습니다",
                "Android {v}이(가) 시작되었고 이제 이 버전을 씁니다.",
            ),
            android_failed: (
                "Android 업데이트에 실패했습니다",
                "Android {v}을(를) 적용하지 못했습니다.",
            ),
            program_ready: (
                "DSA 업데이트가 준비되었습니다",
                "DSA {v}을(를) 내려받았습니다. Android가 실행 중이 아닐 때 설치되며, DSA \
                 설정에서 바로 설치할 수도 있습니다.",
            ),
            program_restart: (
                "DSA 업데이트가 준비되었습니다",
                "DSA {v}을(를) 내려받았습니다.",
            ),
            program_applied: ("DSA를 업데이트했습니다", "DSA {v}을(를) 설치했습니다."),
            program_failed: (
                "DSA 업데이트에 실패했습니다",
                "DSA {v}을(를) 설치하지 못했습니다. 이전 버전을 그대로 씁니다.",
            ),
            why: " 이유: ",
            restart_now: "지금 다시 시작",
            later: "나중에",
        },
        UiLanguage::Japanese => AlertWords {
            android_ready: (
                "Android の更新の準備ができました",
                "Android {v} をダウンロードしました。次回 Android の起動時に使用されます。",
            ),
            android_restart: (
                "Android を再起動して更新してください",
                "Android {v} をダウンロードしました。今すぐ Android を再起動して使用するか、\
                 Android アプリのウィンドウをすべて閉じると使用されます。",
            ),
            android_applied: (
                "Android を更新しました",
                "Android {v} が起動し、現在使用されています。",
            ),
            android_failed: (
                "Android の更新に失敗しました",
                "Android {v} を適用できませんでした。",
            ),
            program_ready: (
                "DSA の更新の準備ができました",
                "DSA {v} をダウンロードしました。Android が実行されていないときにインストール\
                 されます。DSA の設定から今すぐインストールすることもできます。",
            ),
            program_restart: (
                "DSA の更新の準備ができました",
                "DSA {v} をダウンロードしました。",
            ),
            program_applied: ("DSA を更新しました", "DSA {v} をインストールしました。"),
            program_failed: (
                "DSA の更新に失敗しました",
                "DSA {v} をインストールできませんでした。以前のバージョンを引き続き使用します。",
            ),
            why: "理由: ",
            restart_now: "今すぐ再起動",
            later: "後で",
        },
        UiLanguage::ChineseSimplified => AlertWords {
            android_ready: (
                "Android 更新已就绪",
                "已下载 Android {v}。下次启动 Android 时将使用此版本。",
            ),
            android_restart: (
                "重启 Android 以完成更新",
                "已下载 Android {v}。立即重启 Android 即可使用此版本，或在关闭所有 Android \
                 应用窗口后使用。",
            ),
            android_applied: ("Android 已更新", "Android {v} 已启动，现正在使用。"),
            android_failed: ("Android 更新失败", "未能应用 Android {v}。"),
            program_ready: (
                "DSA 更新已就绪",
                "已下载 DSA {v}。它会在 Android 未运行时安装；也可以在 DSA 设置中立即安装。",
            ),
            program_restart: ("DSA 更新已就绪", "已下载 DSA {v}。"),
            program_applied: ("DSA 已更新", "已安装 DSA {v}。"),
            program_failed: ("DSA 更新失败", "未能安装 DSA {v}，仍在使用之前的版本。"),
            why: "原因：",
            restart_now: "立即重启",
            later: "稍后",
        },
        UiLanguage::ChineseTraditional => AlertWords {
            android_ready: (
                "Android 更新已就緒",
                "已下載 Android {v}。下次啟動 Android 時將會使用此版本。",
            ),
            android_restart: (
                "重新啟動 Android 以完成更新",
                "已下載 Android {v}。立即重新啟動 Android 即可使用此版本，或在關閉所有 \
                 Android 應用程式視窗後使用。",
            ),
            android_applied: ("Android 已更新", "Android {v} 已啟動，現正使用中。"),
            android_failed: ("Android 更新失敗", "無法套用 Android {v}。"),
            program_ready: (
                "DSA 更新已就緒",
                "已下載 DSA {v}。它會在 Android 未執行時安裝；您也可以在 DSA 設定中立即安裝。",
            ),
            program_restart: ("DSA 更新已就緒", "已下載 DSA {v}。"),
            program_applied: ("DSA 已更新", "已安裝 DSA {v}。"),
            program_failed: ("DSA 更新失敗", "無法安裝 DSA {v}，仍在使用先前的版本。"),
            why: "原因：",
            restart_now: "立即重新啟動",
            later: "稍後",
        },
        UiLanguage::Spanish => AlertWords {
            android_ready: (
                "Actualización de Android lista",
                "Se descargó Android {v}. Se usará la próxima vez que se inicie Android.",
            ),
            android_restart: (
                "Reinicie Android para actualizar",
                "Se descargó Android {v}. Reinicie Android ahora para usarlo, o se usará \
                 cuando se cierren todas las ventanas de aplicaciones de Android.",
            ),
            android_applied: (
                "Android actualizado",
                "Android {v} se inició y ya está en uso.",
            ),
            android_failed: ("Error al actualizar Android", "No se aplicó Android {v}."),
            program_ready: (
                "Actualización de DSA lista",
                "Se descargó DSA {v}. Se instalará cuando Android no esté en ejecución; puede \
                 instalarla ahora desde la configuración de DSA.",
            ),
            program_restart: ("Actualización de DSA lista", "Se descargó DSA {v}."),
            program_applied: ("DSA actualizado", "Se instaló DSA {v}."),
            program_failed: (
                "Error al actualizar DSA",
                "No se instaló DSA {v}; se sigue usando la versión anterior.",
            ),
            why: " Motivo: ",
            restart_now: "Reiniciar ahora",
            later: "Más tarde",
        },
        UiLanguage::French => AlertWords {
            android_ready: (
                "Mise à jour d'Android prête",
                "Android {v} a été téléchargé. Il sera utilisé au prochain démarrage d'Android.",
            ),
            android_restart: (
                "Redémarrez Android pour le mettre à jour",
                "Android {v} a été téléchargé. Redémarrez Android maintenant pour l'utiliser, \
                 sinon il sera utilisé une fois toutes les fenêtres d'applications Android \
                 fermées.",
            ),
            android_applied: (
                "Android mis à jour",
                "Android {v} a démarré et est maintenant utilisé.",
            ),
            android_failed: (
                "Échec de la mise à jour d'Android",
                "Android {v} n'a pas été appliqué.",
            ),
            program_ready: (
                "Mise à jour de DSA prête",
                "DSA {v} a été téléchargé. Il s'installera quand Android ne sera pas en cours \
                 d'exécution ; vous pouvez aussi l'installer maintenant depuis les paramètres de \
                 DSA.",
            ),
            program_restart: ("Mise à jour de DSA prête", "DSA {v} a été téléchargé."),
            program_applied: ("DSA mis à jour", "DSA {v} a été installé."),
            program_failed: (
                "Échec de la mise à jour de DSA",
                "DSA {v} n'a pas été installé ; la version précédente est toujours utilisée.",
            ),
            why: " Raison : ",
            restart_now: "Redémarrer maintenant",
            later: "Plus tard",
        },
        UiLanguage::German => AlertWords {
            android_ready: (
                "Android-Update bereit",
                "Android {v} wurde heruntergeladen. Es wird beim nächsten Start von Android \
                 verwendet.",
            ),
            android_restart: (
                "Android zum Aktualisieren neu starten",
                "Android {v} wurde heruntergeladen. Starten Sie Android jetzt neu, um es zu \
                 verwenden, oder es wird verwendet, sobald alle Fenster von Android-Apps \
                 geschlossen sind.",
            ),
            android_applied: (
                "Android aktualisiert",
                "Android {v} wurde gestartet und wird jetzt verwendet.",
            ),
            android_failed: (
                "Android-Update fehlgeschlagen",
                "Android {v} wurde nicht angewendet.",
            ),
            program_ready: (
                "DSA-Update bereit",
                "DSA {v} wurde heruntergeladen. Es wird installiert, wenn Android nicht \
                 ausgeführt wird; in den DSA-Einstellungen können Sie es jetzt installieren.",
            ),
            program_restart: ("DSA-Update bereit", "DSA {v} wurde heruntergeladen."),
            program_applied: ("DSA aktualisiert", "DSA {v} wurde installiert."),
            program_failed: (
                "DSA-Update fehlgeschlagen",
                "DSA {v} wurde nicht installiert; die vorherige Version wird weiterhin \
                 verwendet.",
            ),
            why: " Grund: ",
            restart_now: "Jetzt neu starten",
            later: "Später",
        },
        UiLanguage::PortugueseBrazil => AlertWords {
            android_ready: (
                "Atualização do Android pronta",
                "O Android {v} foi baixado. Ele será usado na próxima vez que o Android \
                 iniciar.",
            ),
            android_restart: (
                "Reinicie o Android para atualizar",
                "O Android {v} foi baixado. Reinicie o Android agora para usá-lo, ou ele será \
                 usado quando todas as janelas de apps Android forem fechadas.",
            ),
            android_applied: (
                "Android atualizado",
                "O Android {v} foi iniciado e já está em uso.",
            ),
            android_failed: (
                "Falha na atualização do Android",
                "O Android {v} não foi aplicado.",
            ),
            program_ready: (
                "Atualização do DSA pronta",
                "O DSA {v} foi baixado. Ele será instalado quando o Android não estiver em \
                 execução; você pode instalá-lo agora nas Configurações do DSA.",
            ),
            program_restart: ("Atualização do DSA pronta", "O DSA {v} foi baixado."),
            program_applied: ("DSA atualizado", "O DSA {v} foi instalado."),
            program_failed: (
                "Falha na atualização do DSA",
                "O DSA {v} não foi instalado; a versão anterior continua em uso.",
            ),
            why: " Motivo: ",
            restart_now: "Reiniciar agora",
            later: "Mais tarde",
        },
        UiLanguage::Russian => AlertWords {
            android_ready: (
                "Обновление Android готово",
                "Android {v} загружен. Он будет использоваться при следующем запуске Android.",
            ),
            android_restart: (
                "Перезапустите Android для обновления",
                "Android {v} загружен. Перезапустите Android сейчас, чтобы использовать его, \
                 или он будет применён после закрытия всех окон приложений Android.",
            ),
            android_applied: (
                "Android обновлён",
                "Android {v} запущен и уже используется.",
            ),
            android_failed: ("Не удалось обновить Android", "Android {v} не применён."),
            program_ready: (
                "Обновление DSA готово",
                "DSA {v} загружен. Он будет установлен, когда Android не запущен; установить \
                 его сейчас можно в параметрах DSA.",
            ),
            program_restart: ("Обновление DSA готово", "DSA {v} загружен."),
            program_applied: ("DSA обновлён", "DSA {v} установлен."),
            program_failed: (
                "Не удалось обновить DSA",
                "DSA {v} не установлен; по-прежнему используется предыдущая версия.",
            ),
            why: " Причина: ",
            restart_now: "Перезапустить сейчас",
            later: "Позже",
        },
        UiLanguage::Italian => AlertWords {
            android_ready: (
                "Aggiornamento di Android pronto",
                "Android {v} è stato scaricato. Verrà usato al prossimo avvio di Android.",
            ),
            android_restart: (
                "Riavvia Android per aggiornarlo",
                "Android {v} è stato scaricato. Riavvia Android ora per usarlo, oppure verrà \
                 usato quando tutte le finestre delle app Android saranno chiuse.",
            ),
            android_applied: (
                "Android aggiornato",
                "Android {v} è stato avviato ed è ora in uso.",
            ),
            android_failed: (
                "Aggiornamento di Android non riuscito",
                "Android {v} non è stato applicato.",
            ),
            program_ready: (
                "Aggiornamento di DSA pronto",
                "DSA {v} è stato scaricato. Verrà installato quando Android non è in \
                 esecuzione; puoi installarlo subito dalle impostazioni di DSA.",
            ),
            program_restart: ("Aggiornamento di DSA pronto", "DSA {v} è stato scaricato."),
            program_applied: ("DSA aggiornato", "DSA {v} è stato installato."),
            program_failed: (
                "Aggiornamento di DSA non riuscito",
                "DSA {v} non è stato installato; è ancora in uso la versione precedente.",
            ),
            why: " Motivo: ",
            restart_now: "Riavvia ora",
            later: "Più tardi",
        },
        UiLanguage::Vietnamese => AlertWords {
            android_ready: (
                "Bản cập nhật Android đã sẵn sàng",
                "Đã tải xuống Android {v}. Phiên bản này sẽ được dùng vào lần khởi động \
                 Android tiếp theo.",
            ),
            android_restart: (
                "Khởi động lại Android để cập nhật",
                "Đã tải xuống Android {v}. Hãy khởi động lại Android ngay để dùng phiên bản \
                 này, hoặc phiên bản này sẽ được dùng khi mọi cửa sổ ứng dụng Android đã đóng.",
            ),
            android_applied: (
                "Đã cập nhật Android",
                "Android {v} đã khởi động và hiện đang được sử dụng.",
            ),
            android_failed: (
                "Cập nhật Android không thành công",
                "Không áp dụng được Android {v}.",
            ),
            program_ready: (
                "Bản cập nhật DSA đã sẵn sàng",
                "Đã tải xuống DSA {v}. Bản cập nhật sẽ được cài đặt khi Android không chạy; \
                 bạn có thể cài đặt ngay trong Cài đặt DSA.",
            ),
            program_restart: ("Bản cập nhật DSA đã sẵn sàng", "Đã tải xuống DSA {v}."),
            program_applied: ("Đã cập nhật DSA", "Đã cài đặt DSA {v}."),
            program_failed: (
                "Cập nhật DSA không thành công",
                "Không cài đặt được DSA {v}; phiên bản trước vẫn đang được sử dụng.",
            ),
            why: " Lý do: ",
            restart_now: "Khởi động lại ngay",
            later: "Để sau",
        },
        UiLanguage::Indonesian => AlertWords {
            android_ready: (
                "Pembaruan Android siap",
                "Android {v} sudah diunduh. Versi ini digunakan saat Android dimulai lagi.",
            ),
            android_restart: (
                "Mulai ulang Android untuk memperbarui",
                "Android {v} sudah diunduh. Mulai ulang Android sekarang untuk menggunakannya, \
                 atau versi ini digunakan setelah semua jendela aplikasi Android ditutup.",
            ),
            android_applied: (
                "Android diperbarui",
                "Android {v} sudah dimulai dan kini digunakan.",
            ),
            android_failed: ("Pembaruan Android gagal", "Android {v} tidak diterapkan."),
            program_ready: (
                "Pembaruan DSA siap",
                "DSA {v} sudah diunduh. Pembaruan ini diinstal saat Android tidak berjalan; \
                 Anda dapat menginstalnya sekarang di Pengaturan DSA.",
            ),
            program_restart: ("Pembaruan DSA siap", "DSA {v} sudah diunduh."),
            program_applied: ("DSA diperbarui", "DSA {v} sudah diinstal."),
            program_failed: (
                "Pembaruan DSA gagal",
                "DSA {v} tidak diinstal; versi sebelumnya masih digunakan.",
            ),
            why: " Alasan: ",
            restart_now: "Mulai ulang sekarang",
            later: "Nanti",
        },
        UiLanguage::Turkish => AlertWords {
            android_ready: (
                "Android güncellemesi hazır",
                "Android {v} indirildi. Android bir sonraki başlatılışında kullanılacak.",
            ),
            android_restart: (
                "Güncellemek için Android'i yeniden başlatın",
                "Android {v} indirildi. Kullanmak için Android'i şimdi yeniden başlatın; aksi \
                 takdirde tüm Android uygulama pencereleri kapatıldığında kullanılacak.",
            ),
            android_applied: (
                "Android güncellendi",
                "Android {v} başlatıldı ve artık kullanımda.",
            ),
            android_failed: (
                "Android güncellemesi başarısız oldu",
                "Android {v} uygulanamadı.",
            ),
            program_ready: (
                "DSA güncellemesi hazır",
                "DSA {v} indirildi. Android çalışmıyorken yüklenecek; DSA Ayarları'ndan hemen de \
                 yükleyebilirsiniz.",
            ),
            program_restart: ("DSA güncellemesi hazır", "DSA {v} indirildi."),
            program_applied: ("DSA güncellendi", "DSA {v} yüklendi."),
            program_failed: (
                "DSA güncellemesi başarısız oldu",
                "DSA {v} yüklenemedi; önceki sürüm kullanılmaya devam ediyor.",
            ),
            why: " Neden: ",
            restart_now: "Şimdi yeniden başlat",
            later: "Daha sonra",
        },
    }
}

/// Whether `key` is an update alert's notification key, or the key the alerts about one
/// subject share ([`thread_of`]), which a host may show an alert under. A click on one opens
/// the settings app's update page on the host; nothing about it goes to Android.
pub fn is_update_key(key: &str) -> bool {
    UpdateAlert::parse_key(key).is_some()
        || key
            .strip_prefix(KEY_PREFIX)
            .is_some_and(|rest| Subject::of_word(rest).is_some())
}

/// The key that the alerts about one subject share, such as `dsa:update:android`: a host shows
/// one alert per subject at a time, so the news that an update was applied replaces the news
/// that it was ready. `None` for a key that is no update alert's.
pub fn thread_of(key: &str) -> Option<String> {
    let (subject, _, _) = UpdateAlert::parse_key(key)?;
    Some(format!("{KEY_PREFIX}{}", subject.word()))
}

/// The alerts the service raised, and which of them each user was told.
#[derive(Default)]
pub struct UpdateAlerts(Mutex<Board>);

#[derive(Default)]
struct Board {
    /// In the order they were raised.
    raised: Vec<UpdateAlert>,
    /// The keys of the alerts each user was told, by user.
    told: HashMap<String, HashSet<String>>,
}

impl UpdateAlerts {
    fn board(&self) -> std::sync::MutexGuard<'_, Board> {
        self.0.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Raises `alert`. An alert raised before (the same subject, kind and version) is not
    /// raised again. Returns whether it was new.
    pub fn raise(&self, alert: UpdateAlert) -> bool {
        let mut board = self.board();
        let key = alert.key();
        if board.raised.iter().any(|known| known.key() == key) {
            return false;
        }
        board.raised.push(alert);
        if board.raised.len() > KEPT {
            board.raised.remove(0);
        }
        true
    }

    /// Every alert raised and kept, in the order they were raised.
    pub fn raised(&self) -> Vec<UpdateAlert> {
        self.board().raised.clone()
    }

    /// What `user` has not been told yet: for each subject, the latest alert, unless `user`
    /// was told it. An older alert is never told once a newer one about the same subject is
    /// raised, as it is news no longer.
    pub fn untold(&self, user: &str) -> Vec<UpdateAlert> {
        let board = self.board();
        let told = board.told.get(user);
        let mut latest: Vec<&UpdateAlert> = Vec::new();
        for alert in board.raised.iter().rev() {
            if !latest.iter().any(|seen| seen.subject == alert.subject) {
                latest.push(alert);
            }
        }
        latest
            .into_iter()
            .rev()
            .filter(|alert| told.is_none_or(|told| !told.contains(&alert.key())))
            .cloned()
            .collect()
    }

    /// `user`'s desktop was told `alert`.
    pub fn told(&self, user: &str, alert: &UpdateAlert) {
        let mut board = self.board();
        let key = alert.key();
        if board.raised.iter().any(|known| known.key() == key) {
            board.told.entry(user.to_string()).or_default().insert(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fr14_an_alert_key_names_what_it_is_about() {
        let alert = UpdateAlert::new(Subject::Android, AlertKind::Ready, "17.0.0-16373615-8");
        assert_eq!(alert.key(), "dsa:update:android:ready:17.0.0-16373615-8");
        assert_eq!(
            UpdateAlert::parse_key(&alert.key()),
            Some((Subject::Android, AlertKind::Ready, "17.0.0-16373615-8"))
        );
        assert!(is_update_key(&alert.key()));
        assert!(is_update_key("dsa:update:program"));
        for other in [
            "dsa:update:",
            "dsa:update:kernel",
            "dsa:update:android:ready:",
            "dsa:update:android:soon:16.0.3",
            "dsa:update:kernel:ready:16.0.3",
            "dsa:run-as-arm:offer:com.example.game",
            "0|com.example.chat|1|null|10100",
        ] {
            assert!(!is_update_key(other), "{other}");
        }
        assert_eq!(
            thread_of("dsa:update:program:failed:0.1.10").as_deref(),
            Some("dsa:update:program")
        );
        assert_eq!(thread_of("0|com.example|1|null|1"), None);
    }

    #[test]
    fn fr14_the_question_to_restart_has_restart_now_and_later() {
        let asked = UpdateAlert::new(Subject::Android, AlertKind::Restart, "16.0.3");
        assert_eq!(asked.key(), "dsa:update:android:restart:16.0.3");
        assert!(is_update_key(&asked.key()));
        assert_eq!(
            thread_of(&asked.key()).as_deref(),
            Some("dsa:update:android"),
            "the news that it was applied replaces the question"
        );
        let buttons = buttons_of(&asked.key(), UiLanguage::English);
        assert_eq!(
            buttons
                .iter()
                .map(|b| (b.action, b.label.as_str()))
                .collect::<Vec<_>>(),
            [(RESTART_NOW, "Restart now"), (LATER, "Later")]
        );
        let korean = buttons_of(&asked.key(), UiLanguage::Korean);
        assert_eq!(korean[0].label, "지금 다시 시작");
        assert_eq!(korean[1].label, "나중에");
        let (title, text) = asked.text(UiLanguage::English);
        assert_eq!(title, "Restart Android to update");
        assert!(text.contains("Android 16.0.3"), "{text}");
        let ready = UpdateAlert::new(Subject::Android, AlertKind::Ready, "16.0.3");
        assert!(buttons_of(&ready.key(), UiLanguage::English).is_empty());
        assert!(buttons_of("0|com.example|1|null|1", UiLanguage::English).is_empty());
    }

    #[test]
    fn fr15_an_alert_is_shown_in_english_and_korean() {
        let failed = UpdateAlert::failed(Subject::Program, "0.1.10", "setup ended with 1603");
        let (title, text) = failed.text(UiLanguage::English);
        assert_eq!(title, "DSA update failed");
        assert!(text.contains("DSA 0.1.10") && text.ends_with("Why: setup ended with 1603"));
        let (title, text) = failed.text(UiLanguage::Korean);
        assert_eq!(title, "DSA 업데이트에 실패했습니다");
        assert!(text.contains("이유: setup ended with 1603"), "{text}");
        let notification = failed.notification();
        assert_eq!(notification.title, "DSA update failed");
        assert_eq!(notification.text, "setup ended with 1603");
        assert_eq!(UpdateAlert::of_notification(&notification), Some(failed));
    }

    fn every_alert() -> Vec<UpdateAlert> {
        let mut out = Vec::new();
        for subject in [Subject::Android, Subject::Program] {
            for kind in [
                AlertKind::Ready,
                AlertKind::Restart,
                AlertKind::Applied,
                AlertKind::Failed,
            ] {
                out.push(UpdateAlert {
                    detail: "setup ended with 1603".into(),
                    ..UpdateAlert::new(subject, kind, "16.0.3-1234")
                });
            }
        }
        out
    }

    /// Every alert reads in every interface language: a title and a text, none empty and none
    /// with an em dash, naming the version and, for a failure, why it failed. The question to
    /// restart has both buttons in every language.
    #[test]
    fn fr17_every_update_alert_is_shown_in_every_language() {
        let asked = UpdateAlert::new(Subject::Android, AlertKind::Restart, "16.0.3").key();
        for lang in UiLanguage::ALL {
            for alert in every_alert() {
                let (title, text) = alert.text(lang);
                assert!(!title.trim().is_empty(), "{lang:?} {:?}", alert.kind);
                assert!(!text.trim().is_empty(), "{lang:?} {:?}", alert.kind);
                assert!(!title.contains('\u{2014}') && !text.contains('\u{2014}'));
                assert!(!text.contains("{v}"), "{lang:?}: {text}");
                assert!(text.contains("16.0.3-1234"), "{lang:?}: {text}");
                let named = match alert.subject {
                    Subject::Android => "Android",
                    Subject::Program => "DSA",
                };
                assert!(text.contains(named), "{lang:?}: {text}");
                assert_eq!(
                    text.ends_with("setup ended with 1603"),
                    alert.kind == AlertKind::Failed,
                    "{lang:?}: {text}"
                );
            }
            let buttons = buttons_of(&asked, lang);
            assert_eq!(buttons.len(), 2, "{lang:?}");
            assert_eq!((buttons[0].action, buttons[1].action), (RESTART_NOW, LATER));
            for button in &buttons {
                assert!(!button.label.trim().is_empty(), "{lang:?}");
                assert!(!button.label.contains('\u{2014}'), "{lang:?}");
            }
            assert_ne!(buttons[0].label, buttons[1].label, "{lang:?}");
        }
        // A language with no translation reads in English.
        let english = every_alert()[0].text(UiLanguage::English);
        assert_eq!(
            every_alert()[0].text(UiLanguage::from_tag_or_english("pl-PL")),
            english
        );
        let (title, _) = every_alert()[0].text(UiLanguage::from_tag_or_english("zh-HK"));
        assert_eq!(title, "Android 更新已就緒");
    }
}
