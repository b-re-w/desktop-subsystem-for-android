# Apps e janelas

[English](../apps.md) · [한국어](../ko/apps.md) · [日本語](../ja/apps.md) · [简体中文](../zh-CN/apps.md) · [繁體中文](../zh-TW/apps.md) · [Español](../es/apps.md) · [Français](../fr/apps.md) · [Deutsch](../de/apps.md) · **Português (Brasil)** · [Русский](../ru/apps.md) · [Italiano](../it/apps.md) · [Tiếng Việt](../vi/apps.md) · [Bahasa Indonesia](../id/apps.md) · [Türkçe](../tr/apps.md)

Cada tarefa de um app Android abre como uma janela própria na área de trabalho, com o próprio
botão na barra de tarefas. Não há moldura de emulador em volta de uma tela de celular. Esta
página descreve o Desktop Subsystem for Android™ (DSA) no Windows.

## Abrir um app

- **Menu Iniciar.** Cada app instalado tem uma entrada no menu Iniciar com o próprio nome e
  ícone, então a Pesquisa do Windows o encontra. Abri-lo inicia o Android antes, quando o
  Android não está rodando.
- **DSA Settings** (as configurações do DSA). Na página **Apps**, selecione **Open** (abrir) ao
  lado do app.
- **Linha de comando.** `dsa apps list` lista os apps instalados com os pacotes deles.
  `dsa launch <package>` abre um deles, por exemplo `dsa launch com.android.settings`.

Quando a entrada no menu Iniciar de outro programa já tem o nome do app, o DSA acrescenta o
pacote ao nome, como `Clock (com.example.clock)`. O DSA só altera ou remove as entradas que ele
mesmo criou.

## Atalhos na área de trabalho e listas de atalhos

Os ícones dos apps funcionam como os dos apps do Windows, e não como os da tela inicial do
Android.

- **Atalho na área de trabalho.** Na página **Apps** das DSA Settings, selecione **Create
  desktop shortcut** (criar atalho na área de trabalho) ao lado do app. Você também pode clicar
  com o botão direito no botão do app na barra de tarefas e selecionar **Create desktop
  shortcut**. Pela linha de comando: `dsa apps desktop-shortcut <package>`. O atalho abre o app
  como a entrada dele no menu Iniciar faz.
- **Atalhos do app.** Os atalhos que um app Android oferece, como uma nova conversa de um
  mensageiro, aparecem quando você clica com o botão direito no botão do app na barra de tarefas
  ou na entrada dele no menu Iniciar. Selecionar um abre aquela tela do app em uma janela,
  iniciando o Android antes quando ele não está rodando. Quando o app muda os atalhos, a lista
  acompanha. Pela linha de comando: `dsa apps shortcut <package> <id>`;
  `dsa apps info <package> --json` lista os IDs.
- **Atalhos que um app pede.** Quando um app pede para adicionar um dos atalhos dele à sua área
  de trabalho, o DSA pergunta a você antes e só o adiciona se você concordar.

Quando um app muda de nome, por exemplo depois de uma atualização, a entrada dele no menu
Iniciar, os atalhos dele na área de trabalho e a fixação dele na barra de tarefas seguem o novo
nome e continuam funcionando. Um atalho na área de trabalho que você excluiu não é criado de
novo. Desinstalar um app também remove os atalhos dele na área de trabalho.

## Trabalhar com as janelas dos apps

- Cada tarefa tem a própria janela, o próprio botão na barra de tarefas e a própria entrada no
  Alt+Tab, com o nome e o ícone do app. O Gerenciador de Tarefas lista cada app com o próprio
  nome.
- A barra de título faz parte do app: as cores do app chegam ao topo da janela, e os botões de
  legenda do Windows 11 ficam sobre ela. O ícone e o nome do app ficam à esquerda. Quando o app
  pode voltar, o ícone vira uma seta de voltar; selecione-a para voltar.
- Os layouts de ajuste (passe o ponteiro sobre o botão de maximizar), arrastar a barra de
  título, clicar duas vezes nela para maximizar e o redimensionamento ao vivo funcionam.
  Redimensionar não reinicia o app. Uma janela maximizada preenche a área de trabalho do monitor
  e deixa a barra de tarefas visível.
- Não há barra de navegação do Android. A barra inferior de um app fica na borda inferior da
  janela, sem uma faixa vazia abaixo dela.
- Mouse (com hover), toque, caneta (pressão, borracha e botão lateral) e teclado funcionam. O
  texto vem do seu método de entrada do Windows, incluindo o coreano e outros idiomas; a
  composição aparece diretamente no editor do Android, e a janela de candidatos fica ao lado do
  cursor. Não há app de teclado do Android.
- Arraste arquivos do Explorador de Arquivos para a janela de um app, ou arraste arquivos de um
  app para o Explorador de Arquivos.
- Leitores de tela como o Narrador (Narrator) e o NVDA leem o app pelo UI Automation.
- Fechar uma janela pede ao Android que encerre aquela tarefa.

## O menu da janela

Clique com o botão direito na barra de título, selecione o ícone do app ou pressione
Alt+Espaço. Depois dos itens habituais do Windows, o menu tem:

- **Full screen** (tela cheia): preenche a tela inteira, como F11. Selecione de novo, ou
  pressione F11, para sair.
- **New window** (nova janela): abre outra janela do app, para apps que aceitam várias.
- **Screenshot** (captura de tela): copia a tela do app, sem a barra de título, para a área de
  transferência e a salva na pasta Capturas de Tela da sua pasta Imagens.
- **Aspect ratio** (proporção): abre a configuração do Android para a proporção do app, para
  apps que parecem esticados em uma janela grande.
- **Open by default** (abrir por padrão): escolhe se os links que o app aceita abrem no app ou
  no seu navegador.
- **App info** (informações do app): abre a entrada do app na página **Apps** das DSA Settings.

## Tela cheia

Quando um app pede tela cheia, a janela dele a mostra de uma de duas formas:

- **Na tela inteira**: a janela preenche o monitor, e a barra de título se esconde. Toque a
  borda superior da tela com o ponteiro para mostrá-la, com minimizar e fechar; o botão do meio
  sai da tela cheia.
- **Dentro da janela**: a janela mantém o tamanho e a barra de título, e o app a preenche.

Os apps pedem de dois tipos de forma, e cada tipo tem o próprio padrão:

| Tipo de pedido | Exemplo | Padrão |
|---|---|---|
| Um pedido claro de tela cheia | O botão de tela cheia de um player de vídeo | Na tela inteira |
| Só esconder as barras | Um feed de vídeos curtos | Dentro da janela |

Altere os padrões em **App full screen** e **Apps that hide their bars** na página **Display &
GPU** das DSA Settings, ou para um único app em **Full screen** e **Bars hidden** na página
**Apps**. A escolha do próprio app vem primeiro. O F11 e o item **Full screen** do menu da
janela alternam a tela cheia na tela inteira a qualquer momento, seja qual for a configuração.

Pela linha de comando:

```
dsa settings set app-fullscreen screen|window           # padrão para pedidos claros
dsa settings set app-immersive screen|window            # padrão para barras escondidas
dsa app-window <package> --fullscreen screen|window|default
dsa app-window <package> --immersive screen|window|default
```

## Definir o tamanho da janela de um app

Na página **Apps** das DSA Settings:

- Desative **Resizable** (redimensionável) para manter as janelas do app no tamanho em que
  abrem.
- Escolha o tamanho em que as janelas do app abrem: o tamanho que o Android escolhe, ou um
  tamanho de celular ou tablet em retrato ou paisagem.

A alteração vale para a próxima janela que o app abrir. Pela linha de comando:

```
dsa app-window <package> --fixed-size on        # não pode ser redimensionada nem maximizada
dsa app-window <package> --start-size 1280x800  # abre neste tamanho; auto deixa o Android escolher
dsa app-window <package> --dpi-scaling system   # como escala em outro monitor; veja abaixo
dsa app-window <package> reset                  # volta aos padrões
```

## Mover um app para um monitor com outra escala

Um app abre na escala do monitor em que a janela dele abre, desde o primeiro quadro, então ele
não é recriado logo depois de abrir. Quando você move uma janela para um monitor com outra
escala, o app mantém o tamanho na tela. Cada app faz isso de uma destas formas, que você escolhe
em **Scaling on another monitor** (escala em outro monitor) na página **Apps**, ou com
`dsa app-window <package> --dpi-scaling`:

- **Automatic** (`auto`, o padrão): **System** para os apps da lista interna do DSA de apps que
  quebram quando a escala muda durante a execução, e **App** para todos os outros. A lista
  contém o YouTube: ele mantém o layout da escala em que iniciou, então, depois de uma mudança,
  as guias inferiores e o miniplayer ficariam quebrados até ele reiniciar. As DSA Settings mostram
  o padrão de um app assim como **Scaling: system (recommended for this app)**. A sua própria
  escolha para o app vem primeiro.
- **App** (`app`): enquanto você arrasta a janela, o DSA a escala para o novo monitor. Quando
  você solta, o app é redesenhado na escala do novo monitor, e o texto continua nítido: o
  Android recria a tela do app uma vez e restaura o estado dele (posição de rolagem, texto
  digitado), então ele é redesenhado rapidamente depois de cada mudança, como um programa do
  Windows faz com a escala "Aplicativo". Isso vale mesmo para um app que diz tratar sozinho uma
  mudança de escala, porque alguns desses apps redesenham só parte da tela na nova escala.
- **System** (`system`): o app mantém a escala, e o DSA escala a janela dele para o novo
  monitor. O app nunca é recriado. Ampliado, o texto pode parecer um pouco borrado.
- **System (forced)** (`system-forced`): o app recebe a nova escala sem reiniciar, mesmo que não
  diga que a trata. Ainda não disponível: isso exige uma alteração no Android que esta versão
  não tem, então, até lá, funciona como **System**.

O texto nos apps segue o tamanho do texto do Windows (Configurações > Acessibilidade > Tamanho
do texto).

## Notificações

As notificações do Android aparecem como notificações do Windows, com o nome e o ícone do app.

- Selecione uma notificação para abrir o app.
- Descarte-a, e ela também é descartada no Android, a menos que o app a mantenha visível.
- As notificações continuam chegando enquanto o Android está em Doze. Veja
  [Memória](memory.md).

## Apps rodando em segundo plano

Um app que continua rodando depois que a janela dele fecha, como um player de música, aparece
como um ícone na área de notificação. A dica de ferramenta dele é a notificação do app.

- Selecione o ícone para abrir o app.
- Clique nele com o botão direito para ver **Open**, as próprias ações da notificação e
  **Stop** (parar).
- **Stop** pergunta antes. Marque **Do not ask again** (não perguntar de novo) para pular a
  pergunta daí em diante. Para voltar a ser perguntado, ative **Ask before stopping an app from
  the notification area** na página **Running in the background** (em segundo plano) das DSA
  Settings.
- Um app que reproduz mídia também funciona com as teclas de mídia e os controles de mídia do
  Windows.

A página **Running in the background** das DSA Settings lista esses apps, com **Open** e
**Stop** para cada um. Pela linha de comando: `dsa background`,
`dsa background open <package>` e `dsa background stop <package>`.

## Widgets na área de trabalho

Um widget do Android pode ficar na sua área de trabalho, em uma janela própria. Ele fica abaixo
das outras janelas, permanece quando você mostra a área de trabalho e não aparece na barra de
tarefas nem no Alt+Tab.

1. Abra a página **Widgets** das DSA Settings.
2. Em **Widgets your apps offer** (widgets que seus apps oferecem), selecione **Add to desktop**
   (adicionar à área de trabalho) ao lado de um widget. Se o app pedir que você configure o
   widget, a configuração dele abre em uma janela.
3. Para mover um widget, pressione em qualquer ponto dele e fique parado por meio segundo. Ele
   se levanta, com uma borda e uma alça em cada lado pelo qual pode ser redimensionado. Arraste-o
   para movê-lo, ou arraste uma alça para redimensioná-lo. Um arraste rápido continua sendo do
   próprio widget, então as listas dele ainda rolam. Você também pode arrastar uma borda do
   widget para redimensioná-lo sem levantá-lo antes; só as bordas pelas quais ele pode ser
   redimensionado fazem isso. O DSA lembra onde você o colocou e em que tamanho.
4. Clique com o botão direito no widget, ou solte depois do toque longo, para ver **Widget
   settings** (quando o widget pode ser configurado de novo), **Resize**, para escolher um dos
   tamanhos que ele permite, e **Remove**. A tecla de menu e Shift+F10 também o abrem. As teclas
   de seta, Enter e Esc funcionam nesse menu, e uma letra escolhe um item: R para **Resize** e M
   para **Remove**.

Pela linha de comando: `dsa widgets list`, `dsa widgets add <provider>` e
`dsa widgets remove <id>`.

## Links e a área de transferência

- Um link da web que você toca em um app Android abre no seu navegador padrão. Um link que um
  app instalado verificou como sendo dele abre nesse app.
- Para abrir todos os links da web em apps Android, ative **Open web links in Android apps** na
  página **Advanced** (avançado) das DSA Settings (`dsa settings set links-in-android on`).
- O texto que você copia é compartilhado nos dois sentidos entre o Windows e o Android. Para
  impedir isso, desative **Share the clipboard with Android** na página **Advanced**
  (`dsa settings set clipboard off`).

## Google Play e vídeo protegido

O DSA não inclui nenhum software do Google. Se você quiser, o DSA o baixa da origem para você,
depois de mostrar a origem e os termos. Os dois passam a valer na próxima inicialização do
Android, e os dois podem ser removidos de novo.

- **Google Play.** Na página **Advanced** das DSA Settings, selecione **Install** em **Google
  Play**, ou execute `dsa gapps install`. O DSA baixa o pacote público LiteGapps Core (Google
  Play services, a Play Store e a sincronização de conta) para a sua versão do Android. Este
  Android não é certificado pelo Google, então o Google pode se recusar a deixar você entrar até
  que você registre o dispositivo: **Register this device with Google** mostra o ID do Android e
  abre a página de registro. `dsa gapps status` mostra o estado, e `dsa gapps remove` remove os
  apps do Google, mesmo com o Android parado.
- **Widevine.** Os apps de streaming precisam do módulo Widevine do Google para reproduzir vídeo
  protegido. Na página **Advanced**, selecione **Get** em **Protected content**, ou execute
  `dsa drm widevine install`. O DSA baixa a imagem do emulador Android do Google para a sua
  versão do Android (cerca de 2 GB), mantém só o módulo Widevine, sem alterações, e exclui o
  resto. Aqui o Widevine tem nível de segurança L3, então os serviços de streaming enviam no
  máximo definição padrão; para alta definição, use um navegador ou app do Windows.
  `dsa drm widevine remove` o remove.

## Apps feitos só para ARM

Em um PC x64, um app com código só para processadores ARM roda por meio de um tradutor de ARM.
O Android em um PC x64 já vem com um, o tradutor de código aberto Digitalis, e o usa desde a
primeira inicialização. Você pode usar outro no lugar dele:

- **O tradutor de ARM do Google.** Na página **Advanced**, selecione **Get** ao lado de
  **Google's ARM translator**, ou execute `dsa arm-translation fetch google`. O DSA mostra
  primeiro de onde ele vem e os termos do Google (a
  [Android SDK License](https://developer.android.com/studio/terms)), e não baixa nada até você
  aceitar. Depois, o seu PC baixa a imagem do emulador Android que o Google oferece para a sua
  versão do Android (cerca de 2 GB), mantém só o tradutor e exclui o resto. O DSA não inclui nem
  repassa o tradutor do Google.
- **Um pacote que você compila ou obtém em outro lugar**, como um feito a partir do tradutor de
  código aberto Digitalis: `dsa arm-translation install <folder>`.

Depois, reinicie o Android: pare-o na página **General** (geral) das DSA Settings e, em
seguida, abra um app.

Para voltar ao tradutor que vem com o Android, selecione **Use** ao lado de **Use the default
translator** na página **Advanced**, ou execute `dsa arm-translation use-default`. Isso remove o
pacote que você instalou, a partir da próxima inicialização do Android.

`dsa arm-translation status` informa qual tradutor o Android usa. A chave **Run apps built only
for ARM** na página **Advanced** ativa ou desativa a tradução sem remover nada. Para um app que
tem código ARM e x64, **Run as ARM** na página **Apps** (`dsa apps run-as-arm <package> on`)
executa o código ARM dele. Só apps ARM de 64 bits (`arm64-v8a`) podem rodar. Em um PC ARM64, o
próprio Android roda os apps ARM e não precisa de tradutor.

## Acesso root

Alguns apps, como ferramentas de backup, precisam de root. O DSA o concede como o `sudo` faz em
um PC com Linux: só quando um administrador do PC aprova, um pedido de cada vez. Ele vem
desativado por padrão, e apps sem permissão nem enxergam o `su`.

1. Um administrador ativa **Let Android apps ask for root** em **Root access** (acesso root) na
   página **Advanced** das DSA Settings (`dsa settings set root-elevation on`). Isso vale a
   partir da próxima inicialização do Android.
2. Na página **Apps**, defina o **Root access** do app:
   - **Root: not allowed** (o padrão): o app não tem `su`.
   - **Root: ask each time**: cada vez que o app usa o `su`, o Windows mostra a solicitação de
     administrador (UAC) com o nome e o ícone do próprio app. Um usuário padrão digita a senha
     de um administrador. Uma aprovação vale por 15 minutos para aquele app.
   - **Root: remember (no asking)**: o app usa o `su` sem perguntar. Escolher esta opção exige a
     aprovação de um administrador.

   Pela linha de comando: `dsa apps root <package> off|ask|remember`.

Com o modo de desenvolvedor ativado, o `adb shell` também pode usar o `su`. Ele tem a própria
linha na página **Apps**, **adb shell**, e pergunta todas as vezes por padrão
(`dsa apps root adb ask`). A solicitação dele se chama **adb shell** e mostra o comando.

Quando uma parte do próprio Android pede, a solicitação mostra o nome dela: **Android system**,
**Android phone services**, **Android Bluetooth**, **Android NFC** ou **Android system
service**. Os apps que compartilham um mesmo ID de usuário são aprovados como um único grupo, e
a solicitação lista todos eles.

Cada pedido fica registrado no log do serviço, com o app, o horário, o comando e se foi
aprovado. Desativar o recurso esconde o `su` de todos os apps de uma vez, sem reiniciar.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
