# Mémoire et comportement au repos

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · **Français** · [Deutsch](../de/memory.md) · [Português (Brasil)](../pt-BR/memory.md) · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

Desktop Subsystem for Android™ (DSA) rend la mémoire à votre PC pendant qu'Android tourne, et
pas seulement quand il s'arrête. Il n'y a rien à configurer pour cela. Cette page explique ce
qui se passe et comment l'observer.

## Voir combien de mémoire Android utilise

Ouvrez la page **Memory** (mémoire) de DSA Settings, ou exécutez `dsa memory`. Les deux
affichent :

- **On this PC** (sur ce PC) : ce que la machine virtuelle d'Android, le gestionnaire de
  fenêtres et chaque fenêtre d'application occupent.
- **Inside Android** (dans Android) : la mémoire utilisée, le cache, la mémoire compressée et
  les plus gros processus. Android communique ces informations quelques minutes après son
  démarrage.

## Doze, l'état de repos

Quand toutes les fenêtres Android sont fermées ou réduites depuis 30 secondes, l'écran
d'Android s'éteint et Android passe en Doze, comme un téléphone dans une poche.

- Les notifications, les alarmes et les widgets continuent de fonctionner en Doze. Le
  processeur se repose entre-temps.
- `dsa status` indique `Dozing`. La page **General** (général) de DSA Settings indique
  **Dozing (idle)**.
- Afficher n'importe quelle fenêtre Android réveille Android immédiatement.

Android suit aussi la mise en veille de votre PC. Il enregistre son état avant la mise en
veille du PC et règle son horloge à la sortie de veille.

## Gel (désactivé par défaut)

Le gel arrête tous les processus d'Android. Android n'utilise alors plus de processeur, mais
les notifications, les alarmes et les widgets s'arrêtent aussi. C'est pourquoi DSA ne gèle
jamais Android sauf si vous le demandez.

- Pour geler maintenant : **Freeze** sous **Freeze Android now** sur la page **General**, ou
  `dsa freeze`.
- Pour geler après 30 minutes en Doze : activez **Freeze Android when idle** sur la page
  **Advanced** (avancé), ou `dsa settings set freeze on`.
- Ouvrir une fenêtre Android, ou une saisie dans l'une d'elles, relance Android.

## Comment la mémoire est rendue

- La machine virtuelle d'Android ne prend de la mémoire à votre PC que lorsqu'Android
  l'utilise pour la première fois.
- La mémoire qu'Android libère est rendue à votre PC pendant qu'Android tourne.
- Quand la dernière fenêtre d'une application se ferme, Android met fin aux processus de
  cette application, donc leur mémoire revient en quelques secondes. Une application qui a un
  widget sur le bureau, ou qui travaille encore en arrière-plan (comme un lecteur de musique),
  continue de tourner.
- Une application qui garde un service en arrière-plan actif sans fenêtre, sans
  notification, sans son et sans widget est arrêtée au bout de cinq minutes. Si Android la
  relance encore et encore, DSA attend plus longtemps à chaque fois, jusqu'à une heure.
- Tant qu'aucune fenêtre d'application n'est ouverte, Android vide ses caches de fichiers et
  déplace la mémoire inutilisée vers la mémoire compressée, pour qu'une plus grande partie
  puisse être rendue.
- Le gestionnaire de fenêtres rend la mémoire qu'il n'utilise pas 30 secondes après la
  fermeture de la dernière fenêtre d'application.

## Limite de mémoire

La limite est **Automatic** (automatique) par défaut, ce qui ne fixe aucune limite. Android
démarre à 2 Go et grandit pendant qu'il tourne, jusqu'à la mémoire de votre PC moins ce que
Windows garde (10 % de la mémoire ou 2 Go, la plus grande des deux valeurs). Android ne
conserve que ce qu'il utilise.
Pour fixer une limite, utilisez **Memory limit** (limite de mémoire) sur la page **Memory**,
ou :

```
dsa settings set memory-limit 4g     # 4 Gio
dsa settings set memory-limit auto   # revient au mode automatique
```

Une nouvelle limite s'applique au prochain démarrage d'Android.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
