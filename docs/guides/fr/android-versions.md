# Versions d'Android

[English](../android-versions.md) · [한국어](../ko/android-versions.md) · [日本語](../ja/android-versions.md) · [简体中文](../zh-CN/android-versions.md) · [繁體中文](../zh-TW/android-versions.md) · [Español](../es/android-versions.md) · **Français** · [Deutsch](../de/android-versions.md) · [Português (Brasil)](../pt-BR/android-versions.md) · [Русский](../ru/android-versions.md) · [Italiano](../it/android-versions.md) · [Tiếng Việt](../vi/android-versions.md) · [Bahasa Indonesia](../id/android-versions.md) · [Türkçe](../tr/android-versions.md)

Desktop Subsystem for Android™ (DSA) prend en charge les deux versions majeures d'Android les
plus récentes : aujourd'hui Android 16 et Android 17. Par défaut, il exécute la plus récente.
Certaines applications ne fonctionnent que sur l'une des deux, vous pouvez donc choisir.

## Voir quelle version tourne

Ouvrez la page **Updates** (mises à jour) de DSA Settings. Sous **Android**, elle affiche la
version, son build, son niveau de correctif de sécurité, la version qu'elle suit et l'état de
sa mise à jour, avec la progression du téléchargement. **Check now** (vérifier maintenant)
demande au service de chercher une mise à jour immédiatement. Ou exécutez :

```
dsa android version   # la version qui tourne, et celles que vous pouvez choisir
```

## Changer de version

Tous les utilisateurs du PC exécutent la même version d'Android, donc seul un administrateur
peut la changer.

1. Ouvrez la page **Updates** de DSA Settings.
2. Sous **Version to use** (version à utiliser), choisissez une version, ou **Always the
   newest** (toujours la plus récente).
3. Sélectionnez **Change** (changer), puis approuvez l'invite d'administrateur.

La version choisie est téléchargée et utilisée à partir du prochain démarrage d'Android.
Depuis un terminal administrateur :

```
dsa android use 16
dsa android use latest
```

### Passer à une version plus ancienne efface les données

Android ne peut pas lire les données écrites par une version plus récente. Passer à une
version majeure plus ancienne efface donc les données Android de tous les utilisateurs du
PC : les applications qu'ils ont installées, leurs comptes et toutes les données des
applications. Les réglages de DSA sont conservés. Cette opération est irréversible.

DSA Settings vous demande de confirmer avec **Erase data and change** (effacer les données et
changer). En ligne de commande, ajoutez `--erase-data` :

```
dsa android use 16 --erase-data
```

Passer à une version plus récente conserve vos applications et vos données.

## Mises à jour

- La version choisie continue de recevoir les mises à jour de sécurité.
- Quand une version n'est plus prise en charge, DSA passe à la suivante et conserve vos
  applications.
- DSA ne passe jamais de lui-même à une version majeure plus ancienne.
- Chaque image est vérifiée par rapport à une liste signée avant d'être utilisée. Une image
  qui ne parvient pas à démarrer est remplacée par la précédente.

Un administrateur peut désactiver les mises à jour de l'image avec **Update the Android image
automatically** sur la page **Updates** de DSA Settings, ou avec
`dsa settings set image-updates off`.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
