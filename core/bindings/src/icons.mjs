const strokeStyle = [
    'fill:none',
    'stroke-width:2',
    'stroke-linecap:round',
    'stroke-linejoin:round',
    'stroke:currentColor',
    'stroke-opacity:1',
    'stroke-miterlimit:4',
].join(';');

export const openEye = `
<svg xmlns="http://www.w3.org/2000/svg" width="22" height="16" viewBox="0 0 24 24">
    <path style="${strokeStyle}"
          d="M 21.998437 12 C 21.998437 12 18.998437 18 12 18
             C 5.001562 18 2.001562 12 2.001562 12
             C 2.001562 12 5.001562 6 12 6
             C 18.998437 6 21.998437 12 21.998437 12 Z" />
    <path style="${strokeStyle}"
          d="M 15 12
             C 15 13.654687 13.654687 15 12 15
             C 10.345312 15 9 13.654687 9 12
             C 9 10.345312 10.345312 9 12 9
             C 13.654687 9 15 10.345312 15 12 Z" />
</svg>
`;

export const closedEye = `
<svg xmlns="http://www.w3.org/2000/svg" width="22" height="16" viewBox="0 0 24 24">
    <path style="${strokeStyle}"
          d="M 3 3 L 21 21" />
    <path style="${strokeStyle}"
          d="M 21.998437 12
             C 21.998437 12 18.998437 18 12 18
             C 5.001562 18 2.001562 12 2.001562 12
             C 2.001562 12 5.001562 6 12 6
             C 14.211 6 16.106 6.897 17.7 8.1" />
    <path style="${strokeStyle}"
          d="M 9.9 9.9
             C 9.367 10.434 9 11.178 9 12
             C 9 13.654687 10.345312 15 12 15
             C 12.822 15 13.566 14.633 14.1 14.1" />
</svg>
`;

/** Build an SVG element from its markup. */
export function createSvgIcon(svgContent) {
    const container = document.createElement('div');
    container.innerHTML = svgContent.trim();

    return container.querySelector('svg');
}
