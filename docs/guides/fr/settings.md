# Réglages

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · **Français** · [Deutsch](../de/settings.md) · [Português (Brasil)](../pt-BR/settings.md) · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** est l'application de réglages de Desktop Subsystem for Android™ (DSA) sous
Windows. Ouvrez-la depuis le menu Démarrer. Elle s'affiche en anglais ou en coréen, selon la
langue d'affichage de Windows, et suit votre thème Windows. Elle fonctionne au clavier et
avec les lecteurs d'écran.

Chaque modification faite par DSA Settings passe par la commande `dsa`, donc la ligne de
commande voit les mêmes valeurs :

```
dsa settings                        # tous les réglages et leur valeur
dsa settings set <name> <value>     # en modifie un ; les interrupteurs prennent on ou off
```

Certains réglages s'appliquent à tous les utilisateurs du PC, donc seul un administrateur peut
les modifier. DSA Settings les signale par une icône de bouclier et demande l'approbation d'un
administrateur quand vous en modifiez un. Ils sont marqués « (admin) » ci-dessous.

## Pages

| Page | Ce que vous y trouvez | Ligne de commande |
|---|---|---|
| **General** (général) | L'état d'Android, **Start**, **Stop**, **Freeze Android now**, le dernier problème, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (mémoire) | **Memory limit**, la taille de la VM d'Android, ce que chaque partie utilise | `memory-limit`, `dsa memory` |
| **Apps** (applications) | Les applications installées : **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Pour chaque application : **Resizable**, la taille à laquelle les fenêtres s'ouvrent, **Scaling on another monitor**, **Full screen**, **Bars hidden**, **Run as ARM**, **Root access**. Une ligne pour l'accès root d'**adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (en arrière-plan) | Les applications qui tournent sans fenêtre : **Open**, **Stop** ; demander ou non avant d'arrêter | `dsa background`, `stop-without-asking` |
| **Widgets** | Les widgets sur votre bureau, et **Add to desktop** | `dsa widgets` |
| **Display & GPU** (affichage et GPU) | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (Windows 11 uniquement, activé par défaut), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (identité de l'appareil) | D'abord cet ordinateur, en lecture seule : son nom, son fabricant et son modèle, et **Device specifications** avec **Copy**. Ensuite le fabricant, le modèle et l'empreinte de build qu'Android communique aux applications | `dsa settings device`, `device.<field>` |
| **Network** (réseau) | Comment Android accède au réseau, son adresse, qui répond à ses noms (DNS) et sa passerelle, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (mises à jour) | **Android** : la version et le build en cours d'exécution, le niveau de correctif de sécurité, la version suivie, la mise à jour Android avec sa progression et **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA** : **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (avancé) | Presse-papiers, fichiers, liens web, gel au repos, sauvegarde, réinitialisation, applications ARM, Google Play, contenu protégé, accès root, traces (admin), dossiers de journaux | voir ci-dessous |
| **Logs** (journaux) | Le journal en direct d'Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (à propos) | Versions, licences, code source | |

## La page Advanced

| Réglage | Par défaut | Ligne de commande |
|---|---|---|
| **Share the clipboard with Android** | Activé | `clipboard` |
| **Share Windows folders with Android** | Activé | `file-sharing` |
| **Shared folders** | Les six | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Désactivé | `freeze` |
| **Open web links in Android apps** | Désactivé | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Désactivé | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (pas sur ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (pas sur ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Non installé | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (sous **Protected content**) | Non installé | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Désactivé | `root-elevation` |
| **Input trace** (admin) | Désactivé | `input-trace` |
| **Resize trace** (admin) | Désactivé | `resize-trace` |

La sauvegarde conserve une copie chiffrée des données de vos applications dans votre dossier
utilisateur, avec une clé que Windows garde pour vous. Pour la restaurer sur un autre PC, ou
après une réinstallation de Windows, il vous faut la clé de récupération : notez-la et
gardez-la ailleurs que sur le PC. Google Play, Widevine et l'accès root sont décrits dans
[Applications et fenêtres](apps.md).

## Quand une modification s'applique

- **Render GPU** : au prochain démarrage du gestionnaire de fenêtres, à votre prochaine
  ouverture de session ou quand vous sélectionnez **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, l'activation de l'accès root et la version d'Android : au
  prochain démarrage d'Android. Pour redémarrer Android, arrêtez-le sur la page **General**,
  puis ouvrez une application.
- Les réglages de fenêtre d'une application : à partir de la prochaine fenêtre que
  l'application ouvre.
- Tout le reste, y compris la désactivation de l'accès root : immédiatement.

## Comment Android résout les noms

Votre ordinateur répond à chaque nom que les applications d'Android recherchent, avec le même
résolveur que celui des applications Windows (sous Linux et macOS, le résolveur du système).
Ainsi, votre fichier hosts, les règles de noms de votre entreprise, les serveurs DNS propres à
un VPN et un filtre DNS s'appliquent aux applications d'Android comme à l'ordinateur. Android
n'interroge pas lui-même de serveur DNS.

- C'est la même chose avec **Advanced networking** activé : votre ordinateur est sur le même
  réseau et connaît ses noms, et un VPN ou un filtre DNS sur l'ordinateur s'applique toujours.
- Le réglage **Private DNS** (DNS privé) d'Android est désactivé par défaut, car il
  interrogerait un serveur DNS en contournant votre ordinateur. Si vous choisissez un serveur DNS privé dans les paramètres
  d'Android, Android l'utilise.
- Si DSA ne peut pas répondre pendant un moment, Android interroge les serveurs DNS du réseau
  jusqu'à ce que DSA puisse de nouveau répondre.
- La ligne **DNS** de la page **Network**, et `dsa network`, indiquent qui répond en ce
  moment : le résolveur de votre ordinateur, ou les serveurs DNS du réseau, qu'ils listent.

## Valeurs acceptées par la ligne de commande

- `render-gpu` : `display` (le GPU de l'écran principal, la valeur par défaut), `integrated`,
  `discrete`, ou une partie du nom d'un GPU tel que `dsa gpus` le liste.
- `memory-limit` : `auto` (la valeur par défaut : aucune limite fixe, jusqu'à la mémoire du PC
  moins ce que Windows garde), ou une taille comme `4g` ou `3072` (Mio).
- `shared-folders` : `all`, `none`, ou des noms séparés par des virgules, comme
  `Documents,Pictures`.
- `adb-port` : un port TCP de 1 à 65535. La valeur par défaut est 58526.
- `app-fullscreen` : `screen` (la valeur par défaut) ou `window`, pour une demande explicite
  de plein écran d'une application. `app-immersive` : `screen` ou `window` (la valeur par
  défaut), pour une application qui masque seulement ses barres. Voir
  [Plein écran](apps.md#plein-écran).
- `touch-gestures-off` : `on` (la valeur par défaut) désactive les gestes tactiles à trois et
  quatre doigts de Windows 11 tant qu'une fenêtre d'application DSA est au premier plan, afin
  que les applications reçoivent jusqu'à dix doigts, et rétablit votre propre réglage quand
  une autre fenêtre passe au premier plan. Si Windows refuse, le réglage ne peut pas être
  utilisé et l'application de réglages en indique la raison ; vous pouvez toujours désactiver
  les gestes pour toutes les applications dans les Paramètres de Windows, sous Bluetooth et
  appareils, Tactile.
- `host-certificates` : `on` (la valeur par défaut) permet aux applications Android de faire
  confiance aux certificats racines ajoutés à ce PC, comme ceux de votre entreprise ou d'un
  logiciel de sécurité.
- `adb-auto-connect` : `on` (la valeur par défaut) affiche Android dans `adb devices` et dans
  Android Studio tant que le mode développeur est activé, sans `adb connect`.
- `image-auto-restart` : `on` (la valeur par défaut) redémarre Android sur une mise à jour
  d'image téléchargée dès qu'aucune fenêtre d'application n'est ouverte. Sinon, DSA demande
  d'abord, ou attend que vous les fermiez.
- `prerelease-updates` : `on` prend les préversions en plus des versions stables, `off` ne
  prend que les versions stables. Sa valeur initiale dépend du programme d'installation
  utilisé : un programme d'installation de préversion l'active, un programme stable le laisse
  désactivé, et les mises à niveau conservent votre choix. Toutes les versions antérieures à
  1.0 sont des préversions. Le désactiver ne fait jamais revenir DSA à une version plus
  ancienne ; DSA attend une version stable plus récente que celle installée.
- `root-elevation` : `on` permet aux applications que vous autorisez sur la page **Apps** de
  demander root. Le niveau de chaque application se règle avec
  `dsa apps root <package> off|ask|remember`.
- `device.<field>` : `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, ou une partie de l'empreinte comme `fingerprint.release`.
  `dsa settings reset device` rétablit toutes les valeurs.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
