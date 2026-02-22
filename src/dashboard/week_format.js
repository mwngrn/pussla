function formatWeekLabel(weekKey) {
  const match = /^(\d{4})-W(\d{2})$/.exec(String(weekKey || ''));
  if (!match) return String(weekKey || '');
  return `W${match[2]}`;
}

if (typeof window !== 'undefined') {
  window.formatWeekLabel = formatWeekLabel;
}

if (typeof module !== 'undefined' && module.exports) {
  module.exports = { formatWeekLabel };
}
