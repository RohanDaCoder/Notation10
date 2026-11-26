// Design tokens measured from the shipped application (docs/re/theme.json).
// All sizes are 1080p-base design units; app.css exposes `--rp` as the scale
// so components can write calc(var(--rp) * N) for every dimension, mirroring
// the vendor's RelativePixel markup system.

export const T = {
  measurement: {
    baseHeight: 1080,
    defaultWindow: [1280, 720] as const,
    shellMinWidth: 1440,
  },
  shell: {
    dockHeight: 80,
    dockHideOffset: 24,
    dockSlideMs: 200,
    subjectChangeDrop: 118,
    subjectChangeMs: 600,
    systemTimeMargin: [30, 24] as const,
    systemTimeOpacity: 0.8,
  },
  dock: {
    skinEdge: [30, 30, 30, 0] as const,
    cornerRadius: 16,
    collapseButtonWidth: 30,
    iconCell: 56,
    labelCell: 24,
    labelFontSize: 12,
    radioLift: 12,
    radioAnimMs: 200,
    activePillRadius: 20,
    disabledOpacity: 0.33,
    centerClusterSideMargin: 30,
    popupTopBand: 32,
    popupSideBands: 22,
    pointerSize: [50, 34] as const,
  },
  modeSelector: {
    containerSkinEdge: 20,
    inkItemWidth: 60,
    genericItem: 52,
    ringSize: 48,
    ringColor: '#1E90FF',
    ringThickness: 4,
    ringRadius: 12,
    ringOffset: [10, 10] as const,
    ringAnimMs: 200,
  },
  colorSelector: {
    panelGradient: ['#030303', '#464646'] as const,
    panelRadius: 10,
    panelPadding: 5,
    swatch: 40,
    swatchRadius: 5,
    grid: [5, 3] as const,
    selectedRing: '#9D1B01',
    selectedRingThickness: 2,
    eyedropperSize: 40,
    opacitySliderHeight: 116,
    opacityRange: [1, 10] as const,
    opacityDefault: 10,
  },
  startMenu: {
    railButtonMin: [175, 55] as const,
    railRadius: 8,
    railFont: 17,
    iconSize: 40,
    submenuPaneWidth: 120,
    externalToolTile: 64,
    externalToolFill: '#D8EBF7',
  },
  elementEditor: {
    windowSize: [372, 443] as const,
    tabHeaderSize: [103, 52] as const,
    closeButtonSize: [38, 46] as const,
  },
  thickness: { min: 1, max: 48, setterMinWidth: 280 },
  iconChip: {
    canvas: 52,
    chip: 44,
    chipRadius: 8,
    chipGradient: ['#E0E0E0', '#C0C0C0'] as const,
  },
  windows: {
    common: { closeButton: 32, titleFont: 15 },
  },
  desktopMode: {
    traceToolBar: [316, 53] as const,
    penChip: [233, 44] as const,
    eraserChip: [151, 53] as const,
    clearBoard: [170, 52] as const,
  },
  palettes: {
    quick12: [
      '#FF1100', '#FF8B00', '#FFC511', '#66D553',
      '#331EB5', '#336ED9', '#4FA0B7', '#FF1FCE',
      '#306D00', '#000000', '#8B7F6E', '#FFFFFE',
    ] as const,
    sixStandard: ['#FFFFFF', '#231F20', '#F2343E', '#FFDE17', '#29CC7B', '#00AEEF'] as const,
  },
} as const;

export function hexToRgb(hex: string): [number, number, number] {
  const v = hex.replace('#', '');
  return [
    parseInt(v.slice(0, 2), 16),
    parseInt(v.slice(2, 4), 16),
    parseInt(v.slice(4, 6), 16),
  ];
}

export function rgbToHex(rgb: [number, number, number]): string {
  return '#' + rgb.map((c) => c.toString(16).padStart(2, '0')).join('').toUpperCase();
}
