# Mode développeur et adb

[English](../developer-mode.md) · [한국어](../ko/developer-mode.md) · [日本語](../ja/developer-mode.md) · [简体中文](../zh-CN/developer-mode.md) · [繁體中文](../zh-TW/developer-mode.md) · [Español](../es/developer-mode.md) · **Français** · [Deutsch](../de/developer-mode.md) · [Português (Brasil)](../pt-BR/developer-mode.md) · [Русский](../ru/developer-mode.md) · [Italiano](../it/developer-mode.md) · [Tiếng Việt](../vi/developer-mode.md) · [Bahasa Indonesia](../id/developer-mode.md) · [Türkçe](../tr/developer-mode.md)

Le mode développeur permet à l'Android Debug Bridge (adb) de votre PC de se connecter à votre
Android dans Desktop Subsystem for Android™ (DSA). Il est désactivé par défaut.

adb ne fait pas partie de DSA. Obtenez-le avec les Android SDK Platform-Tools.

## Connecter adb

1. Ouvrez la page **Network** (réseau) de DSA Settings et activez **Developer mode** (mode
   développeur). Ou exécutez `dsa settings set developer-mode on`.
2. Démarrez Android, s'il ne tourne pas : `dsa start --wait`.
3. Connectez-vous :

   ```
   adb connect 127.0.0.1:58526
   adb devices
   ```

   Android ne vous demande pas d'autoriser la connexion. Le port ne répond que sur ce PC, donc
   les autres ordinateurs ne peuvent pas se connecter.
4. Quand vous avez terminé, désactivez de nouveau **Developer mode**, ou exécutez
   `dsa settings set developer-mode off`.

**Connect adb** (connecter adb) sur la page **Network** affiche la commande exacte, et
`dsa settings` l'affiche sur la ligne `Developer mode`.

## Utiliser un autre port

Le port par défaut est 58526. Pour le changer, modifiez **Port** sous **Connect adb**, ou
exécutez :

```
dsa settings set adb-port 5555
```

## Ce que le mode développeur ne change pas

DSA n'utilise jamais adb pour son propre fonctionnement. Les fenêtres, la saisie et les
installations passent par le canal de contrôle de DSA, donc tout fonctionne de la même façon
avec le mode développeur désactivé.

Pour lire le journal d'Android, vous n'avez pas besoin d'adb. Voir
[Dépannage](troubleshooting.md).

---

Android est une marque de Google LLC. DSA n'est ni affilié à Google, ni approuvé ou parrainé
par Google.
