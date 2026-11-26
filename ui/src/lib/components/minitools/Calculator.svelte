<script lang="ts">
  // Calculator (360x530, dark #363636 body, #65A0EC operators): expression
  // display + memory indicator + 4-column keypad, like the recovered layout.
  let expr = $state('');
  let result = $state('0');
  let memory: number | null = $state(null);

  const KEYS: Array<{ k: string; cls?: string }> = [
    { k: 'MC' }, { k: 'MR' }, { k: 'M+' }, { k: 'M-' },
    { k: '%', cls: 'fn' }, { k: 'CE', cls: 'fn' }, { k: 'C', cls: 'fn' }, { k: '±', cls: 'fn' },
    { k: '7' }, { k: '8' }, { k: '9' }, { k: '÷', cls: 'op' },
    { k: '4' }, { k: '5' }, { k: '6' }, { k: '×', cls: 'op' },
    { k: '1' }, { k: '2' }, { k: '3' }, { k: '−', cls: 'op' },
    { k: '0' }, { k: '.' }, { k: '=' , cls: 'eq' }, { k: '+', cls: 'op' },
  ];

  function evaluate(): number | null {
    try {
      const js = expr.replace(/×/g, '*').replace(/÷/g, '/').replace(/−/g, '-').replace(/%/g, '/100');
      if (!/^[\d+\-*/(). ]*$/.test(js) || !js.trim()) return null;
      // Function constructor evaluates the sanitized arithmetic expression.
      const value = Function(`"use strict";return (${js})`)();
      return typeof value === 'number' && Number.isFinite(value) ? value : null;
    } catch {
      return null;
    }
  }

  function press(key: string) {
    if (key === 'C') {
      expr = '';
      result = '0';
    } else if (key === 'CE') {
      expr = '';
    } else if (key === '±') {
      result = result.startsWith('-') ? result.slice(1) : `-${result}`;
    } else if (key === '=') {
      const value = evaluate();
      if (value != null) {
        result = String(Math.round(value * 1e10) / 1e10);
        expr = '';
      }
    } else if (key === 'MC') {
      memory = null;
    } else if (key === 'MR') {
      if (memory != null) expr += String(memory);
    } else if (key === 'M+') {
      const v = evaluate() ?? (Number(result) || 0);
      memory = (memory ?? 0) + v;
    } else if (key === 'M-') {
      const v = evaluate() ?? (Number(result) || 0);
      memory = (memory ?? 0) - v;
    } else {
      expr += key;
      const live = evaluate();
      if (live != null) result = String(Math.round(live * 1e10) / 1e10);
    }
  }
</script>

<div class="calculator">
  <div class="calc-display">
    <div class="calc-expr">{expr || ' '}{#if memory != null}<span title="Memory"> M</span>{/if}</div>
    <div class="calc-result">{result}</div>
  </div>
  <div class="calc-pad">
    {#each KEYS as { k, cls } (k)}
      <button type="button" class="calc-key {cls ?? ''}" onclick={() => press(k)}>{k}</button>
    {/each}
  </div>
</div>
