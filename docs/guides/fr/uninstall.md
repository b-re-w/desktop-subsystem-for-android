# Désinstallation

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · **Français** · [Deutsch](../de/uninstall.md) · [Português (Brasil)](../pt-BR/uninstall.md) · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Désinstallez Desktop Subsystem for Android™ (DSA) de l'une de ces façons :

- **Paramètres** de Windows > **Applications** > **Applications installées** : trouvez
  Desktop Subsystem for Android, puis sélectionnez **Désinstaller**.
- Relancez le programme d'installation et sélectionnez **Uninstall** (désinstaller).

La désinstallation supprime les éléments suivants, pour tous les utilisateurs du PC,
connectés ou non :

- le programme, le service `DsaHost`, la commande `dsa`, DSA Settings et l'APK Installer ;
- les images Android et les mises à jour téléchargées ;
- les entrées du menu Démarrer des applications Android et leurs entrées dans
  **Applications installées** ;
- **Android** dans l'Explorateur de fichiers, l'association des fichiers `.apk` et les entrées
  du menu contextuel ;
- les réglages et les journaux de DSA.

Elle **conserve** le disque Android de chaque utilisateur, `%LOCALAPPDATA%\DSA\android.vhdx`,
pour qu'une installation ultérieure retrouve vos applications et vos données. Pour supprimer
aussi vos données, exécutez `dsa reset-data --yes` avant de désinstaller, ou supprimez ce
fichier ensuite.

Si la redirection de dossiers a déplacé le menu Démarrer d'un utilisateur, les entrées des
applications Android de cet utilisateur restent, car le programme de désinstallation ne
regarde qu'à l'emplacement par défaut. Supprimez-les à la main.

Votre configuration WSL n'est pas touchée, car DSA ne la modifie jamais.

## Linux (pas encore publié)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Supprimer le paquet supprime aussi l'entrée du dépôt de paquets qu'il a ajoutée.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
