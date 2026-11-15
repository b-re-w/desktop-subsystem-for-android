# Desinstalação

[English](../uninstall.md) · [한국어](../ko/uninstall.md) · [日本語](../ja/uninstall.md) · [简体中文](../zh-CN/uninstall.md) · [繁體中文](../zh-TW/uninstall.md) · [Español](../es/uninstall.md) · [Français](../fr/uninstall.md) · [Deutsch](../de/uninstall.md) · **Português (Brasil)** · [Русский](../ru/uninstall.md) · [Italiano](../it/uninstall.md) · [Tiếng Việt](../vi/uninstall.md) · [Bahasa Indonesia](../id/uninstall.md) · [Türkçe](../tr/uninstall.md)

## Windows

Desinstale o Desktop Subsystem for Android™ (DSA) de uma destas formas:

- **Configurações** > **Aplicativos** > **Aplicativos instalados** do Windows: encontre
  Desktop Subsystem for Android e selecione **Desinstalar**.
- Execute a instalação de novo e selecione **Uninstall** (desinstalar).

A desinstalação remove estes itens, para todos os usuários do PC, conectados ou não:

- o programa, o serviço `DsaHost`, o comando `dsa`, as DSA Settings e o APK Installer;
- as imagens do Android baixadas e as atualizações;
- as entradas dos apps Android no menu Iniciar e as entradas deles em **Aplicativos
  instalados**;
- **Android** no Explorador de Arquivos, a associação de arquivos `.apk` e as entradas do menu
  de contexto;
- as configurações e os logs do DSA.

Ela **mantém** o disco do Android de cada usuário, `%LOCALAPPDATA%\DSA\android.vhdx`, para que
uma instalação posterior encontre de novo os seus apps e dados. Para remover também os seus
dados, execute `dsa reset-data --yes` antes de desinstalar, ou exclua esse arquivo depois.

Se o redirecionamento de pastas tiver movido o menu Iniciar de um usuário, as entradas dos apps
Android desse usuário permanecem, porque o desinstalador procura só no local padrão. Exclua-as
manualmente.

A sua configuração do WSL não é afetada, porque o DSA nunca a altera.

## Linux (ainda não lançado)

```
sudo apt remove desktop-subsystem-for-android    # Debian, Ubuntu
sudo dnf remove desktop-subsystem-for-android    # Fedora
```

Remover o pacote também remove a entrada do repositório de pacotes que ele adicionou.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
