# Configurações

[English](../settings.md) · [한국어](../ko/settings.md) · [日本語](../ja/settings.md) · [简体中文](../zh-CN/settings.md) · [繁體中文](../zh-TW/settings.md) · [Español](../es/settings.md) · [Français](../fr/settings.md) · [Deutsch](../de/settings.md) · **Português (Brasil)** · [Русский](../ru/settings.md) · [Italiano](../it/settings.md) · [Tiếng Việt](../vi/settings.md) · [Bahasa Indonesia](../id/settings.md) · [Türkçe](../tr/settings.md)

**DSA Settings** é o app de configurações do Desktop Subsystem for Android™ (DSA) no Windows.
Abra-o pelo menu Iniciar. Ele aparece em inglês ou coreano, conforme o idioma de exibição do
Windows, e segue o tema do Windows. Ele funciona com o teclado e com leitores de tela.

Toda alteração que as DSA Settings fazem passa pelo comando `dsa`, então a linha de comando vê
os mesmos valores:

```
dsa settings                        # todas as configurações e seus valores
dsa settings set <name> <value>     # altera uma; as chaves aceitam on ou off
```

Algumas configurações valem para todos os usuários do PC, então só um administrador pode
alterá-las. As DSA Settings as marcam com um ícone de escudo e pedem a aprovação de um
administrador quando você altera uma delas. Abaixo, elas estão marcadas com "(admin)".

## Páginas

| Página | O que você encontra nela | Linha de comando |
|---|---|---|
| **General** (geral) | O estado do Android, **Start**, **Stop**, **Freeze Android now**, o último problema, **Open my log folder**, **Refresh** | `dsa status`, `dsa start`, `dsa stop`, `dsa freeze` |
| **Memory** (memória) | **Memory limit**, o tamanho da VM do Android, o que cada parte usa | `memory-limit`, `dsa memory` |
| **Apps** | Apps instalados: **Open**, **App info**, **Uninstall**, **Create desktop shortcut**. Para cada app: **Resizable**, o tamanho em que as janelas abrem, **Scaling on another monitor**, **Full screen**, **Bars hidden**, **Run as ARM**, **Root access**. Uma linha para o acesso root do **adb shell** | `dsa apps`, `dsa app-window`, `dsa apps run-as-arm`, `dsa apps root` |
| **Running in the background** (em segundo plano) | Apps rodando sem janela: **Open**, **Stop**; se deve perguntar antes de parar | `dsa background`, `stop-without-asking` |
| **Widgets** | Os widgets na sua área de trabalho e **Add to desktop** | `dsa widgets` |
| **Display & GPU** (tela e GPU) | **Render GPU**, **GPU of the primary monitor**, **App full screen**, **Apps that hide their bars**, **Turn off Windows touch gestures in DSA app windows** (só no Windows 11, ativado por padrão), **Restart the window manager** | `render-gpu`, `dsa gpus`, `app-fullscreen`, `app-immersive`, `touch-gestures-off` |
| **Device identity** (identidade do dispositivo) | Primeiro este computador, só leitura: o nome, o fabricante e o modelo, e **Device specifications** com **Copy**. Depois, o fabricante, o modelo e a impressão digital do build que o Android informa aos apps | `dsa settings device`, `device.<field>` |
| **Network** (rede) | Como o Android acessa a rede, o endereço dele, quem responde aos nomes dele (DNS) e o gateway, **Advanced networking**, **Use this PC's added root certificates**, **Developer mode** (adb), **Connect adb automatically** | `dsa network`, `advanced-networking`, `host-certificates`, `developer-mode`, `adb-port`, `adb-auto-connect` |
| **Updates** (atualizações) | **Android**: a versão e o build em execução, o nível do patch de segurança, a versão que ele acompanha, a atualização do Android com o progresso e **Check now** (admin), **Restart Android into updates by itself**, **Update the Android image automatically** (admin), **Version to use** (admin). **DSA**: **Update DSA automatically** (admin), **Get pre-release versions** (admin), **Install the newest version of DSA now** | `dsa android version`, `dsa android use`, `image-auto-restart`, `image-updates`, `program-updates`, `prerelease-updates`, `dsa update` |
| **Advanced** (avançado) | Área de transferência, arquivos, links da web, congelar quando ocioso, backup, redefinição, apps ARM, Google Play, conteúdo protegido, acesso root, rastreamentos (admin), pastas de log | veja abaixo |
| **Logs** | O log ao vivo do Android, **Save diagnostics** | `dsa logcat`, `dsa logs collect` |
| **About** (sobre) | Versões, licenças, código-fonte | |

## A página Advanced

| Configuração | Padrão | Linha de comando |
|---|---|---|
| **Share the clipboard with Android** | Ativado | `clipboard` |
| **Share Windows folders with Android** | Ativado | `file-sharing` |
| **Shared folders** | Todas as seis | `shared-folders` |
| **Android files** | | `dsa files open` |
| **Send files to Android apps** | | `dsa open`, `dsa share` |
| **Freeze Android when idle** | Desativado | `freeze` |
| **Open web links in Android apps** | Desativado | `links-in-android` |
| **Back up apps' data to this PC**, **Back up now**, **Restore** | Desativado | `dsa backup on`, `dsa backup now`, `dsa backup restore` |
| **Recovery key**, **Use a recovery key** | | `dsa backup key show`, `dsa backup key import <key>` |
| **Protection of Android's keys** | | |
| **Reset Android's data** | | `dsa reset-data` |
| **Run apps built only for ARM** | | `arm-translation` |
| **Google's ARM translator** (não em ARM64) | | `dsa arm-translation fetch google` |
| **Use the default translator** (não em ARM64) | | `dsa arm-translation use-default` |
| **Remove the ARM translation package** | | `dsa arm-translation remove` |
| **Google Play services and Play Store**, **Remove Google Play** | Não instalado | `dsa gapps install`, `dsa gapps remove` |
| **Widevine** (em **Protected content**) | Não instalado | `dsa drm widevine install`, `dsa drm widevine remove` |
| **Let Android apps ask for root** (admin) | Desativado | `root-elevation` |
| **Input trace** (admin) | Desativado | `input-trace` |
| **Resize trace** (admin) | Desativado | `resize-trace` |

O backup mantém uma cópia criptografada dos dados dos seus apps na sua pasta de usuário, com
uma chave que o Windows guarda para você. Para restaurá-lo em outro PC, ou depois de reinstalar
o Windows, você precisa da chave de recuperação, então anote-a e guarde-a longe do PC. O Google
Play, o Widevine e o acesso root estão descritos em [Apps e janelas](apps.md).

## Quando uma alteração passa a valer

- **Render GPU**: quando o gerenciador de janelas inicia de novo, na próxima vez que você entrar
  na sua conta ou quando você selecionar **Restart the window manager**.
- **Memory limit**, **Device identity**, **Advanced networking**, **Run apps built only for
  ARM**, Google Play, Widevine, a ativação do acesso root e a versão do Android: na próxima
  inicialização do Android. Para reiniciar o Android, pare-o na página **General** e, em
  seguida, abra um app.
- As configurações de janela de um app: a partir da próxima janela que o app abrir.
- Todo o resto, incluindo a desativação do acesso root: na hora.

## Como o Android encontra os nomes

O seu computador responde a todos os nomes que os apps do Android consultam, com o mesmo
resolvedor que os apps do Windows usam (no Linux e no macOS, o resolvedor do sistema). Assim, o
seu arquivo hosts, as regras de nomes da sua empresa, os servidores DNS próprios de uma VPN e um
filtro de DNS valem para os apps do Android como valem para o computador. O Android não consulta
um servidor DNS por conta própria.

- Isso vale também com **Advanced networking** ativado: o seu computador está na mesma rede e
  conhece os nomes dela, e uma VPN ou um filtro de DNS nele continua valendo.
- O **Private DNS** (DNS privado) do próprio Android vem desativado por padrão, porque ele
  consultaria um servidor DNS contornando o seu computador. Se você escolher um servidor de DNS
  privado nas configurações do Android, o Android o usa.
- Se o DSA não conseguir responder por um tempo, o Android consulta os servidores DNS da rede
  até que o DSA consiga de novo.
- A linha **DNS** da página **Network**, e `dsa network`, informam quem responde agora: o
  resolvedor do seu computador, ou os servidores DNS da rede, que eles listam.

## Valores que a linha de comando aceita

- `render-gpu`: `display` (a GPU do monitor principal, o padrão), `integrated`, `discrete`, ou
  parte do nome de uma GPU como `dsa gpus` a lista.
- `memory-limit`: `auto` (o padrão: sem limite fixo, até a memória do PC menos o que o Windows
  reserva), ou um tamanho como `4g` ou `3072` (MiB).
- `shared-folders`: `all`, `none`, ou nomes separados por vírgulas, como `Documents,Pictures`.
- `adb-port`: uma porta TCP de 1 a 65535. O padrão é 58526.
- `app-fullscreen`: `screen` (o padrão) ou `window`, para um pedido claro de tela cheia de um
  app. `app-immersive`: `screen` ou `window` (o padrão), para um app que só esconde as barras.
  Veja [Tela cheia](apps.md#tela-cheia).
- `touch-gestures-off`: `on` (o padrão) desativa os gestos de toque com três e quatro dedos do
  Windows 11 enquanto uma janela de app do DSA está em primeiro plano, para que os apps recebam
  até dez dedos, e restaura a sua própria configuração quando outra janela vem para o primeiro
  plano. Se o Windows recusar, a configuração não pode ser usada, e o app de configurações
  explica por quê; você ainda pode desativar os gestos para todos os apps nas Configurações do
  Windows, em Bluetooth e dispositivos, Toque.
- `host-certificates`: `on` (o padrão) permite que os apps Android confiem em certificados raiz
  adicionados a este PC, como os da sua empresa ou de um programa de segurança.
- `adb-auto-connect`: `on` (o padrão) mostra o Android em `adb devices` e no Android Studio
  enquanto o modo de desenvolvedor está ativado, sem `adb connect`.
- `image-auto-restart`: `on` (o padrão) reinicia o Android na hora em uma atualização de imagem
  baixada quando não há nenhuma janela de app aberta. Caso contrário, o DSA pergunta antes, ou
  espera até você fechá-las.
- `prerelease-updates`: `on` recebe pré-lançamentos além das versões estáveis, `off` recebe só
  versões estáveis. O valor inicial vem do instalador que você usou: um instalador de
  pré-lançamento o ativa, um estável o deixa desativado, e as atualizações mantêm a sua escolha.
  Todas as versões anteriores à 1.0 são pré-lançamentos. Desativá-lo nunca leva o DSA de volta a
  uma versão mais antiga; o DSA espera por uma versão estável mais nova do que a instalada.
- `root-elevation`: `on` permite que os apps que você autorizar na página **Apps** peçam root.
  O nível de cada app é definido com `dsa apps root <package> off|ask|remember`.
- `device.<field>`: `manufacturer`, `brand`, `model`, `device`, `name`, `characteristics`,
  `fingerprint`, ou uma parte da impressão digital, como `fingerprint.release`.
  `dsa settings reset device` restaura todos os valores.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
