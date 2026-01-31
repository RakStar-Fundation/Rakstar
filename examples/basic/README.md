# Basic Example

Este é um exemplo básico de como usar o framework RakStar.

## Estrutura

- Implementa o trait `Component`
- Usa a macro `entrypoint!` para criar o ponto de entrada
- Define nome e versão do componente

## Como compilar

```bash
cd examples/basic
cargo build --release
```

A biblioteca será gerada em `target/release/libbasic.so`

## Como usar

1. Copie `libbasic.so` para o diretório `components/` do servidor open.mp
2. Inicie o servidor
3. O componente será carregado automaticamente
