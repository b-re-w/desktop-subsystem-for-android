# Memória e comportamento ocioso

[English](../memory.md) · [한국어](../ko/memory.md) · [日本語](../ja/memory.md) · [简体中文](../zh-CN/memory.md) · [繁體中文](../zh-TW/memory.md) · [Español](../es/memory.md) · [Français](../fr/memory.md) · [Deutsch](../de/memory.md) · **Português (Brasil)** · [Русский](../ru/memory.md) · [Italiano](../it/memory.md) · [Tiếng Việt](../vi/memory.md) · [Bahasa Indonesia](../id/memory.md) · [Türkçe](../tr/memory.md)

O Desktop Subsystem for Android™ (DSA) devolve memória ao seu PC enquanto o Android roda, e não
só quando ele para. Não há nada a configurar para isso. Esta página explica o que acontece e
como acompanhar.

## Ver quanta memória o Android usa

Abra a página **Memory** (memória) das DSA Settings, ou execute `dsa memory`. As duas opções
mostram:

- **On this PC** (neste PC): o que a máquina virtual do Android, o gerenciador de janelas e
  cada janela de app ocupam.
- **Inside Android** (dentro do Android): a memória em uso, o cache, a memória comprimida e os
  maiores processos. O Android informa isso alguns minutos depois de iniciar.

## Doze, o estado ocioso

Quando todas as janelas do Android ficam fechadas ou minimizadas por 30 segundos, a tela do
Android se apaga e o Android entra em Doze, como um celular faz no bolso.

- Notificações, alarmes e widgets continuam funcionando em Doze. A CPU descansa entre eles.
- `dsa status` informa `Dozing`. A página **General** (geral) das DSA Settings informa **Dozing
  (idle)**.
- Mostrar qualquer janela do Android acorda o Android na hora.

O Android também acompanha a suspensão do seu PC. Ele salva o estado antes de o PC entrar em
suspensão e acerta o relógio quando o PC acorda.

## Congelamento (desativado por padrão)

O congelamento para todos os processos do Android. Assim ele não usa CPU, mas notificações,
alarmes e widgets também param. Por isso o DSA nunca congela o Android a menos que você peça.

- Para congelar agora: **Freeze** em **Freeze Android now** na página **General**, ou
  `dsa freeze`.
- Para congelar depois de 30 minutos em Doze: ative **Freeze Android when idle** na página
  **Advanced** (avançado), ou `dsa settings set freeze on`.
- Abrir uma janela do Android, ou interagir com uma delas, retoma o Android.

## Como a memória volta

- A máquina virtual do Android só toma memória do seu PC quando o Android a usa pela primeira
  vez.
- A memória que o Android libera é devolvida ao seu PC enquanto o Android roda.
- Quando a última janela de um app fecha, o Android encerra os processos desse app, então a
  memória deles volta em segundos. Um app com um widget na área de trabalho, ou um que ainda
  trabalha em segundo plano (como um player de música), continua rodando.
- Um app que mantém um serviço em segundo plano rodando sem janela, sem notificação, sem som e
  sem widget é parado depois de cinco minutos. Se o Android o iniciar de novo e de novo, o DSA
  espera mais a cada vez, até uma hora.
- Enquanto nenhuma janela de app está aberta, o Android descarta os caches de arquivos e move a
  memória ociosa para a memória comprimida, para que mais memória possa voltar.
- O gerenciador de janelas devolve a memória que não usa 30 segundos depois que a última janela
  de app fecha.

## Limite de memória

O limite é **Automatic** (automático) por padrão, o que não define nenhum limite fixo. O
Android começa com 2 GB e cresce enquanto roda, até a memória do seu PC menos o que o Windows
reserva (10% da memória ou 2 GB, o que for maior). O Android só ocupa o que usa.
Para definir um limite, use **Memory limit** na página **Memory**, ou:

```
dsa settings set memory-limit 4g     # 4 GiB
dsa settings set memory-limit auto   # volta ao automático
```

Um novo limite passa a valer na próxima inicialização do Android.

---

Android é uma marca registrada da Google LLC. Windows é uma marca registrada do grupo de
empresas Microsoft. O DSA não é afiliado ao Google nem à Microsoft, nem endossado ou patrocinado
por eles.
