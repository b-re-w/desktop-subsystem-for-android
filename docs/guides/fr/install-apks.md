# Installer des APK

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · **Français** · [Deutsch](../de/install-apks.md) · [Português (Brasil)](../pt-BR/install-apks.md) · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

Desktop Subsystem for Android™ (DSA) ne fournit aucune boutique d'applications. Vous
installez les applications à partir de fichiers APK, avec l'APK Installer ou la commande
`dsa`. Cette page décrit DSA sous Windows.

## Installer avec l'APK Installer

1. Double-cliquez sur un fichier `.apk` dans l'Explorateur de fichiers. Il s'ouvre dans
   l'**APK Installer**.
2. Vérifiez ce qu'est l'APK. La fenêtre l'affiche sans démarrer Android :
   - le nom, l'icône, le paquet et la version de l'application ;
   - l'éditeur, tiré du certificat de signature, car un APK ne nomme aucun éditeur ;
   - la source : l'hôte depuis lequel le fichier a été téléchargé, ou son dossier ;
   - sous **Capabilities** (capacités), les premières autorisations qu'il demande. **More**
     (plus) les affiche toutes, ainsi que les versions d'Android pour lesquelles il est conçu,
     sa taille, ses types de processeur et le SHA-256 de son certificat.

   Quand l'APK n'a du code que pour un autre type de processeur, la fenêtre vous avertit.
3. Sélectionnez **Install** (installer). Quand l'application est déjà installée, le bouton
   indique **Update** (mettre à jour) ou **Install again** (réinstaller). Android démarre
   d'abord s'il ne tourne pas.
4. Avec **Launch when ready** (lancer une fois prêt) coché (par défaut), l'application s'ouvre
   dès qu'elle est installée et le programme d'installation se ferme. Sinon, sélectionnez
   **Launch** (lancer), ou ouvrez l'application plus tard depuis le menu Démarrer.

Quand Android tourne, la fenêtre indique aussi si le fichier est une nouvelle installation,
une mise à jour, la même version une nouvelle fois, ou quelque chose qu'Android refuse : une
version plus ancienne, ou la même application signée avec une autre clé.

Vous pouvez aussi déposer des fichiers APK sur l'APK Installer, sur DSA Settings ou sur la
fenêtre d'une application. Chaque APK s'ouvre dans sa propre fenêtre d'installation.

## Installer depuis la ligne de commande

```
dsa apk-info app.apk            # ce qu'est l'APK ; lit le fichier sans démarrer Android
dsa install app.apk             # l'installe ; démarre Android s'il ne tourne pas
dsa install app.apk --progress  # affiche aussi chaque étape
```

Quand Android refuse l'APK, `dsa install` se termine avec le code 1, et son message commence
par `not installed (<reason>):`.

## Pourquoi une installation est refusée

| Raison | Signification | Que faire |
|---|---|---|
| `incompatible` | L'application nécessite un autre type de processeur, une version d'Android plus récente ou une fonctionnalité dont cet appareil est dépourvu | Voir [Applications conçues uniquement pour ARM](apps.md#applications-conçues-uniquement-pour-arm) ou [Versions d'Android](android-versions.md) |
| `conflicting-signature` | L'application est déjà installée, signée avec une autre clé | Désinstallez d'abord l'application installée |
| `downgrade` | Une version plus récente de l'application est installée | Désinstallez d'abord l'application installée |
| `invalid-apk` | Le fichier n'est pas un APK signé qu'Android peut lire | Récupérez de nouveau l'APK |
| `insufficient-storage` | Android n'a pas de place pour l'application | Libérez de l'espace dans Android |
| `blocked` | Une règle de cet Android n'autorise pas l'application | |
| `other` | Tout autre cas ; le message d'Android lui-même suit | |

Un APK peut faire jusqu'à 2 Gio. Les installations se font une à la fois : une deuxième
attend la fin de la première.

## Désinstaller une application

Utilisez l'une de ces méthodes :

- **Paramètres** de Windows > **Applications** > **Applications installées**.
- La page **Apps** (applications) de DSA Settings : **Uninstall** (désinstaller) à côté de
  l'application.
- `dsa apps uninstall <package>`.

Les applications qui font partie d'Android ne peuvent pas être désinstallées.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
