# Princípios de UX

1. **O gráfico é o protagonista.** O canvas ocupa a maior área; painéis são recolhíveis.
   Nada de decoração que compita com a matemática.

2. **Feedback imediato.** Não existe botão "Calcular". Digitou, o gráfico muda; arrastou, os
   valores atualizam. Erros aparecem inline, sublinhando o trecho da expressão, sem modais.

3. **Uma cor, um significado.** Cores do gráfico são semânticas e consistentes no app inteiro:
   função = tinta neutra, tangente = verde, derivada = vermelho/rosa, ponto ativo = acento.
   Nunca usar cor como *único* canal: linhas também variam em estilo (sólida, tracejada) e têm rótulo.

4. **Teclado primeiro.** `Ctrl/⌘ + K` abre a command palette; `Tab` percorre expressões;
   setas movem o ponto ativo; `+`/`−` dão zoom. Todo atalho aparece em tooltip.

5. **Divulgação progressiva.** O básico visível; o avançado (passos da derivação, tabela de
   valores, configurações de amostragem) atrás de um clique.

6. **Notação real.** Mostrar `f'(x) = 2x` renderizado em LaTeX, não `f'(x)=2*x`.
   Aceitar entrada natural: `2x`, `x²`, `sin x`, `√x`.

7. **Estados sempre desenhados.** Todo componente tem vazio, carregando, erro e sucesso definidos.
   Estado vazio ensina: "Digite uma função, por exemplo `x^2`".

8. **Acessibilidade.** Contraste AA (4.5:1 para texto), foco visível, `aria-label` em canvas com
   descrição textual do gráfico, respeito a `prefers-reduced-motion`.

9. **Consistência > criatividade.** Reutilize componentes do [design system](design-system.md)
   antes de criar novos.
