# Fichiers

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · **Français** · [Deutsch](../de/files.md) · [Português (Brasil)](../pt-BR/files.md) · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Desktop Subsystem for Android™ (DSA) vous permet d'ouvrir les fichiers d'Android dans
l'Explorateur de fichiers, de partager vos dossiers Windows avec les applications Android et
d'envoyer un fichier à une application Android. Cette page décrit DSA sous Windows.

## Ouvrir les fichiers Android dans l'Explorateur de fichiers

Utilisez l'une de ces méthodes. Chacune démarre Android s'il ne tourne pas.

- **Android** dans le volet de navigation de l'Explorateur de fichiers, juste sous **Linux**
  de WSL.
- **Android files** (fichiers Android) sur la page **Advanced** (avancé) de DSA Settings.
- `dsa files open`. `dsa files` indique où se trouve le stockage, sans l'ouvrir.

Vous voyez le stockage partagé d'Android : les fichiers que les applications Android ont
enregistrés, comme les photos et les téléchargements. Vous pouvez y copier des fichiers et en
extraire, comme dans n'importe quel dossier. L'Explorateur de fichiers accède au stockage via le
client WebDAV de Windows. Si le stockage ne répond pas, DSA affiche une erreur avec la raison.

## Partager vos dossiers Windows avec les applications Android

Vos dossiers Documents, Téléchargements, Images, Musique, Vidéos et Bureau apparaissent dans
le stockage partagé d'Android sous `Host`, par exemple `Host/Pictures`. Les applications
Android peuvent y ouvrir et y enregistrer des fichiers. Une application a d'abord besoin de
l'autorisation de stockage d'Android, et Windows ne lui permet que ce que votre propre compte
peut faire.

Le partage est activé par défaut. Pour le modifier, ouvrez la page **Advanced** de DSA
Settings :

- Désactivez **Share Windows folders with Android** pour ne partager aucun dossier.
- Sous **Shared folders** (dossiers partagés), décochez un dossier pour arrêter de le
  partager. Il quitte Android immédiatement.

Depuis la ligne de commande :

```
dsa settings set shared-folders Documents,Pictures   # uniquement ceux-ci ; aussi all ou none
dsa settings set file-sharing off                     # ne partager aucun dossier
```

Les versions précédentes utilisaient un dossier nommé `Windows` au lieu de `Host`. Android
supprime l'ancien dossier une fois qu'il est vide, donc les fichiers que vous y avez
enregistrés restent.

## Envoyer un fichier à une application Android

- **Glisser-déposer.** Faites glisser un fichier depuis l'Explorateur de fichiers sur la
  fenêtre d'une application. Le fichier va à cette application. Un APK va plutôt à l'APK
  Installer. Les dossiers ne sont pas envoyés.
- **Menu contextuel.** Faites un clic droit sur un fichier et sélectionnez **Open in an
  Android app** (ouvrir dans une application Android) ou **Share with an Android app**
  (partager avec une application Android). Sous Windows 11, ils se trouvent sous **Afficher
  d'autres options**. Android demande ensuite quelle application reçoit le fichier.
- **Ligne de commande.** `dsa open <file>` et `dsa share <file>`.

Ces méthodes envoient des fichiers jusqu'à 64 Mio. Un fichier plus volumineux est refusé
avant tout envoi. Pour donner un fichier plus volumineux à une application, placez-le plutôt
dans un dossier partagé.

## Récupérer un fichier depuis Android

- Enregistrez-le dans Android sous `Host/<folder>`. Il se trouve alors dans ce dossier sur
  votre PC.
- Ou ouvrez **Android** dans l'Explorateur de fichiers et copiez le fichier à l'extérieur.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
