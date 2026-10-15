const DEFAULT_MAX_DECIMALS = 3;

const formatters = new Map<number, Intl.NumberFormat>();

/** `1.5` → `"1,5"` (pt-BR), without thousands separators and without `-0`. */
export function formatNumber(value: number, maxDecimals = DEFAULT_MAX_DECIMALS): string {
  return formatterFor(maxDecimals).format(value).replace(/^-0$/, '0');
}

/** Same as {@link formatNumber}, escaped for LaTeX (a bare `,` would add a space). */
export function formatNumberLatex(value: number, maxDecimals = DEFAULT_MAX_DECIMALS): string {
  return formatNumber(value, maxDecimals).replace(',', '{,}');
}

function formatterFor(maxDecimals: number): Intl.NumberFormat {
  let formatter = formatters.get(maxDecimals);
  if (!formatter) {
    formatter = new Intl.NumberFormat('pt-BR', {
      maximumFractionDigits: maxDecimals,
      useGrouping: false,
    });
    formatters.set(maxDecimals, formatter);
  }
  return formatter;
}
