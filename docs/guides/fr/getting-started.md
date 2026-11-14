# Prise en main

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · **Français** · [Deutsch](../de/getting-started.md) · [Português (Brasil)](../pt-BR/getting-started.md) · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

Desktop Subsystem for Android™ (DSA) exécute Android dans une machine virtuelle légère dotée
de son propre noyau. Chaque application Android s'ouvre dans sa propre fenêtre sur votre
bureau.

## État par plateforme

| Hôte | État |
|---|---|
| Windows 11, x64 | Publié. |
| Windows 11, ARM64 | Publié. Android est lui aussi en ARM64, donc les applications ARM fonctionnent sans traduction. |
| Linux, x64 et ARM64 | En cours. Les paquets `.deb` et `.rpm` sont compilés en CI ; pas encore publiés. |
| macOS, Apple silicon | En cours. `DSA.app`, son `.pkg` et son `.dmg` sont compilés en CI ; pas encore publiés. |

## Installation sous Windows

Il vous faut Windows 11 sur un PC x64 ou ARM64, avec la fonctionnalité Windows
**Plateforme de machine virtuelle** activée.

1. Activez la Plateforme de machine virtuelle. Ouvrez **Activer ou désactiver des
   fonctionnalités Windows**, cochez **Plateforme de machine virtuelle**, sélectionnez
   **OK**, puis redémarrez. Ou exécutez
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` dans un terminal
   administrateur. Le programme d'installation ne l'active pas pour vous : il s'arrête avec
   un message si la fonctionnalité est désactivée.
2. Téléchargez le programme d'installation et `SHA256SUMS.txt` depuis la
   [dernière version](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest) :
   `DSA-v<version>-win-x64-setup.exe` sur un PC x64, `DSA-v<version>-win-arm64-setup.exe` sur
   un PC ARM64.
3. Vérifiez le téléchargement. Le hachage affiché par cette commande doit correspondre à la
   ligne de `SHA256SUMS.txt` :

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Lancez le programme d'installation, sélectionnez **Install** (installer) et approuvez
   l'invite d'administrateur.
5. Attendez la première image Android. Si le programme d'installation n'en contient pas, le
   service DSA la télécharge en arrière-plan. `dsa status` affiche le téléchargement.
6. Ouvrez une application depuis le menu Démarrer, ou vérifiez depuis un terminal :

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## Ce qu'installe le programme d'installation

- Le service `DsaHost`, qui exécute l'Android de chaque utilisateur.
- La commande `dsa`.
- **DSA Settings** dans le menu Démarrer.
- **Android** dans le volet de navigation de l'Explorateur de fichiers, qui ouvre les
  fichiers d'Android.
- L'APK Installer, qui ouvre les fichiers `.apk`.
- Le gestionnaire de fenêtres de DSA (`dsa-wm`). Il démarre à l'ouverture de votre session et
  démarre alors Android, donc la première application que vous ouvrez n'attend pas le
  démarrage d'Android.

Chaque utilisateur Windows a son propre Android et son propre disque de données,
`%LOCALAPPDATA%\DSA\android.vhdx`. Les autres utilisateurs ne peuvent pas le lire.

DSA utilise son propre noyau et sa propre machine virtuelle. Il n'utilise pas et ne modifie
pas vos paramètres, votre noyau ni vos distributions WSL.

## Mises à jour

DSA se met à jour lui-même, ainsi que l'image Android. Les deux sont signés, et DSA
n'applique rien dont la signature, la taille ou le hachage ne correspond pas.

- **DSA lui-même** est mis à jour quand l'Android d'aucun utilisateur ne tourne. D'ici là, la
  mise à jour attend ; `dsa update` l'installe immédiatement. DSA ne passe jamais à une
  version inférieure.
- **Canaux de publication.** À partir de 1.0, DSA a un canal stable et un canal de
  préversion pour les versions encore en cours de finition. Votre canal dépend du programme
  d'installation utilisé, et les mises à niveau le conservent. Un administrateur le change
  avec **Get pre-release versions** (obtenir les préversions) sur la page **Updates** (mises à
  jour) de DSA Settings. Toutes les versions antérieures à 1.0 sont des préversions.
- **L'image Android** ne dépend pas du canal. Si une image ne parvient pas à démarrer, DSA
  revient à la précédente, et `dsa status` le signale.

Un administrateur peut désactiver les mises à jour sur la page **Updates** de DSA Settings.
Consultez [Réglages](settings.md) pour tous les interrupteurs.

## Linux (en cours, non publié)

DSA pour Linux est compilé sous forme de `.deb` (Debian, Ubuntu) et de `.rpm` (Fedora,
openSUSE). Il nécessite KVM (`/dev/kvm`) et un bureau Wayland. Android tourne dans une machine
virtuelle crosvm.

L'installation d'un paquet ajoute aussi le dépôt de paquets signé de DSA, donc les versions
suivantes arrivent par `apt`, `dnf`, `zypper` ou la logithèque. Sous Linux, DSA n'a pas de
programme de mise à jour propre.

- Debian et Ubuntu : `/etc/apt/sources.list.d/dsa.sources`, avec la clé dans
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora et openSUSE : `/etc/yum.repos.d/dsa.repo` (et `/etc/zypp/repos.d/dsa.repo` là où
  zypper est présent), avec la clé dans `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

Le dépôt doit être servi depuis
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Il n'est pas encore publié.

Une fois publié, la première installation ressemble à ceci :

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

Le paquet installe le service système `dsa-hostd`, les services utilisateur `dsa-devices` et
`dsa-compositor`, la commande `dsa` dans `/usr/bin`, l'entrée de menu **DSA Files**, et les
actions sur les fichiers **Open in an Android app** (ouvrir dans une application Android),
**Share with an Android app** (partager avec une application Android) et **Install in
Android** (installer dans Android).

## macOS (en cours, non publié)

DSA pour macOS exécute Android sur le framework Virtualization d'Apple, sur Apple silicon. Il
est compilé sous forme de `DSA.app`, dans un paquet d'installation (`DSA-<version>.pkg`) et
une image disque (`DSA-<version>.dmg`). Les builds de CI sont signés ad hoc, donc ils ne
fonctionnent que sur la machine qui les a compilés. Aucune version signée avec un Developer
ID n'existe encore.

## Ensuite

- [Applications et fenêtres](apps.md)
- [Installer des APK](install-apks.md)

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
macOS est une marque d'Apple Inc. DSA n'est ni affilié à Google, Microsoft ou Apple, ni
approuvé ou parrainé par eux.
