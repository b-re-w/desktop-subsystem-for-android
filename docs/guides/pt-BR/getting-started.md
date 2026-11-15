# Primeiros passos

[English](../getting-started.md) · [한국어](../ko/getting-started.md) · [日本語](../ja/getting-started.md) · [简体中文](../zh-CN/getting-started.md) · [繁體中文](../zh-TW/getting-started.md) · [Español](../es/getting-started.md) · [Français](../fr/getting-started.md) · [Deutsch](../de/getting-started.md) · **Português (Brasil)** · [Русский](../ru/getting-started.md) · [Italiano](../it/getting-started.md) · [Tiếng Việt](../vi/getting-started.md) · [Bahasa Indonesia](../id/getting-started.md) · [Türkçe](../tr/getting-started.md)

O Desktop Subsystem for Android™ (DSA) executa o Android em uma máquina virtual leve, com o
próprio kernel. Cada app Android abre em uma janela própria na sua área de trabalho.

## Status por plataforma

| Host | Status |
|---|---|
| Windows 11, x64 | Lançado. |
| Windows 11, ARM64 | Lançado. O Android também é ARM64, então os apps ARM rodam sem tradução. |
| Linux, x64 e ARM64 | Em andamento. Os pacotes `.deb` e `.rpm` são compilados na CI; ainda não lançado. |
| macOS, Apple silicon | Em andamento. O `DSA.app`, o `.pkg` e o `.dmg` dele são compilados na CI; ainda não lançado. |

## Instalar no Windows

Você precisa de Windows 11 em um PC x64 ou ARM64, com o recurso do Windows **Plataforma de
Máquina Virtual** ativado.

1. Ative a Plataforma de Máquina Virtual. Abra **Ativar ou desativar recursos do Windows**,
   marque **Plataforma de Máquina Virtual**, selecione **OK** e reinicie. Ou execute
   `dism /online /enable-feature /featurename:VirtualMachinePlatform /all` em um terminal de
   administrador. A instalação não ativa o recurso por você: ela para com uma mensagem se ele
   estiver desativado.
2. Baixe o programa de instalação e o `SHA256SUMS.txt` da
   [versão mais recente](https://github.com/thisisthepy/desktop-subsystem-for-android/releases/latest):
   `DSA-v<version>-win-x64-setup.exe` em um PC x64, `DSA-v<version>-win-arm64-setup.exe` em um
   PC ARM64.
3. Confira o download. O hash que este comando mostra precisa ser igual ao da linha em
   `SHA256SUMS.txt`:

   ```
   certutil -hashfile DSA-v<version>-win-x64-setup.exe SHA256
   ```

4. Execute a instalação, selecione **Install** (instalar) e aprove a solicitação de
   administrador.
5. Aguarde a primeira imagem do Android. Quando o instalador não traz uma, o serviço do DSA a
   baixa em segundo plano. `dsa status` mostra o download.
6. Abra um app pelo menu Iniciar ou verifique por um terminal:

   ```
   dsa start --wait
   dsa launch com.android.settings
   ```

## O que a instalação instala

- O serviço `DsaHost`, que executa o Android de cada usuário.
- O comando `dsa`.
- **DSA Settings** (as configurações do DSA) no menu Iniciar.
- **Android** no painel de navegação do Explorador de Arquivos, que abre os arquivos do Android.
- O APK Installer (instalador de APK), que abre arquivos `.apk`.
- O DSA Window Manager (`dsa-wm`). Ele inicia quando você entra na sua conta e já inicia o
  Android nesse momento, então o primeiro app que você abrir não espera o Android inicializar.

Cada usuário do Windows tem o próprio Android e o próprio disco de dados,
`%LOCALAPPDATA%\DSA\android.vhdx`. Outros usuários não conseguem lê-lo.

O DSA usa o próprio kernel e a própria máquina virtual. Ele não usa nem altera as suas
configurações, o seu kernel ou as suas distribuições do WSL.

## Atualizações

O DSA atualiza a si mesmo e a imagem do Android. Os dois são assinados, e o DSA não aplica nada
cuja assinatura, tamanho ou hash não confira.

- **O próprio DSA** é atualizado quando o Android de nenhum usuário está rodando. Até lá, a
  atualização espera; `dsa update` a instala na hora. O DSA nunca passa para uma versão mais
  antiga.
- **Canais de versão.** A partir da 1.0, o DSA tem um canal estável e um canal de
  pré-lançamento, para versões que ainda estão sendo finalizadas. O seu canal começa pelo
  instalador que você usou, e as atualizações o mantêm. Um administrador o altera com **Get
  pre-release versions** (receber versões de pré-lançamento) na página **Updates**
  (atualizações) das DSA Settings. Todas as versões anteriores à 1.0 são pré-lançamentos.
- **A imagem do Android** não depende do canal. Se uma imagem não consegue inicializar, o DSA
  volta para a anterior, e `dsa status` informa isso.

Um administrador pode desativar as atualizações na página **Updates** das DSA Settings. Veja
[Configurações](settings.md) para todas as chaves.

## Linux (em andamento, não lançado)

O DSA para Linux é empacotado como `.deb` (Debian, Ubuntu) e `.rpm` (Fedora, openSUSE). Ele
precisa de KVM (`/dev/kvm`) e de uma área de trabalho Wayland. O Android roda em uma máquina
virtual crosvm.

Instalar um pacote também adiciona o repositório de pacotes assinado do DSA, então as versões
seguintes chegam pelo `apt`, `dnf`, `zypper` ou pela central de software. O DSA não tem
atualizador próprio no Linux.

- Debian e Ubuntu: `/etc/apt/sources.list.d/dsa.sources`, com a chave em
  `/usr/share/keyrings/dsa-archive-keyring.gpg`.
- Fedora e openSUSE: `/etc/yum.repos.d/dsa.repo` (e `/etc/zypp/repos.d/dsa.repo` onde o zypper
  estiver presente), com a chave em `/etc/pki/rpm-gpg/RPM-GPG-KEY-dsa`.

O repositório será servido a partir de
`https://thisisthepy.github.io/desktop-subsystem-for-android/`. Ele ainda não foi publicado.

Depois do lançamento, a primeira instalação fica assim:

```
sudo apt install ./desktop-subsystem-for-android_<version>_amd64.deb   # Debian, Ubuntu
sudo dnf install ./desktop-subsystem-for-android-<version>*.rpm       # Fedora
```

O pacote instala o serviço de sistema `dsa-hostd`, os serviços de usuário `dsa-devices` e
`dsa-compositor`, o comando `dsa` em `/usr/bin`, a entrada de menu **DSA Files** e as ações de
arquivo **Open in an Android app**, **Share with an Android app** e **Install in Android**.

## macOS (em andamento, não lançado)

O DSA para macOS executa o Android no Virtualization framework da Apple, em Apple silicon. Ele
é empacotado como `DSA.app`, em um pacote de instalação (`DSA-<version>.pkg`) e em uma imagem de
disco (`DSA-<version>.dmg`). Os builds da CI têm assinatura ad hoc, então só rodam na máquina
que os compilou. Ainda não existe uma versão assinada com um Developer ID.

## Próximos passos

- [Apps e janelas](apps.md)
- [Instalar APKs](install-apks.md)

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. macOS é uma marca registrada da Apple Inc. O DSA não é afiliado ao Google,
à Microsoft nem à Apple, nem endossado ou patrocinado por eles.
