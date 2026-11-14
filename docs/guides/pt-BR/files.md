# Arquivos

[English](../files.md) · [한국어](../ko/files.md) · [日本語](../ja/files.md) · [简体中文](../zh-CN/files.md) · [繁體中文](../zh-TW/files.md) · [Español](../es/files.md) · [Français](../fr/files.md) · [Deutsch](../de/files.md) · **Português (Brasil)** · [Русский](../ru/files.md) · [Italiano](../it/files.md) · [Tiếng Việt](../vi/files.md) · [Bahasa Indonesia](../id/files.md) · [Türkçe](../tr/files.md)

Com o Desktop Subsystem for Android™ (DSA), você abre os arquivos do Android no Explorador de
Arquivos, compartilha suas pastas do Windows com apps Android e envia um arquivo para um app
Android. Esta página descreve o DSA no Windows.

## Abrir os arquivos do Android no Explorador de Arquivos

Use qualquer uma destas opções. Cada uma inicia o Android quando ele não está rodando.

- **Android** no painel de navegação do Explorador de Arquivos, logo abaixo de **Linux**, do
  WSL.
- **Android files** (arquivos do Android) na página **Advanced** (avançado) das DSA Settings.
- `dsa files open`. `dsa files` informa onde fica o armazenamento, sem abri-lo.

Você vê o armazenamento compartilhado do Android: os arquivos que os apps Android salvaram, como
fotos e downloads. Você pode copiar arquivos para dentro e para fora como em qualquer pasta. O
Explorador de Arquivos acessa o armazenamento pelo cliente WebDAV do Windows. Se o armazenamento
não responder, o DSA mostra um erro com o motivo.

## Compartilhar suas pastas do Windows com apps Android

Suas pastas Documentos, Downloads, Imagens, Músicas, Vídeos e Área de Trabalho aparecem no
armazenamento compartilhado do Android em `Host`, como `Host/Pictures`. Os apps Android podem
abrir e salvar arquivos ali. Um app precisa antes da permissão de armazenamento do Android, e o
Windows só o deixa fazer o que a sua própria conta pode fazer.

O compartilhamento vem ativado por padrão. Para alterá-lo, abra a página **Advanced** das DSA
Settings:

- Desative **Share Windows folders with Android** para não compartilhar nenhuma pasta.
- Em **Shared folders** (pastas compartilhadas), desmarque uma pasta para parar de
  compartilhá-la. Ela sai do Android na hora.

Pela linha de comando:

```
dsa settings set shared-folders Documents,Pictures   # só estas; também all ou none
dsa settings set file-sharing off                     # não compartilha nenhuma pasta
```

As versões anteriores usavam uma pasta chamada `Windows` em vez de `Host`. O Android remove a
pasta antiga quando ela fica vazia, então os arquivos que você salvou nela permanecem.

## Enviar um arquivo para um app Android

- **Arrastar e soltar.** Arraste um arquivo do Explorador de Arquivos para a janela de um app. O
  arquivo vai para esse app. Um APK vai para o APK Installer. Pastas não são enviadas.
- **Menu de contexto.** Clique com o botão direito em um arquivo e selecione **Open in an
  Android app** ou **Share with an Android app**. No Windows 11, essas opções ficam em **Mostrar
  mais opções**. O Android então pergunta qual app recebe o arquivo.
- **Linha de comando.** `dsa open <file>` e `dsa share <file>`.

Essas formas enviam arquivos de até 64 MiB. Um arquivo maior é recusado antes de qualquer envio.
Para entregar um arquivo maior a um app, coloque-o em uma pasta compartilhada.

## Obter um arquivo do Android

- Salve-o no Android em `Host/<folder>`. Ele passa a estar nessa pasta no seu PC.
- Ou abra **Android** no Explorador de Arquivos e copie o arquivo para fora.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
