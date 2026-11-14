# Dépannage et journaux

[English](../troubleshooting.md) · [한국어](../ko/troubleshooting.md) · [日本語](../ja/troubleshooting.md) · [简体中文](../zh-CN/troubleshooting.md) · [繁體中文](../zh-TW/troubleshooting.md) · [Español](../es/troubleshooting.md) · **Français** · [Deutsch](../de/troubleshooting.md) · [Português (Brasil)](../pt-BR/troubleshooting.md) · [Русский](../ru/troubleshooting.md) · [Italiano](../it/troubleshooting.md) · [Tiếng Việt](../vi/troubleshooting.md) · [Bahasa Indonesia](../id/troubleshooting.md) · [Türkçe](../tr/troubleshooting.md)

Cette page vous aide à comprendre pourquoi Desktop Subsystem for Android™ (DSA) ne fonctionne
pas, à lire le journal d'Android et à envoyer un signalement de problème. Elle couvre Windows,
et Linux là où c'est précisé.

## Vérifier d'abord l'état

```
dsa status          # l'état d'Android, tout problème et toute mise à jour en attente
dsa start --wait    # démarre Android et attend la fin de son démarrage
```

La page **General** (général) de DSA Settings affiche le même état.

| `dsa status` | DSA Settings | Signification |
|---|---|---|
| `Active` | **Running** | Android a démarré et l'agent de DSA qu'il contient est connecté |
| `Dozing` | **Dozing (idle)** | Android est au repos. Voir [Mémoire](memory.md) |
| `Frozen` | **Frozen** | Android est gelé jusqu'à ce que vous ouvriez l'une de ses fenêtres |
| `Stalled` | **Not ready** | Android tourne mais met trop de temps. La ligne suivante indique pourquoi et que faire |

## Problèmes courants

| Problème | Que faire |
|---|---|
| Le programme d'installation indique que la Plateforme de machine virtuelle est désactivée | Activez la fonctionnalité Windows, redémarrez et relancez le programme d'installation. Voir [Prise en main](getting-started.md) |
| Aucune fenêtre d'application ne s'ouvre | Fermez votre session puis rouvrez-la. Le gestionnaire de fenêtres démarre à l'ouverture de session |
| Une application s'ouvre à une mauvaise taille | `dsa app-window <package> reset` |
| Android ne démarre pas après une mise à jour de l'image | Rien. DSA revient de lui-même à l'image précédente, et `dsa status` le signale |
| Android n'a pas de réseau | Ouvrez la page **Network** (réseau) de DSA Settings, ou exécutez `dsa network`. Elle affiche le mode, l'adresse, les serveurs DNS et tout problème |
| Android ne fonctionne plus et rien d'autre n'aide | Réinitialisez les données d'Android (voir ci-dessous) |

### Réinitialiser les données d'Android

La réinitialisation efface toutes les applications que vous avez installées, vos comptes et
toutes les données des applications. Android démarre ensuite comme lors de sa première
installation. L'image Android et les réglages de DSA sont conservés.

- Dans DSA Settings : page **Advanced** (avancé) > **Reset Android's data** > **Reset**.
- En ligne de commande : `dsa reset-data` indique ce qui serait perdu.
  `dsa reset-data --yes` l'efface.

## Lire le journal d'Android

Le journal d'Android reste en mémoire dans Android et n'est jamais écrit dans un fichier. Il
ne parvient à votre PC que tant que quelque chose le lit.

- Dans DSA Settings : la page **Logs** (journaux). Vous pouvez filtrer par étiquette ou par
  texte, et mettre en pause.
- En ligne de commande :

  ```
  dsa logcat                        # suit le journal ; Ctrl+C l'arrête
  dsa logcat -d -b all *:W          # le contenu actuel du journal, avertissements et plus graves
  dsa logcat ActivityManager:I *:S  # une seule étiquette
  ```

## Envoyer un signalement de problème

1. Enregistrez les diagnostics. Sur la page **Logs**, sélectionnez **Save diagnostics**
   (enregistrer les diagnostics), ou exécutez :

   ```
   dsa logs collect -o diag.zip
   ```

   Cela prend une minute ou plus.
2. Joignez le fichier zip à un
   [ticket](https://github.com/thisisthepy/desktop-subsystem-for-android/issues).

L'archive contient tout le journal d'Android, ses rapports de plantage (tombstones, traces
ANR, dropbox, pstore), un résumé des services système d'Android, et les journaux de DSA sur ce
PC qui vous concernent, sans les lignes des autres utilisateurs. `summary.txt` dans l'archive
indique ce qui n'a pas pu être collecté.

## Emplacement des journaux sous Windows

| Dossier | Fichiers |
|---|---|
| `%LOCALAPPDATA%\DSA\logs\` | `dsa-wm.log` (le gestionnaire de fenêtres), `dsa-app-window.log` (les fenêtres des applications) |
| `%ProgramData%\DSA\logs\` | `dsa-hostd.log` (le service), `android-<SID>.log` (la console Android de chaque utilisateur, 2 Mio au plus par démarrage), `setup-<version>.log` (les mises à jour automatiques) |

Pour les ouvrir, utilisez **Open my log folder** et **Open the service's log folder
(administrators)** sur la page **Advanced** de DSA Settings.

## Emplacement des journaux sous Linux (pas encore publié)

Les services écrivent dans le journal systemd :

```
journalctl -u dsa-hostd
journalctl --user -u dsa-devices -u dsa-compositor
```

## Toutes les commandes

`dsa` sans argument affiche toutes les commandes. Ajoutez `--json` pour obtenir une sortie
lisible par des programmes.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
