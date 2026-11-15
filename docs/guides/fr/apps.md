# Applications et fenêtres

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · **Français** · [Deutsch](../de/apps.md) · [Português (Brasil)](../pt-BR/apps.md) · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Chaque tâche d'application Android s'ouvre dans sa propre fenêtre de bureau, avec son propre
bouton dans la barre des tâches. Il n'y a pas de cadre d'émulateur autour d'un écran de
téléphone. Cette page décrit Desktop Subsystem for Android™ (DSA) sous Windows.

## Ouvrir une application

- **Menu Démarrer.** Chaque application installée a une entrée dans le menu Démarrer avec son
  propre nom et sa propre icône, donc la recherche Windows la trouve. L'ouvrir démarre
  d'abord Android s'il ne tourne pas.
- **DSA Settings.** Sur la page **Apps** (applications), sélectionnez **Open** (ouvrir) à
  côté de l'application.
- **Ligne de commande.** `dsa apps list` liste les applications installées avec leurs
  paquets. `dsa launch <package>` en ouvre une, par exemple `dsa launch com.android.settings`.

Quand l'entrée du menu Démarrer d'un autre programme porte déjà le nom de l'application, DSA
ajoute le paquet au nom, par exemple `Clock (com.example.clock)`. DSA ne modifie ou ne
supprime que les entrées qu'il a créées.

## Raccourcis sur le bureau et listes de raccourcis

Les icônes des applications fonctionnent comme celles des applications Windows, et non comme
celles d'un écran d'accueil Android.

- **Raccourci sur le bureau.** Sur la page **Apps** de DSA Settings, sélectionnez **Create
  desktop shortcut** (créer un raccourci sur le bureau) à côté de l'application. Vous pouvez
  aussi faire un clic droit sur le bouton de l'application dans la barre des tâches et
  sélectionner **Create desktop shortcut**. Depuis la ligne de commande :
  `dsa apps desktop-shortcut <package>`. Le raccourci ouvre l'application comme son entrée du
  menu Démarrer.
- **Raccourcis d'application.** Les raccourcis qu'une application Android propose, comme une
  nouvelle conversation dans une messagerie, s'affichent quand vous faites un clic droit sur
  le bouton de l'application dans la barre des tâches ou sur son entrée du menu Démarrer. En
  sélectionner un ouvre cet écran de l'application dans une fenêtre, en démarrant d'abord
  Android s'il ne tourne pas. Quand l'application change ses raccourcis, la liste suit.
  Depuis la ligne de commande : `dsa apps shortcut <package> <id>` ;
  `dsa apps info <package> --json` liste les identifiants.
- **Raccourcis demandés par une application.** Quand une application demande à ajouter l'un
  de ses raccourcis à votre bureau, DSA vous demande d'abord votre accord et ne l'ajoute que
  si vous acceptez.

Quand une application change de nom, par exemple après une mise à jour, son entrée du menu
Démarrer, ses raccourcis sur le bureau et son épinglage dans la barre des tâches suivent le
nouveau nom et continuent de fonctionner. Un raccourci sur le bureau que vous avez supprimé
n'est pas recréé. Désinstaller une application supprime aussi ses raccourcis sur le bureau.

## Utiliser les fenêtres des applications

- Chaque tâche a sa propre fenêtre, son propre bouton dans la barre des tâches et sa propre
  entrée dans Alt+Tab, avec le nom et l'icône de l'application. Le Gestionnaire des tâches
  liste chaque application sous son propre nom.
- La barre de titre fait partie de l'application : les couleurs de l'application montent
  jusqu'en haut de la fenêtre, et les boutons de légende de Windows 11 se placent par-dessus.
  L'icône et le nom de l'application sont à gauche. Quand l'application peut revenir en
  arrière, l'icône devient une flèche de retour ; sélectionnez-la pour revenir en arrière.
- Les dispositions d'ancrage (survolez le bouton Agrandir), le déplacement par la barre de
  titre, le double-clic sur celle-ci pour agrandir et le redimensionnement en direct
  fonctionnent. Le redimensionnement ne redémarre pas l'application. Une fenêtre agrandie
  remplit la zone de travail de l'écran et laisse la barre des tâches visible.
- Il n'y a pas de barre de navigation Android. La barre inférieure d'une application se place
  au bord inférieur de la fenêtre, sans bande vide en dessous.
- La souris (avec survol), le tactile, le stylet (pression, gomme et bouton latéral) et le
  clavier fonctionnent tous. Le texte vient de votre méthode de saisie Windows, y compris pour
  le coréen et d'autres langues ; la composition s'affiche directement dans l'éditeur
  Android, et la fenêtre des candidats se place à côté du curseur. Il n'y a aucune
  application de clavier Android.
- Faites glisser des fichiers depuis l'Explorateur de fichiers sur la fenêtre d'une
  application, ou faites glisser des fichiers hors d'une application vers l'Explorateur de
  fichiers.
- Les lecteurs d'écran comme le Narrateur (Narrator) et NVDA lisent l'application via UI
  Automation.
- Fermer une fenêtre demande à Android de mettre fin à cette tâche.

## Le menu de la fenêtre

Faites un clic droit sur la barre de titre, sélectionnez l'icône de l'application ou appuyez
sur Alt+Espace. Après les éléments habituels de Windows, le menu contient :

- **Full screen** (plein écran) : remplit tout l'écran, comme F11. Sélectionnez-le à nouveau,
  ou appuyez sur F11, pour en sortir.
- **New window** (nouvelle fenêtre) : ouvre une autre fenêtre de l'application, pour les
  applications qui en prennent plusieurs en charge.
- **Screenshot** (capture d'écran) : copie l'écran de l'application, sans la barre de titre,
  dans le presse-papiers et l'enregistre dans le dossier Captures d'écran de vos Images.
- **Aspect ratio** (format d'image) : ouvre le réglage Android du format d'image de
  l'application, pour les applications qui paraissent étirées dans une grande fenêtre.
- **Open by default** (ouvrir par défaut) : choisit si les liens que l'application prend en
  charge s'ouvrent dans l'application ou dans votre navigateur.
- **App info** (infos sur l'application) : ouvre l'entrée de l'application sur la page
  **Apps** de DSA Settings.

## Plein écran

Quand une application demande le plein écran, sa fenêtre l'affiche de l'une de ces deux
façons :

- **Sur tout l'écran** : la fenêtre remplit l'écran, et la barre de titre se masque. Touchez
  le bord supérieur de l'écran avec le pointeur pour l'afficher, avec Réduire et Fermer ; le
  bouton du milieu quitte le plein écran.
- **Dans la fenêtre** : la fenêtre garde sa taille et sa barre de titre, et l'application la
  remplit.

Les applications le demandent de deux manières, et chacune a son propre comportement par
défaut :

| Type de demande | Exemple | Par défaut |
|---|---|---|
| Une demande explicite de plein écran | Le bouton plein écran d'un lecteur vidéo | Sur tout l'écran |
| Seulement masquer les barres | Un flux de vidéos courtes | Dans la fenêtre |

Changez les valeurs par défaut sous **App full screen** et **Apps that hide their bars** sur
la page **Display & GPU** (affichage et GPU) de DSA Settings, ou pour une seule application
sous **Full screen request** et **When only the bars hide** sur la page **Apps**. Le choix propre à une
application passe en premier. F11 et **Full screen** dans le menu de la fenêtre basculent à
tout moment le plein écran sur tout l'écran, quel que soit le réglage.

Depuis la ligne de commande :

```
dsa settings set app-fullscreen screen|window           # valeur par défaut pour les demandes explicites
dsa settings set app-immersive screen|window            # valeur par défaut pour les barres masquées
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Définir la taille de la fenêtre d'une application

Sur la page **Apps** de DSA Settings :

- Désactivez **Resizable** (redimensionnable) pour garder les fenêtres de l'application à la
  taille à laquelle elles s'ouvrent.
- Choisissez la taille à laquelle les fenêtres de l'application s'ouvrent : la taille
  choisie par Android, ou une taille de téléphone ou de tablette en portrait ou en paysage.

Le changement s'applique à la prochaine fenêtre que l'application ouvre. Depuis la ligne de
commande :

```
dsa app-window <package> --fixed-size on        # ne peut être ni redimensionnée ni agrandie
dsa app-window <package> --start-size 1280x800  # s'ouvre à cette taille ; auto laisse Android choisir
dsa app-window <package> --dpi-scaling system   # mise à l'échelle sur un autre écran ; voir plus bas
dsa app-window <package> reset                  # revient aux valeurs par défaut
```

## Faire démarrer une application sous la barre de titre

Une application dessine jusqu'au bord supérieur de sa fenêtre, sous la barre de titre.
Certaines applications écrasent alors leur barre supérieure ou en masquent une partie sous la
barre de titre. Pour une telle application, activez **Start below the title bar** (démarrer sous la barre de
titre) sur la page **Apps** de DSA Settings. Le contenu de l'application démarre alors sous la barre de
titre, et la barre de titre prend la couleur que l'application affiche à cet endroit. C'est
désactivé par défaut, et cela s'applique à la prochaine ouverture de la fenêtre de
l'application. En ligne de commande :

```
dsa app-window <package> --below-title-bar on   # off revient à la valeur par défaut
```

## Déplacer une application vers un écran d'une autre échelle

Android démarre à l'échelle de votre écran principal, donc les applications s'y ouvrent à la
bonne taille. Quand vous déplacez une fenêtre
vers un écran d'une autre échelle, l'application garde sa taille à l'écran, et avec **Automatic** et **System** elle n'est jamais redémarrée. Chaque
application le fait de l'une des manières suivantes, que vous choisissez sous **Scaling** (mise à l'échelle sur un autre écran) sur la page **Apps**, ou avec
`dsa app-window <package> --dpi-scaling` :

- **Automatic** (`auto`, la valeur par défaut) : **System** pour les applications de la liste
  intégrée de DSA, celles qui se dérèglent quand leur échelle change en cours d'exécution.
  Pour toute autre application, **App** si elle déclare gérer elle-même un changement
  d'échelle, **System** sinon. La liste contient YouTube : il garde la disposition de
  l'échelle à laquelle il a démarré, donc après un déplacement ses onglets inférieurs et son
  mini-lecteur resteraient déréglés jusqu'à son redémarrage. DSA Settings affiche la valeur par
  défaut d'une telle application comme **Automatic (system, recommended for this app)**. Votre
  propre choix pour l'application passe en premier.
- **App** (`app`) : l'application se redessine à l'échelle du nouvel écran dès que la fenêtre
  y arrive, et son texte reste net. Pour une application qui ne déclare pas gérer elle-même un
  changement d'échelle, Android recrée son écran et restaure son état (position de défilement,
  texte saisi), donc elle peut se redessiner brièvement quand vous la déplacez, comme un
  programme Windows avec la mise à l'échelle « Application ».
- **System** (`system`) : l'application garde son échelle, et DSA met sa fenêtre à l'échelle
  du nouvel écran. Agrandi, le texte peut paraître un peu flou.
- **System (forced)** (`system-forced`) : l'application reçoit la nouvelle échelle sans
  redémarrage, même si elle ne déclare pas la gérer. Pas encore disponible : cela nécessite
  une modification d'Android que cette version n'a pas, donc d'ici là ce choix fonctionne
  comme **System**.

Le texte des applications suit la taille du texte de Windows (Paramètres > Accessibilité >
Taille du texte).

## Notifications

Les notifications Android s'affichent comme des notifications Windows, avec le nom et
l'icône de l'application.

- Sélectionnez une notification pour ouvrir l'application.
- Fermez-la, et elle est aussi fermée dans Android, sauf si l'application la garde affichée.
- Les notifications continuent d'arriver pendant qu'Android est en Doze. Voir
  [Mémoire](memory.md).

## Applications en arrière-plan

Une application qui continue de tourner après la fermeture de sa fenêtre, comme un lecteur de
musique, s'affiche sous forme d'icône dans la zone de notification. Son info-bulle est la
notification de l'application.

- Sélectionnez l'icône pour ouvrir l'application.
- Faites un clic droit dessus pour **Open**, les actions propres à la notification, et
  **Stop** (arrêter).
- **Stop** demande d'abord confirmation. Cochez **Do not ask again** (ne plus demander) pour
  ne plus voir la question. Pour qu'elle soit de nouveau posée, activez **Ask before stopping
  an app from the notification area** sur la page **Running in the background** (en
  arrière-plan) de DSA Settings.
- Une application qui lit du contenu multimédia fonctionne aussi avec les touches
  multimédias et les contrôles multimédias de Windows.

La page **Running in the background** de DSA Settings liste ces applications, avec **Open**
et **Stop** pour chacune. Depuis la ligne de commande : `dsa background`,
`dsa background open <package>` et `dsa background stop <package>`.

## Widgets sur le bureau

Un widget Android peut se placer sur votre bureau dans sa propre fenêtre. Il reste sous les
autres fenêtres, reste visible quand vous affichez le bureau, et n'apparaît ni dans la barre
des tâches ni dans Alt+Tab.

1. Ouvrez la page **Widgets** de DSA Settings.
2. Sous **Widgets your apps offer** (widgets proposés par vos applications), sélectionnez
   **Add to desktop** (ajouter au bureau) à côté d'un widget. Si l'application vous demande de
   configurer le widget, sa configuration s'ouvre dans une fenêtre.
3. Pour déplacer un widget, appuyez n'importe où dessus et restez immobile une demi-seconde.
   Il se soulève, avec une bordure et une poignée sur chaque bord le long duquel il peut être
   redimensionné. Faites-le glisser pour le déplacer, ou faites glisser une poignée pour le
   redimensionner. Un glissement rapide reste destiné au widget, donc ses listes défilent
   toujours. Vous pouvez aussi faire glisser un bord du widget pour le redimensionner sans le
   soulever d'abord ; seuls les bords le long desquels il peut être redimensionné le
   permettent. DSA retient où vous l'avez placé et sa taille.
4. Faites un clic droit sur le widget, ou relâchez après l'appui long, pour **Widget
   settings** (réglages du widget, quand le widget peut être reconfiguré), **Resize**
   (redimensionner), pour choisir l'une des tailles qu'il autorise, et **Remove** (retirer).
   La touche Menu et Maj+F10 l'ouvrent aussi. Les touches fléchées, Entrée et Échap
   fonctionnent dans ce menu, et une lettre choisit un élément : R pour **Resize** et M pour
   **Remove**.

Depuis la ligne de commande : `dsa widgets list`, `dsa widgets add <provider>` et
`dsa widgets remove <id>`.

## Liens et presse-papiers

- Un lien web sur lequel vous appuyez dans une application Android s'ouvre dans votre
  navigateur par défaut. Un lien qu'une application installée a vérifié comme lui
  appartenant s'ouvre dans cette application.
- Pour ouvrir plutôt tous les liens web dans les applications Android, activez **Open web
  links in Android apps** sur la page **Advanced** (avancé) de DSA Settings
  (`dsa settings set links-in-android on`).
- Le texte que vous copiez est partagé dans les deux sens entre Windows et Android. Pour
  l'arrêter, désactivez **Share the clipboard with Android** sur la page **Advanced**
  (`dsa settings set clipboard off`).

## Google Play et vidéo protégée

DSA ne fournit aucun logiciel Google. Si vous le souhaitez, DSA le télécharge pour vous depuis
sa source, après vous avoir montré la source et les conditions. Les deux s'appliquent à partir
du prochain démarrage d'Android, et les deux peuvent être retirés.

- **Google Play.** Sur la page **Advanced** de DSA Settings, sélectionnez **Install** sous
  **Google Play**, ou exécutez `dsa gapps install`. DSA télécharge le paquet public LiteGapps
  Core (services Google Play, Play Store et synchronisation du compte) pour votre version
  d'Android. Cet Android n'est pas certifié par Google, donc Google peut refuser de vous
  connecter tant que vous n'avez pas enregistré l'appareil : **Register this device with
  Google** affiche l'identifiant Android et ouvre la page d'enregistrement.
  `dsa gapps status` affiche l'état, et `dsa gapps remove` retire les applications de Google,
  même quand Android est arrêté.
- **Widevine.** Les applications de streaming ont besoin du module Widevine de Google pour
  lire les vidéos protégées. Sur la page **Advanced**, sélectionnez **Get** (obtenir) sous
  **Protected content**, ou exécutez `dsa drm widevine install`. DSA télécharge l'image de
  l'émulateur Android de Google pour votre version d'Android (environ 2 Go), ne garde que le
  module Widevine, sans modification, et supprime le reste. Ici, Widevine est au niveau de
  sécurité L3, donc les services de streaming envoient au mieux de la définition standard ;
  pour la haute définition, utilisez un navigateur ou une application Windows.
  `dsa drm widevine remove` le retire.

## Applications conçues uniquement pour ARM

Sur un PC x64, une application dont le code n'existe que pour les processeurs ARM passe par
un traducteur ARM. Android sur un PC x64 en inclut un, le traducteur open source Digitalis,
et l'utilise dès son premier démarrage. Vous pouvez en utiliser un autre à la place :

- **Le traducteur ARM de Google.** Sur la page **Advanced**, sélectionnez **Get** à côté de
  **Google's ARM translator**, ou exécutez `dsa arm-translation fetch google`. DSA montre
  d'abord d'où il vient et les conditions de Google (la
  [Android SDK License](https://developer.android.com/studio/terms)), et ne télécharge rien
  tant que vous n'avez pas accepté. Votre PC télécharge ensuite l'image de l'émulateur Android
  que Google propose pour votre version d'Android (environ 2 Go), ne garde que le traducteur
  et supprime le reste. DSA ne fournit ni ne redistribue le traducteur de Google.
- **Un paquet que vous compilez ou obtenez ailleurs**, comme un paquet fait à partir du
  traducteur open source Digitalis : `dsa arm-translation install <folder>`.

Redémarrez ensuite Android : arrêtez-le sur la page **General** (général) de DSA Settings,
puis ouvrez une application.

Pour revenir au traducteur fourni avec Android, sélectionnez **Use** à côté de **Use the
default translator** sur la page **Advanced**, ou exécutez
`dsa arm-translation use-default`. Cela retire le paquet que vous avez installé, à partir du
prochain démarrage d'Android.

`dsa arm-translation status` indique quel traducteur Android utilise. L'interrupteur **Run
apps built only for ARM** de la page **Advanced** active ou désactive la traduction sans rien
retirer. Pour une application qui a à la fois du code ARM et du code x64, **Run as ARM** sur
la page **Apps** (`dsa apps run-as-arm <package> on`) exécute plutôt son code ARM. Seules les
applications ARM 64 bits (`arm64-v8a`) peuvent fonctionner. Sur un PC ARM64, Android exécute
lui-même les applications ARM et n'a pas besoin de traducteur.

## Accès root

Certaines applications, comme les outils de sauvegarde, ont besoin de root. DSA le donne
comme `sudo` sur un PC Linux : uniquement quand un administrateur du PC approuve, une demande
à la fois. Il est désactivé par défaut, et les applications non autorisées ne voient pas du
tout `su`.

1. Un administrateur active **Let Android apps ask for root** (laisser les applications
   Android demander root) sous **Root access** sur la page **Advanced** de DSA Settings
   (`dsa settings set root-elevation on`). Le réglage s'applique à partir du prochain
   démarrage d'Android.
2. Sur la page **Apps**, réglez le paramètre **Root access** (accès root) de l'application :
   - **Root: not allowed** (la valeur par défaut) : l'application n'a pas de `su`.
   - **Root: ask each time** : chaque fois que l'application utilise `su`, Windows affiche son
     invite d'administrateur (Contrôle de compte d'utilisateur, UAC) sous le nom et l'icône de
     l'application. Un utilisateur standard saisit le mot de passe d'un administrateur. Une
     approbation dure 15 minutes pour cette application.
   - **Root: remember (no asking)** : l'application utilise `su` sans demander. Ce choix
     nécessite l'approbation d'un administrateur.

   Depuis la ligne de commande : `dsa apps root <package> off|ask|remember`.

Avec le mode développeur activé, `adb shell` peut aussi utiliser `su`. Il a sa propre ligne
sur la page **Apps**, **adb shell**, et demande à chaque fois par défaut
(`dsa apps root adb ask`). Son invite s'appelle **adb shell** et affiche la commande.

Quand c'est une partie d'Android elle-même qui demande, l'invite la nomme : **Android
system**, **Android phone services**, **Android Bluetooth**, **Android NFC** ou **Android
system service**. Les applications qui partagent un même identifiant utilisateur sont
approuvées en tant que groupe, et l'invite les liste toutes.

Chaque demande est conservée dans le journal du service avec l'application, l'heure, la
commande et le fait qu'elle a été approuvée ou non. Désactiver la fonctionnalité masque `su`
pour toutes les applications à la fois, sans redémarrage.

---

Android est une marque de Google LLC. Windows est une marque du groupe de sociétés Microsoft.
DSA n'est ni affilié à Google ou à Microsoft, ni approuvé ou parrainé par eux.
