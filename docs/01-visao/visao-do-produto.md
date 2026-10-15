# Visão do produto

## O problema

Estudantes de física e exatas alternam entre várias ferramentas para estudar: uma calculadora
gráfica para plotar, um CAS para derivar, uma planilha ou script para métodos numéricos, papel
para vetores. Cada uma tem uma interface diferente, muitas são pesadas, fechadas ou datadas.

O que falta é **ver a matemática acontecendo**: arrastar um ponto e observar a reta tangente e a
derivada se formando, como nas figuras que motivaram o projeto:

- `f(x) = x²` com a tangente em verde e `f'(x) = 2x` sendo desenhada em vermelho;
- `f(x) = x³` com `f'(x) = 3x²` — a tangente fica horizontal em `x = 0`.

## A proposta

**Math View** é um laboratório visual, rápido e open source para matemática e física:

- **Visual primeiro:** todo conceito tem uma representação gráfica interativa.
- **Rápido:** núcleo em Rust; responde enquanto o usuário arrasta (meta: 60 fps, < 16 ms por quadro).
- **Minimalista:** a interface some; o gráfico é o protagonista.
- **Abrangente, mas modular:** cálculo, vetores, geometria, álgebra linear, métodos numéricos e física,
  cada um como um módulo independente.
- **Aberto:** código legível para humanos e agentes de IA, fácil de contribuir.

## Público

| Persona                         | Necessidade principal                                        |
| ------------------------------- | ------------------------------------------------------------ |
| Estudante de física (1º–4º sem.) | Entender conceitos de Cálculo e Física I visualmente         |
| Monitor / professor             | Montar demonstrações rápidas para aula                        |
| Contribuidor                    | Adicionar um módulo novo sem entender o projeto inteiro       |

## Princípios de produto

1. **Mostrar, não só calcular.** Resultado numérico sempre acompanhado de visualização.
2. **Feedback imediato.** Nada de botão "calcular": mudou a entrada, mudou o gráfico.
3. **Teclado primeiro, mouse bem-vindo.** Tudo acessível por atalhos e command palette.
4. **Notação matemática de verdade.** Entradas e saídas renderizadas em LaTeX (KaTeX).
5. **Offline e leve.** Funciona sem internet; instalador pequeno (Tauri).

## Não-objetivos (por enquanto)

- Ser um CAS completo como Mathematica/Maple.
- Colaboração em tempo real ou contas de usuário.
- Aplicativo mobile nativo (a versão web deve funcionar razoavelmente em tablets).
