# Instalar APKs

[English](../install-apks.md) · [한국어](../ko/install-apks.md) · [日本語](../ja/install-apks.md) · [简体中文](../zh-CN/install-apks.md) · [繁體中文](../zh-TW/install-apks.md) · [Español](../es/install-apks.md) · [Français](../fr/install-apks.md) · [Deutsch](../de/install-apks.md) · **Português (Brasil)** · [Русский](../ru/install-apks.md) · [Italiano](../it/install-apks.md) · [Tiếng Việt](../vi/install-apks.md) · [Bahasa Indonesia](../id/install-apks.md) · [Türkçe](../tr/install-apks.md)

O Desktop Subsystem for Android™ (DSA) não inclui uma loja de apps. Você instala apps a partir
de arquivos APK, com o APK Installer ou com o comando `dsa`. Esta página descreve o DSA no
Windows.

## Instalar com o APK Installer

1. Clique duas vezes em um arquivo `.apk` no Explorador de Arquivos. Ele abre no **APK
   Installer** (instalador de APK).
2. Confira o que é o APK. A janela mostra isso sem iniciar o Android:
   - o nome, o ícone, o pacote e a versão do app;
   - o editor, tirado do certificado de assinatura, porque um APK não informa o editor;
   - a origem: o host de onde o arquivo foi baixado, ou a pasta dele;
   - em **Capabilities** (recursos), as primeiras permissões que ele pede. **More** (mais)
     mostra todas elas, as versões do Android para as quais ele foi feito, o tamanho, os tipos
     de processador e o SHA-256 do certificado.

   Quando o APK tem código só para outro tipo de processador, a janela avisa você.
3. Selecione **Install** (instalar). Quando o app já está instalado, o botão diz **Update**
   (atualizar) ou **Install again** (instalar de novo). O Android inicia antes, quando não está
   rodando.
4. Com **Launch when ready** (abrir quando estiver pronto) marcado (o padrão), o app abre assim
   que é instalado, e o instalador fecha. Caso contrário, selecione **Launch** (abrir), ou abra
   o app depois pelo menu Iniciar.

Enquanto o Android está rodando, a janela também informa se o arquivo é uma nova instalação,
uma atualização, a mesma versão de novo ou algo que o Android recusa: uma versão mais antiga, ou
o mesmo app assinado com outra chave.

Você também pode soltar arquivos APK no APK Installer, nas DSA Settings ou na janela de um app.
Cada APK abre em uma janela de instalador própria.

## Instalar pela linha de comando

```
dsa apk-info app.apk            # o que é o APK; lê o arquivo sem iniciar o Android
dsa install app.apk             # instala; inicia o Android quando ele não está rodando
dsa install app.apk --progress  # também mostra cada etapa
```

Quando o Android recusa o APK, `dsa install` sai com o código 1, e a mensagem dele começa com
`not installed (<reason>):`.

## Por que uma instalação é recusada

| Motivo | Significado | O que fazer |
|---|---|---|
| `incompatible` | O app precisa de outro tipo de processador, de uma versão mais nova do Android ou de um recurso que este dispositivo não tem | Veja [Apps feitos só para ARM](apps.md#apps-feitos-só-para-arm) ou [Versões do Android](android-versions.md) |
| `conflicting-signature` | O app já está instalado, assinado com outra chave | Desinstale antes o app instalado |
| `downgrade` | Uma versão mais nova do app está instalada | Desinstale antes o app instalado |
| `invalid-apk` | O arquivo não é um APK assinado que o Android consiga ler | Obtenha o APK de novo |
| `insufficient-storage` | O Android não tem espaço para o app | Libere espaço no Android |
| `blocked` | Uma política deste Android não permite o app | |
| `other` | Qualquer outro caso; a mensagem do próprio Android vem em seguida | |

Um APK pode ter até 2 GiB. As instalações rodam uma de cada vez: uma segunda espera a primeira.

## Desinstalar um app

Use qualquer uma destas opções:

- **Configurações** > **Aplicativos** > **Aplicativos instalados** do Windows.
- A página **Apps** das DSA Settings: **Uninstall** (desinstalar) ao lado do app.
- `dsa apps uninstall <package>`.

Os apps que fazem parte do Android não podem ser desinstalados.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
