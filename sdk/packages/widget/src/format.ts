/**
 * Convert a raw ERC-20 amount (uint256 as string, integer) to a human display
 * value. Returns a string with up to `displayDecimals` fraction digits and
 * thousands separators.
 *
 * Example: rawAmount=`"1000000000000000000"`, tokenDecimals=18, displayDecimals=2
 * → `"1.00"`.
 */
export function formatTokenAmount(
  rawAmount: string,
  tokenDecimals = 18,
  displayDecimals = 2,
): string {
  let raw: bigint;
  try {
    raw = BigInt(rawAmount);
  } catch {
    return rawAmount;
  }
  if (raw === 0n) return "0";

  const negative = raw < 0n;
  const abs = negative ? -raw : raw;
  const base = 10n ** BigInt(tokenDecimals);
  const whole = abs / base;
  const frac = abs % base;

  const wholeStr = formatWithCommas(whole.toString());

  if (displayDecimals <= 0 || frac === 0n) {
    return (negative ? "-" : "") + wholeStr;
  }

  // Take the leading `displayDecimals` digits of the fraction, padded.
  const fracStr = frac.toString().padStart(tokenDecimals, "0");
  const fracTrunc = fracStr.slice(0, displayDecimals).replace(/0+$/, "");
  if (fracTrunc.length === 0) return (negative ? "-" : "") + wholeStr;
  return (negative ? "-" : "") + wholeStr + "." + fracTrunc;
}

function formatWithCommas(intStr: string): string {
  return intStr.replace(/\B(?=(\d{3})+(?!\d))/g, ",");
}

/**
 * Truncate a 0x-prefixed hex (tx hash, address) for compact display.
 */
export function shortHex(s: string, head = 6, tail = 4): string {
  if (!s) return "";
  if (s.length <= head + tail + 1) return s;
  return `${s.slice(0, head)}…${s.slice(-tail)}`;
}
